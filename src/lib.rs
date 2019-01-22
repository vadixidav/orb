//!
//! This crate is currently based on the OpenCV implementation found
//! [here](https://github.com/opencv/opencv/blob/50bec53afcf010e425b3f015c71297d46ef78903/modules/features2d/src/orb.cpp).
//!

pub struct ORB {
    /// This determines how many features to sample.
    features: usize,
    /// This determines the magnitude of size change at each detection level.
    scale_factor: f32,
    /// This determines the total number of detection levels.
    levels: usize,
    /// This is how far from the edge features can be detected at.
    edge_threshold: usize,
    /// This is the level detection is started at.
    first_level: usize,
    /// This is the number of pre-calculated points to sample to generate the binary descriptor for a feature.
    sample_points: usize,
    /// The size of the patch from which to generate the BRIEF descriptor.
    patch_size: i32,
    /// The threshold for how much brighter pixels need to be in a contiguous arc for FAST to detect a feature.
    fast_threshold: i32,
}
