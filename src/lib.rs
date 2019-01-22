//!
//! This crate is currently based on the OpenCV implementation found
//! [here](https://github.com/opencv/opencv/blob/50bec53afcf010e425b3f015c71297d46ef78903/modules/features2d/src/orb.cpp).
//!

pub enum ScoreType {
    HarrisScore,
    FastScore,
}

pub struct ORB {
    nfeatures: usize,
    scale_factor: f64,
    nlevels: usize,
    edge_threshold: i32,
    first_level: i32,
    wta_k: i32,
    score_type: ScoreType,
    patch_size: i32,
    fast_threshold: i32,
}
