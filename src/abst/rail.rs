use crate::abst::*;

#[derive(Debug, PartialEq, Clone)]
pub enum RailDir {
    Horizontal,
    Vertical,
}

pub type RailAmount = u64;

pub trait TRail {
}
