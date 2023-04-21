//! Numbers of rounds allowed to be used with the ChaCha stream cipher

/// Marker type for a number of ChaCha rounds to perform.
pub trait Rounds: Copy {
    const COUNT: usize;
}

/// 8-rounds
#[derive(Copy, Clone)]
pub struct R8;

impl Rounds for R8 {
    const COUNT: usize = 8;
}

/// 12-rounds
#[derive(Copy, Clone)]
pub struct R12;

impl Rounds for R12 {
    const COUNT: usize = 12;
}

/// 20-rounds
#[derive(Copy, Clone)]
pub struct R20;

impl Rounds for R20 {
    const COUNT: usize = 20;
}
