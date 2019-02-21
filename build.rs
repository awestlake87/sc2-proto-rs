extern crate protoc_rust;

fn main() {
    if let Err(e) = protoc_rust::run(protoc_rust::Args {
        out_dir: "src/",
        includes: &["s2client-proto/"],
        input: &[
            "s2client-proto/s2clientprotocol/common.proto",
            "s2client-proto/s2clientprotocol/data.proto",
            "s2client-proto/s2clientprotocol/debug.proto",
            "s2client-proto/s2clientprotocol/error.proto",
            "s2client-proto/s2clientprotocol/query.proto",
            "s2client-proto/s2clientprotocol/raw.proto",
            "s2client-proto/s2clientprotocol/sc2api.proto",
            "s2client-proto/s2clientprotocol/score.proto",
            "s2client-proto/s2clientprotocol/spatial.proto",
            "s2client-proto/s2clientprotocol/ui.proto",
        ],
        customize: protoc_rust::Customize {
            ..Default::default()
        },
    }) {
        panic!("{:#?}", e);
    } else {
        println!("protobufs were generated successfully");
    }
}
