use std::{iter::Peekable, str::Chars};

use crate::{error::JsonError, JsonResult};

#[derive(Debug, PartialEq)]
pub enum Token {
    Comma,          // ,
    Colon,          // :
    BracketOn,      // [
    BracketOff,     // ]
    BraceOn,        // {
    BraceOff,       // }
    String(String), // "string"
    Number(f64),    // 123
    Boolen(bool),   // "true/false"
    Null,           // "null"
}

#[derive(Debug)]
pub struct Tokenizer<'a> {
    source: Peekable<Chars<'a>>,
}

impl<'a> Tokenizer<'a> {
    pub fn next(&mut self) -> JsonResult<Token> {
        while let Some(ch) = self.source.next() {
            return Ok(match ch {
                ',' => Token::Comma,
                ':' => Token::Colon,
                '[' => Token::BracketOn,
                ']' => Token::BracketOff,
                '{' => Token::BraceOn,
                '}' => Token::BraceOff,
                '0'..='9' | '-' => Token::Number(self.read_number(ch)?),
                '"' => Token::String(self.read_string(ch)),
                't' | 'f' | 'n' => {
                    let symbol = self.read_symbol(ch);
                    match symbol.as_str() {
                        "true" => Token::Boolen(true),
                        "false" => Token::Boolen(false),
                        "null" => Token::Null,
                        _ => return Err(JsonError::UnexpectedToken(symbol)),
                    }
                }

                _ => {
                    if ch.is_whitespace() {
                        continue;
                    } else {
                        return Err(JsonError::UnexpectedCharacter(ch));
                    }
                }
            });
        }
        Err(JsonError::UnexpectedEndOfJson)
    }
}

impl<'a> Tokenizer<'a> {
    pub fn new(source: &'a str) -> Self {
        Tokenizer {
            source: source.chars().peekable(),
        }
    }

    fn read_number(&mut self, ch: char) -> JsonResult<f64> {
        let mut value = ch.to_string();
        let mut point = false;
        let mut has_e = false;
        while let Some(&n) = self.source.peek() {
            match n {
                '0'..='9' => {
                    value.push(n);
                    self.source.next();
                }
                'e' | 'E' => {
                    if !has_e {
                        has_e = true;
                        value.push(n);
                        self.source.next();

                        match self.source.peek().unwrap() {
                            '-' | '+' => {
                                value.push(self.source.next().unwrap());
                            }
                            _ => continue,
                        }
                    } else {
                        return Err(JsonError::InvalidNumber);
                    }
                }
                '.' => {
                    if !point {
                        point = true;
                        value.push(n);
                        self.source.next();
                    } else {
                        return Err(JsonError::InvalidNumber);
                    }
                }
                _ => break,
            }
        }
        match value.parse::<f64>() {
            Ok(v) => Ok(v),
            Err(_e) => Err(JsonError::InvalidNumber),
        }
    }

    fn read_string(&mut self, first: char) -> String {
        let mut ret = String::new();
        let mut escape = false;

        while let Some(ch) = self.source.next() {
            if ch == first && escape == false {
                return ret;
            }
            match ch {
                '\\' => {
                    if escape {
                        escape = false;
                        ret.push(ch);
                    } else {
                        escape = true;
                    }
                }
                _ => {
                    ret.push(ch);
                    escape = false;
                }
            }
        }

        return ret;
    }

    fn read_symbol(&mut self, ch: char) -> String {
        let mut symbol = String::from(ch).to_string();

        while let Some(&c) = self.source.peek() {
            match c {
                'a'..='z' => {
                    symbol.push(c);
                    self.source.next();
                }
                _ => break,
            }
        }
        symbol
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let a = "abcd";
        let mut t = Tokenizer::new(a);

        assert_eq!(t.source.peek(), Some(&'a'));
        t.source.next();
        assert_eq!(t.source.peek(), Some(&'b'));
        t.source.next();
        assert_eq!(t.source.peek(), Some(&'c'));
    }

    #[test]
    fn read_string() {
        let mut t = Tokenizer::new(r#" "abc def   "  "#);
        let ret = t.next().unwrap();
        assert_eq!(Token::String("abc def   ".to_string()), ret);
    }

    #[test]
    fn read_symbol() {
        let mut t = Tokenizer::new("true");
        let mut f = Tokenizer::new("false");
        let mut n = Tokenizer::new("null");
        assert_eq!(Token::Boolen(true), t.next().unwrap());
        assert_eq!(Token::Boolen(false), f.next().unwrap());
        assert_eq!(Token::Null, n.next().unwrap());
    }

    mod number {
        use super::super::*;

        #[test]
        fn read_number() {
            assert_eq!(Tokenizer::new("11").next().unwrap(), Token::Number(11.0));
            assert_eq!(
                Tokenizer::new("123.4").next().unwrap(),
                Token::Number(123.4)
            );
            assert_eq!(
                Tokenizer::new("-123.40").next().unwrap(),
                Token::Number(-123.4)
            );
        }

        #[test]
        fn read_number_err() {
            let mut a = Tokenizer::new("1.23.4");
            let mut b = Tokenizer::new("1.23e23e");
            let expected = JsonError::InvalidNumber;
            assert_eq!(expected, a.next().err().unwrap());
            assert_eq!(expected, b.next().err().unwrap());
        }

        #[test]
        fn exponent() {
            let mut a = Tokenizer::new("1.8123E2");
            let mut b = Tokenizer::new("-1.8123e2");
            let mut c = Tokenizer::new("1.8123e-2");
            let mut d = Tokenizer::new("-1.8123e-2");

            // while let Some(s) = a.source.next() {
            //     ret = a.read_number(s).unwrap();
            // }

            assert_eq!(a.next().unwrap(), Token::Number(181.23));
            assert_eq!(b.next().unwrap(), Token::Number(-181.23));
            assert_eq!(c.next().unwrap(), Token::Number(0.018123));
            assert_eq!(d.next().unwrap(), Token::Number(-0.018123));
        }
    }

    #[test]
    fn iter() {
        let s = r#",     :[]{} true false null 123 33a"#;
        let mut t = Tokenizer::new(s);
        assert_eq!(Token::Comma, t.next().unwrap());
        assert_eq!(Token::Colon, t.next().unwrap());
        assert_eq!(Token::BracketOn, t.next().unwrap());
        assert_eq!(Token::BracketOff, t.next().unwrap());
        assert_eq!(Token::BraceOn, t.next().unwrap());
        assert_eq!(Token::BraceOff, t.next().unwrap());
        assert_eq!(Token::Boolen(true), t.next().unwrap());
        assert_eq!(Token::Boolen(false), t.next().unwrap());
        assert_eq!(Token::Null, t.next().unwrap());
        assert_eq!(Token::Number(123.0), t.next().unwrap());
        assert_eq!(Token::Number(33.0), t.next().unwrap());
    }

    #[test]
    fn iter_error() {
        let s = r#"tru"#;
        let mut t = Tokenizer::new(s);
        let result = t.next().err().unwrap();
        let expected = JsonError::UnexpectedToken("tru".to_string());
        assert_eq!(expected, result);
    }
}
