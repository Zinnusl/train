use crate::abst::*;

#[derive(Debug, PartialEq)]
pub enum ParseError {
    UnexpectedToken(String),
    ExpectedToken(String),
    UnexpectedEOF,
}

#[derive(Debug, PartialEq)]
pub enum Token {
    Ident(String),
    Number(f64),
    EOF,        
}

pub trait TTextParser {
    fn parse(&mut self) -> Result<Token, ParseError>;
}
