use crate::abst::*;

#[derive(Debug, PartialEq)]
pub enum RailDir {
    Up,
    Down,
    Left,
    Right,
}

pub type RailAmount = u64;

pub trait TRail {
}
