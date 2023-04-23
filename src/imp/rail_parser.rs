use crate::abst::*;

#[derive(Debug, Default)]
pub struct RailParser {
}

impl TRailParser for RailParser {
    fn parse(&mut self, stream: impl std::io::BufRead) -> Result<Vec<rail_parser::Token>, rail_parser::ParseError> {
        let mut tokens = vec![];
        for line in stream.lines() {
            for c in line.unwrap().chars() {
                match c {
                    '◀' | '▶' | '▲' | '▼' => tokens.push(rail_parser::Token::Locomotive),
                    '─' => {
                        let increment_right_rail = match tokens.last_mut() {
                            Some(rail_parser::Token::Rail(dir, _)) if *dir == RailDir::Horizontal => true,
                            _ => false,
                        };
                        if increment_right_rail {
                            if let Some(rail_parser::Token::Rail(_, length)) = tokens.last_mut() {
                                *length += 1;
                            }
                        }
                        else {
                            tokens.push(rail_parser::Token::Rail(RailDir::Horizontal, 1));
                        }
                    },
                    '│' => { 
                        let increment_right_rail = match tokens.last_mut() {
                            Some(rail_parser::Token::Rail(dir, _)) if *dir == RailDir::Vertical => true,
                            _ => false,
                        };
                        if increment_right_rail {
                            if let Some(rail_parser::Token::Rail(_, length)) = tokens.last_mut() {
                                *length += 1;
                            }
                        }
                        else {
                            tokens.push(rail_parser::Token::Rail(RailDir::Vertical, 1));
                        }
                    },
                    '∑' => tokens.push(rail_parser::Token::EndOfRail),
                    ' ' | '\t' => {},
                    _ => return Err(rail_parser::ParseError::InvalidCharacter(c)),
                }
            }
        }
        Ok(tokens)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_right() {
        let test_scenario = "▶─────────────────────────────────────∑";
        // let test_scenario = "∑─────────────────────────────────────◀";
        let mut rail_parser = RailParser::default();
        let tokens = rail_parser.parse(test_scenario.as_bytes()).unwrap();
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
        assert_eq!(tokens.len(), 6);
        assert_eq!(tokens[0], rail_parser::Token::Locomotive);
        assert_eq!(tokens[1], rail_parser::Token::Rail(RailDir::Horizontal, 37));
        assert_eq!(tokens[2], rail_parser::Token::EndOfRail);        
        assert_eq!(tokens[3], rail_parser::Token::Locomotive);
        assert_eq!(tokens[4], rail_parser::Token::Rail(RailDir::Horizontal, 37));
        assert_eq!(tokens[5], rail_parser::Token::EndOfRail);        

        // let file = std::fs::File::open("testdata/RailParser/it_works.train").unwrap();
    }
}

