﻿use crate::abst::*;

pub struct Train {
}

impl TTrain for Train {
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn it_works2() {
        assert_ne!(2 + 2, 3);
    }
}
