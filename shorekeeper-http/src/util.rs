use crate::config::AesSettings;
use aes::{
    cipher::{block_padding::Pkcs7, BlockEncryptMut, KeyIvInit},
    Aes256,
};
use serde::{Deserialize, Deserializer};

pub fn from_base64<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
where
    D: Deserializer<'de>,
{
    use serde::de::Error;
    String::deserialize(deserializer)
        .and_then(|string| rbase64::decode(&string).map_err(|err| Error::custom(err.to_string())))
}

pub fn encrypt_with_aes(data: &[u8], settings: &AesSettings) -> String {
    let encryptor = cbc::Encryptor::<Aes256>::new_from_slices(&settings.key, &settings.iv).unwrap();

    let data = encryptor.encrypt_padded_vec_mut::<Pkcs7>(data);
    rbase64::encode(&data)
}
