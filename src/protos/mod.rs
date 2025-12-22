
pub mod refbox {
    include!(concat!(env!("OUT_DIR"), "/pbgen_refbox/mod.rs"));
}

pub mod vision {
    include!(concat!(env!("OUT_DIR"), "/pbgen_vision/mod.rs"));
}
