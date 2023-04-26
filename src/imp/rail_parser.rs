use crate::abst::*;

use rail_parser::{Token, ParseError};
use std::io::BufRead;

#[derive(Debug, Default)]
pub struct RailParser {
    map: Vec<Vec<char>>,
}

impl TRailParser for RailParser {
    fn parse(&mut self, stream: impl BufRead) -> Result<Vec<Vec<Token>>, ParseError> {
        let mut result = vec![];

        // Parse the input into the map
        for line in stream.lines() {
            let chars = line.map_err(|_| ParseError::IoError)?.chars().collect::<Vec<char>>();
            if !chars.is_empty() {
                self.map.push(chars);
            }
        }

        // Find the starting locomotive and its direction
        let (mut i, mut j, mut direction) = {
            let mut loc = (0,0, RailDir::Horizontal);
            let mut found = false;
            for x in 0..self.map.len() {
                for y in 0..self.map[x].len() {
                    match self.map[x][y] {
                        '▶' => { loc = (x, y, RailDir::Horizontal); found = true; break; },
                        '▲' => { loc = (x, y, RailDir::Vertical); found = true; break; },
                        '◀' => { loc = (x, y, RailDir::Horizontal); found = true; break; },
                        '▼' => { loc = (x, y, RailDir::Vertical); found = true; break; },
                        _ => {}
                    }
                }
                if found { break; }
            }
            loc
        };

        println!("Test 1");

        // Debug Print
        for x in 0..self.map.len() {
            for y in 0..self.map[x].len() {
                print!("{}", self.map[x][y]);
            }
            println!();
        }

        println!("Test 2");

        let mut tokens: Vec<Token> = vec![];
        loop {
            if i >= self.map.len() || j >= self.map[i].len() {
                break;
            }

            println!("i: {}, j: {}, char: {}", i, j, self.map[i][j]);
            match self.map[i][j] {
                '─' => {
                    if direction == RailDir::Horizontal {
                        if j + 1 < self.map[i].len() {
                            j += 1;
                        } else {
                            break;
                        }
                    } else {
                        break; // invalid horizontal rail in a vertical rail path
                    }
                }
                '│' => {
                    if direction == RailDir::Vertical {
                        if i + 1 < self.map.len() {
                            i += 1;
                        } else {
                            break;
                        }
                    } else {
                        break; // invalid vertical rail in a horizontal rail path
                    }
                }
                '◀' | '▼' | '▶' | '▲' => {
                    tokens.push(Token::Locomotive);
                    // update direction based on the locomotive's symbol
                    direction = match self.map[i][j] {
                        '▶' | '◀' => RailDir::Horizontal,
                        '▲' | '▼' => RailDir::Vertical,
                        _ => direction
                    };
                }
                '∑' => {
                    tokens.push(Token::EndOfRail);
                    break; // end of the rail
                }
                _ => {}
            }

            // Update i and j depending on the direction
            match direction {
                RailDir::Horizontal => j += 1,
                RailDir::Vertical => i += 1
            }
        }
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn simple_right() {
        let test_scenario = "▶─────────────────────────────────────∑";
        // let test_scenario = "∑─────────────────────────────────────◀";
        let mut rail_parser = RailParser::default();
        let tokens = rail_parser.parse(test_scenario.as_bytes()).unwrap();
        let tokens = &tokens[0];
        assert_eq!(tokens.len(), 3);
        assert_eq!(tokens[0], rail_parser::Token::Locomotive);
        assert_eq!(tokens[1], rail_parser::Token::Rail(RailDir::Horizontal, 37));
        assert_eq!(tokens[2], rail_parser::Token::EndOfRail);        

        // let file = std::fs::File::open("testdata/RailParser/it_works.train").unwrap();
    }

    #[test]
    fn simple_left() {
        let test_scenario = "∑─────────────────────────────────────◀";
        let mut rail_parser = RailParser::default();
        let tokens = rail_parser.parse(test_scenario.as_bytes()).unwrap();
        let tokens = &tokens[0];
        assert_eq!(tokens.len(), 3);
        assert_eq!(tokens[0], rail_parser::Token::EndOfRail);        
        assert_eq!(tokens[1], rail_parser::Token::Rail(RailDir::Horizontal, 37));
        assert_eq!(tokens[2], rail_parser::Token::Locomotive);
    }

    #[test]
    fn simple_up() {
        let test_scenario = "
            ∑
            │
            │
            │
            │
            │
            │
            │
            │
            │
            │
            │
            │
            │
            │
            │
            │
            │
            │
            │
            │
            │
            │
            │
            │
            │
            │
            │
            │
            │
            │
            ▲";
        let mut rail_parser = RailParser::default();
        let tokens = rail_parser.parse(test_scenario.as_bytes()).unwrap();
        let tokens = &tokens[0];
        assert_eq!(tokens.len(), 3);
        assert_eq!(tokens[0], rail_parser::Token::EndOfRail);        
        assert_eq!(tokens[1], rail_parser::Token::Rail(RailDir::Vertical, 30));
        assert_eq!(tokens[2], rail_parser::Token::Locomotive);
    }

    #[test]
    fn simple_down() {
        let test_scenario = "
            ▼
            │
            │
            │
            │
            │
            │
            │
            │
            │
            │
            │
            │
            │
            │
            │
            │
            │
            │
            │
            │
            │
            │
            │
            │
            │
            │
            │
            │
            │
            │
            ∑";
        let mut rail_parser = RailParser::default();
        let tokens = rail_parser.parse(test_scenario.as_bytes()).unwrap();
        let tokens = &tokens[0];
        assert_eq!(tokens.len(), 3);
        assert_eq!(tokens[0], rail_parser::Token::Locomotive);
        assert_eq!(tokens[1], rail_parser::Token::Rail(RailDir::Vertical, 30));
        assert_eq!(tokens[2], rail_parser::Token::EndOfRail);        
    }

    #[test]
    fn double_right() {
        let test_scenario = "
            ▶─────────────────────────────────────∑
            ▶─────────────────────────────────────∑
        ";
        // let test_scenario = "∑─────────────────────────────────────◀";
        let mut rail_parser = RailParser::default();
        let tokens = rail_parser.parse(test_scenario.as_bytes()).unwrap();
        assert_eq!(tokens.len(), 2);
        let rails_0 = &tokens[0];
        assert_eq!(rails_0.len(), 3);
        let rails_1 = &tokens[1];
        assert_eq!(rails_1.len(), 3);
        assert_eq!(rails_0[0], rail_parser::Token::Locomotive);
        assert_eq!(rails_0[1], rail_parser::Token::Rail(RailDir::Horizontal, 37));
        assert_eq!(rails_0[2], rail_parser::Token::EndOfRail);        
        assert_eq!(rails_1[0], rail_parser::Token::Locomotive);
        assert_eq!(rails_1[1], rail_parser::Token::Rail(RailDir::Horizontal, 37));
        assert_eq!(rails_1[2], rail_parser::Token::EndOfRail);        

        // let file = std::fs::File::open("testdata/RailParser/it_works.train").unwrap();
    }
}
