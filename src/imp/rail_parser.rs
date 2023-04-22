use crate::abst::*;

#[derive(Debug, Default)]
pub struct RailParser {
    tokens: Vec<rail_parser::Token>,
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
                            Some(rail_parser::Token::Rail(dir, _)) if *dir == RailDir::Right => true,
                            _ => false,
                        };
                        if increment_right_rail {
                            if let Some(rail_parser::Token::Rail(_, length)) = tokens.last_mut() {
                                *length += 1;
                            }
                        } else {
                            tokens.push(rail_parser::Token::Rail(RailDir::Right, 1));
                        }
                    }
                    '│' => { /* Similar logic for left rails */ }
                    '∑' => tokens.push(rail_parser::Token::EndOfRail),
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
        assert_eq!(tokens[1], rail_parser::Token::Rail(RailDir::Right, 37));
        assert_eq!(tokens[2], rail_parser::Token::EndOfRail);        

        // let file = std::fs::File::open("testdata/RailParser/it_works.train").unwrap();
    }
}

