//! Protocol buffers for the StarCraft II Client API
//! Automatically generated from the protobuf definitions at
//! [https://github.com/Blizzard/s2client-proto](https://github.com/Blizzard/s2client-proto).

// All magic is done in the build.rs, this just includes the results
#[cfg(feature = "protoc-rust")]
include!(concat!(env!("OUT_DIR"), "/lib.rs"));

macro_rules! cfg_no_protoc {
    ($($item:item)+) => {
        $(
            #[cfg(not(feature = "protoc-rust"))]
            $item
        )*
    };
}
cfg_no_protoc!(
    pub mod common;
    pub mod data;
    pub mod debug;
    pub mod error;
    pub mod query;
    pub mod raw;
    pub mod sc2api;
    pub mod score;
    pub mod spatial;
    pub mod ui;
);
