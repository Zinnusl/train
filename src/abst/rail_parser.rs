use crate::abst::*;

#[derive(Debug, PartialEq)]
pub enum ParseError {
    UnexpectedToken(Token),
    InvalidCharacter(char),
}

#[derive(Debug, PartialEq)]
pub enum Token {
    Locomotive,
    Rail(RailDir, RailAmount),
    EndOfRail,
}

pub trait TRailParser {
    fn parse(&mut self, stream: impl std::io::BufRead) -> Result<Vec<Token>, ParseError>;
}
