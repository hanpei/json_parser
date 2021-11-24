use std::collections::BTreeMap;

use crate::{
    error::JsonError,
    tokenizer::{Token, Tokenizer},
    value::JsonValue,
    JsonResult,
};

struct Parser<'a> {
    tokenizer: Tokenizer<'a>,
}

// parse_value: str -> match Token -> JsonValue
impl<'a> Parser<'a> {
    pub fn new(source: &'a str) -> Self {
        Parser {
            tokenizer: Tokenizer::new(source),
        }
    }

    // str -> Token
    fn consume(&mut self) -> JsonResult<Token> {
        self.tokenizer.next()
    }

    fn parse_value(&mut self, token: Token) -> JsonResult<JsonValue> {
        Ok(match token {
            Token::Null => JsonValue::Null,
            Token::Boolen(b) => JsonValue::Boolen(b),
            Token::Number(n) => JsonValue::Number(n),
            Token::String(s) => JsonValue::String(s),
            Token::BraceOn => self.parse_object()?,
            Token::BracketOn => self.parse_array()?,
            _ => return Err(JsonError::unexpected_token(token)),
        })
    }

    // return json value
    fn value(&mut self) -> JsonResult<JsonValue> {
        let token = self.consume()?;
        self.parse_value(token)
    }

    fn parse_object(&mut self) -> JsonResult<JsonValue> {
        let mut ret = BTreeMap::new();

        match self.consume()? {
            Token::BraceOff => return Ok(ret.into()),
            Token::String(key) => {
                match self.consume()? {
                    Token::Colon => (),
                    token => return Err(JsonError::unexpected_token(token)),
                }
                let value = self.value()?;
                ret.insert(key, value);
            }
            token => return Err(JsonError::unexpected_token(token)),
        }

        loop {
            match self.consume()? {
                Token::Comma => {
                    let key = match self.consume()? {
                        Token::String(key) => key,
                        token => return Err(JsonError::unexpected_token(token)),
                    };
                    match self.consume()? {
                        Token::Colon => (),
                        token => return Err(JsonError::unexpected_token(token)),
                    }
                    let value = self.value()?;
                    ret.insert(key, value);
                }

                Token::BraceOff => break,
                token => return Err(JsonError::unexpected_token(token)),
            }
        }

        Ok(ret.into())
    }

    /*
     * [1,2,3]
     * [[a,b,c], d, e]
     */
    fn parse_array(&mut self) -> JsonResult<JsonValue> {
        let mut ret = Vec::new();
        match self.consume()? {
            Token::BracketOff => return Ok(ret.into()),
            token => ret.push(self.parse_value(token)?),
        }

        loop {
            match self.consume()? {
                Token::Comma => ret.push(self.value()?),
                Token::BracketOff => break,
                token => return Err(JsonError::unexpected_token(token)),
            }
        }

        Ok(ret.into())
    }
}

pub fn parse(json: &str) -> JsonResult<JsonValue> {
    let mut parser = Parser::new(json);
    Ok(parser.value()?)
}

#[cfg(test)]
mod tests {
    use crate::{array, object};

    use super::*;

    #[test]
    fn consume() {
        let s = r#",{[ null true false "abc  d " 1234 , :"#;
        let mut source = Parser::new(s);
        let token = source.consume().unwrap();
        assert_eq!(token, Token::Comma);
        let token = source.consume().unwrap();
        assert_eq!(token, Token::BraceOn);
        let token = source.consume().unwrap();
        assert_eq!(token, Token::BracketOn);
        let token = source.consume().unwrap();
        assert_eq!(token, Token::Null);
        let token = source.consume().unwrap();
        assert_eq!(token, Token::Boolen(true));
        let token = source.consume().unwrap();
        assert_eq!(token, Token::Boolen(false));
        let token = source.consume().unwrap();
        assert_eq!(token, Token::String("abc  d ".to_string()));
        let token = source.consume().unwrap();
        assert_eq!(token, Token::Number(1234.into()));
        let token = source.consume().unwrap();
        assert_eq!(token, Token::Comma);
        let token = source.consume().unwrap();
        assert_eq!(token, Token::Colon);
    }

    #[test]
    fn parse_value() {
        let s = r#"[1,2 , "a",3]"#;
        let mut source = Parser::new(s);
        // let ret = source.consume();
        let a = source.value();
        println!("{:?}", a);
        let a = source.value();
        println!("{:?}", a);
    }

    #[test]
    fn parse_array() {
        // let s = r#"[1,"foo",[3, 4]]"#;
        let s = r#"[1,2,3]"#;
        let mut source = Parser::new(s);
        let ret = source.value();
        println!("{:?}", ret.unwrap());

        // let a = array!["a", "b", "c", 1,  JsonValue::Null, true, false];
        // assert_eq!(a, parse(s).unwrap())
    }

    #[test]
    fn parse_object() {
        let s = r#"{"name": "abc", "age": 123}"#;
        let mut source = Parser::new(s);
        let ret = source.value().unwrap();
        println!("object parsed {:?}", ret);
        let o = object! {"name"=>"abc", "age"=>123};
        assert_eq!(o, ret);
        assert_eq!(o, parse(s).unwrap())
    }

    #[test]
    fn parse_object_with_array() {
        assert_eq!(
            parse(
                r#"
                    {
                        "foo": [1, 2, 3]
                    }
                    "#
            )
            .unwrap(),
            object! {
                "foo" => array![1, 2, 3]
            }
        );
    }

    #[test]
    fn parse_json() {
        let s = r#"
            {
                "code": 200    ,
                "success": true  ,
                "payload": {
                    "features": [
                        "awesfome   fasfaf  ",
                        "easyAPI  ",
                        "lowLearningCurve"
                    ]
                }
            }
        "#;
        let ret = parse(s).unwrap();
        let expect = object! {
            "code" => 200,
            "success" => true,
            "payload" => object! {
                "features" => array![
                    "awesfome   fasfaf  ",
                    "easyAPI  ",
                    "lowLearningCurve"
                ]
            }
        };
        println!("{:?}", ret);
        assert_eq!(expect, ret);
    }

    #[test]
    fn parse_string() {
        let s = "{\"name\":\"myname \\n\",\"password\":123456}";
        // let s = "{\"code\":1000,\"message\":\"\\u67e5\\u8be2\\u6210\\u529f\",\"data\":\"\\u5317\\u4eac\\u9996\\u90fd\"}";

        let ret = parse(s).unwrap();

        println!("{:?}", ret);
    }

    #[test]
    fn parse_unicode() {
        let s = "{\"code\":1000,\"message\":\"\\u67e5\\u8be2\\u6210\\u529f\",\"data\":\"\\u5317\\u4eac\\u9996\\u90fd\"}";
        let ret = parse(s).unwrap();

        println!("{:?}", ret);
    }
    
    #[test]
    fn parse_one_unicode() {
        let s = "\"\\u67e5 \\/12\"";
        let ret = parse(s).unwrap();

        println!("{:?}", ret);
    }
}
