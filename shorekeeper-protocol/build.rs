use std::{
    fs,
    io::{self, BufRead},
    path::Path,
};

use quote::quote;

const CODEGEN_OUT_DIR: &str = "generated/";

pub fn main() {
    let _ = fs::create_dir(CODEGEN_OUT_DIR);

    let config_file = "proto/config.csv";
    let config_path = Path::new("proto/config.csv");
    if config_path.exists() {
        println!("cargo:rerun-if-changed={config_file}");
        impl_proto_config(config_path, Path::new("generated/proto_config.rs")).unwrap();
    }

    let proto_file = "proto/shorekeeper.proto";

    if Path::new(&proto_file).exists() {
        println!("cargo:rerun-if-changed={proto_file}");

        prost_build::Config::new()
            .out_dir(CODEGEN_OUT_DIR)
            .type_attribute(".", "#[derive(shorekeeper_protocol_derive::MessageID)]")
            .compile_protos(&[proto_file], &["shorekeeper"])
            .unwrap();

        impl_message_id(Path::new("generated/shorekeeper.rs")).unwrap();
    }

    let proto_file = "proto/internal.proto";
    if Path::new(&proto_file).exists() {
        println!("cargo:rerun-if-changed={proto_file}");

        prost_build::Config::new()
            .out_dir(CODEGEN_OUT_DIR)
            .type_attribute(".", "#[derive(shorekeeper_protocol_derive::MessageID)]")
            .compile_protos(&[proto_file], &["internal"])
            .unwrap();

        impl_message_id(Path::new("generated/internal.rs")).unwrap();
    }

    let proto_file = "proto/data.proto";
    if Path::new(&proto_file).exists() {
        println!("cargo:rerun-if-changed={proto_file}");

        prost_build::Config::new()
            .out_dir(CODEGEN_OUT_DIR)
            .compile_protos(&[proto_file], &["data"])
            .unwrap();
    }
}

pub fn impl_proto_config(csv_path: &Path, codegen_path: &Path) -> io::Result<()> {
    let file = fs::File::open(csv_path)?;
    let reader = io::BufReader::new(file);

    let mut match_arms = quote! {};
    for line in reader.lines() {
        let line = line?;
        let mut row = line.split(',');

        let message_id = row.next().unwrap().parse::<u16>().unwrap();
        let flags = row.next().unwrap().parse::<u8>().unwrap();

        match_arms = quote! {
            #match_arms
            #message_id => MessageFlags(#flags),
        }
    }

    let generated_code = quote! {
        pub mod proto_config {
            #[derive(Clone, Copy)]
            pub struct MessageFlags(u8);

            impl MessageFlags {
                pub fn value(self) -> u8 {
                    self.0
                }
            }

            pub fn get_message_flags(id: u16) -> MessageFlags {
                match id {
                    #match_arms
                    _ => MessageFlags(0),
                }
            }
        }
    }
    .to_string();

    fs::write(codegen_path, generated_code.as_bytes())?;
    Ok(())
}

pub fn impl_message_id(path: &Path) -> io::Result<()> {
    let file = fs::File::open(path)?;
    let reader = io::BufReader::new(file);
    let mut output = Vec::new();

    let mut attr = None;
    for line in reader.lines() {
        let line = line?;

        if line.contains("MessageId:") {
            attr = Some(make_message_id_attr(&line).unwrap());
        } else {
            output.push(line);
            if let Some(attr) = attr.take() {
                output.push(attr);
            }
        }
    }

    fs::write(path, output.join("\n").as_bytes())?;
    Ok(())
}

fn make_message_id_attr(line: &str) -> Option<String> {
    let id = line.trim_start().split(' ').nth(2)?.parse::<u16>().ok()?;
    Some(format!("#[message_id({id})]"))
}
