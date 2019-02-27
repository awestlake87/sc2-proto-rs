extern crate protoc_rust;

use std::env;
use std::ffi::OsStr;
use std::fs;
use std::io::prelude::*;
use std::path::Path;

fn proto_modules(proto_dir: &Path) -> Vec<String> {
    fs::read_dir(proto_dir)
        .expect("Could not read protobuf directory")
        .filter_map(|entry| {
            let path = entry.ok()?.path();
            if path.is_file() && path.extension() == Some(OsStr::new("proto")) {
                path.file_stem()
                    .and_then(|n| n.to_os_string().into_string().ok())
            } else {
                None
            }
        })
        .collect()
}

fn main() {
    let in_dir = "./s2client-proto/s2clientprotocol";
    let out_dir = &env::var("OUT_DIR").unwrap();

    // Read list of all input protobuf files
    let input_mods = proto_modules(Path::new(in_dir));
    let input_files: Vec<String> = input_mods
        .iter()
        .map(|s| format!("{}/{}.proto", in_dir, s))
        .collect();

    // Compile protocol buffers
    if let Err(e) = protoc_rust::run(protoc_rust::Args {
        out_dir,
        includes: &["s2client-proto/"],
        input: input_files
            .iter()
            .map(|s| s.as_str())
            .collect::<Vec<_>>()
            .as_slice(),
        customize: protoc_rust::Customize {
            ..Default::default()
        },
    }) {
        panic!("{:#?}", e);
    } else {
        println!("protobufs were generated successfully");
    }

    // Generate the lib.rs source code
    let mut buffer = fs::File::create(format!("{}/{}", out_dir, "lib.rs")).unwrap();
    buffer
        .write(
            input_mods
                .iter()
                .map(|s| format!("pub mod {};", s))
                .collect::<Vec<_>>()
                .join("\n")
                .as_bytes(),
        )
        .unwrap();
}
