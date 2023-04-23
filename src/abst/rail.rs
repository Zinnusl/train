use crate::abst::*;

#[derive(Debug, PartialEq)]
pub enum RailDir {
    Horizontal,
    Vertical,
}

pub type RailAmount = u64;

pub trait TRail {
}
