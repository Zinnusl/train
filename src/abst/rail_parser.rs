use crate::abst::*;

#[derive(Debug, PartialEq)]
pub enum ParseError {
    UnexpectedToken(Token),
    InvalidCharacter(char),
    IoError,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Locomotive,
    Rail(RailDir, RailAmount),
    EndOfRail,
}

pub trait TRailParser {
    fn parse(&mut self, stream: impl std::io::BufRead) -> Result<Vec<Vec<Token>>, ParseError>;
}
