use alloc::{string::String, vec::Vec};
use core::fmt::Write;
use stylus_sdk::alloy_primitives::FixedBytes;

use crate::base64::base64_encode;

const SVG_WIDTH: i32 = 1000;
const SVG_HEIGHT: i32 = 1000;
const BACKGROUND_COLOR: &str = "#1a1a1a";

const MIN_OSCILLATIONS: usize = 4;
const MAX_OSCILLATIONS: usize = 15;

const MIN_STROKE_WIDTH: usize = 10;
const MAX_STROKE_WIDTH: usize = 80;

const MIN_PERIOD: usize = 20;
const MAX_PERIOD: usize = 100;

const MIN_AMPLITUDE: usize = 100;
const MAX_AMPLITUDE: usize = 600;

pub struct SquiggleGenerator {
    seed: FixedBytes<32>,
}

struct SquiggleParameters {
    x_offsets: Vec<i32>,
    y_coordinates: Vec<i32>,
    stroke_width: i32,
    gradient_type: u8,
}
