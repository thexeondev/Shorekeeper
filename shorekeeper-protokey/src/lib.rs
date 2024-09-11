use std::array::TryFromSliceError;

use aes::cipher::block_padding::{Pkcs7, UnpadError};
use aes::cipher::{BlockDecrypt, BlockEncrypt, InvalidLength, KeyInit};
use aes::Aes256;
use rsa::pkcs1::{DecodeRsaPrivateKey, DecodeRsaPublicKey};
use rsa::rand_core::RngCore;

mod config;
pub use config::ProtoKeySettings;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("RSA Error: {0}")]
    RSA(#[from] rsa::errors::Error),
    #[error("Failed to parse RSA key from PEM: {0}")]
    ParsePemFailed(#[from] rsa::pkcs1::Error),
    #[error("PKCS7 UnPad Error: {0}")]
    UnPad(String),
    #[error("AES key invalid length Error: {0}")]
    AesKeyInvalidLen(#[from] InvalidLength),
    #[error("TryFromSliceError: {0}")]
    TryFromSlice(#[from] TryFromSliceError),
    #[error("a request to process a session key was received while engine is disabled")]
    ProcessSessionKeyWhileDisabled,
}

impl From<UnpadError> for Error {
    fn from(err: UnpadError) -> Self {
        Error::UnPad(err.to_string())
    }
}

pub type ServerProtoKeyHelper = ProtoKeyHelper<rsa::RsaPublicKey>;
pub type ClientProtoKeyHelper = ProtoKeyHelper<rsa::RsaPrivateKey>;

pub struct ProtoKeyHelper<T> {
    enabled: bool,
    settings: &'static ProtoKeySettings,
    // TODO: Instead of supporting either operation, shall we support both as option?
    //  Use case, Man In The Middle / proxy
    //  Or is it supported with ProtoKeyHelper<rsa::RsaPrivateKey + rsa::RsaPublicKey> ??
    client_key: T,
}

impl ProtoKeyHelper<rsa::RsaPublicKey> {
    pub fn with_public_key(settings: &'static ProtoKeySettings, pem: &str) -> Result<Self, Error> {
        Ok(Self {
            enabled: true,
            settings,
            client_key: rsa::RsaPublicKey::from_pkcs1_pem(pem)?,
        })
    }

    pub fn generate_session_key(&self) -> Result<([u8; 32], Option<Vec<u8>>), Error> {
        if !self.enabled {
            return Err(Error::ProcessSessionKeyWhileDisabled);
        }
        let mut session_key: [u8; 32] = [0; 32];
        if !self.settings.use_client_key {
            return Ok((session_key, None));
        }
        let mut rng = rand::thread_rng();
        rng.fill_bytes(&mut session_key);
        let key = self
            .client_key
            .encrypt(&mut rng, rsa::Pkcs1v15Encrypt, &session_key[..])?;
        Ok((session_key, Some(key)))
    }
}

impl ProtoKeyHelper<rsa::RsaPrivateKey> {
    pub fn with_private_key(settings: &'static ProtoKeySettings, pem: &str) -> Result<Self, Error> {
        Ok(Self {
            enabled: true,
            settings,
            client_key: rsa::RsaPrivateKey::from_pkcs1_pem(pem)?,
        })
    }

    pub fn unwrap_session_key(&self, wrapped_key: Vec<u8>) -> Result<[u8; 32], Error> {
        if !self.enabled {
            return Err(Error::ProcessSessionKeyWhileDisabled);
        }
        if !self.settings.use_client_key {
            return Ok([0; 32]);
        }
        let key = self
            .client_key
            .decrypt(rsa::Pkcs1v15Encrypt, &wrapped_key[..])?;
        Ok(key.as_slice().try_into()?)
    }
}

impl<T> ProtoKeyHelper<T> {
    fn is_encryption_enabled_for_message(&self, msg_id: u16) -> bool {
        self.enabled && !self.settings.builtin_encryption_msg_id.contains(&msg_id)
    }

    #[tracing::instrument(skip(self))]
    pub fn decrypt(
        &self,
        msg_id: u16,
        sequence_number: u32,
        session_key: &[u8; 32],
        data: Box<[u8]>,
    ) -> Result<Box<[u8]>, Error> {
        if !self.is_encryption_enabled_for_message(msg_id) || data.is_empty() {
            return Ok(data);
        }
        tracing::trace!("Data before decryption {:02X?}", data);
        let mut decrypted = decrypt_aes256_ecb_pkcs7(session_key, &data)?;
        tracing::trace!("Data after decryption {:02X?}", decrypted);
        kuro_magic(
            self.settings.use_client_key,
            sequence_number,
            session_key,
            &mut decrypted,
        );
        tracing::trace!(
            "Data after xor with seqNo({}): {:02X?}",
            sequence_number,
            decrypted
        );
        return Ok(decrypted);
    }

    #[tracing::instrument(skip(self))]
    pub fn encrypt(
        &self,
        msg_id: u16,
        sequence_number: u32,
        session_key: &[u8; 32],
        mut data: Box<[u8]>,
    ) -> Result<Box<[u8]>, Error> {
        if !self.is_encryption_enabled_for_message(msg_id) || data.is_empty() {
            return Ok(data);
        }
        tracing::trace!(
            "Data before xor with seqNo({}): {:02X?}",
            sequence_number,
            data
        );

        kuro_magic(
            self.settings.use_client_key,
            sequence_number,
            session_key,
            &mut data,
        );
        tracing::trace!("Data before encryption {:02X?}", data);
        let encrypted = encrypt_aes256_ecb_pkcs7(session_key, &data)?;
        tracing::trace!("Data after encryption {:02X?}", encrypted);
        return Ok(encrypted);
    }
}

fn kuro_magic(use_client_key: bool, sequence_number: u32, session_key: &[u8; 32], data: &mut [u8]) {
    if !use_client_key {
        let mut index = (sequence_number + 0xd) & 0x8000001f; // TODO: Verify this again in Ghidra / IDA
        if (index as i32) < 0 {
            index = ((index - 1) | 0xffffffe0) + 1;
        }
        data.iter_mut()
            .for_each(|element| *element ^= session_key[index as usize]);
    } else {
        let mut index = sequence_number & 0x8000001f;
        if (index as i32) < 0 {
            index = ((index - 1) | 0xffffffe0) + 1;
        }
        let length = data.len();
        data[sequence_number as usize % length] ^= session_key[index as usize]
    }
}

#[inline]
fn decrypt_aes256_ecb_pkcs7(session_key: &[u8; 32], data: &[u8]) -> Result<Box<[u8]>, Error> {
    let cipher = Aes256::new_from_slice(session_key)?;
    let result = cipher.decrypt_padded_vec::<Pkcs7>(data)?;
    Ok(result.into_boxed_slice())
}

#[inline]
fn encrypt_aes256_ecb_pkcs7(
    session_key: &[u8; 32],
    data: &[u8],
) -> Result<Box<[u8]>, InvalidLength> {
    let cipher = Aes256::new_from_slice(session_key)?;
    Ok(cipher
        .encrypt_padded_vec::<Pkcs7>(&data[..])
        .into_boxed_slice())
}
