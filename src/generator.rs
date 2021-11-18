use crate::{error::JsonError, value::JsonValue};
use std::collections::BTreeMap;

// r#"
//     {
//         "code": 200    ,
//         "success": true  ,
//         "payload": {
//             "features": [
//                 "awesfome   fasfaf  ",
//                 "easyAPI  ",
//                 "lowLearningCurve"
//             ]
//         }
//     }
// "#

pub fn stringify<T>(input: T) -> String
where
    T: Into<JsonValue>,
{
    let mut gen = Generator::new(true, 4);
    gen.write_json(&input.into());
    gen.value()
}

enum Tab {
    Right,
    Left,
    Stay,
}

pub struct Generator {
    code: String,
    minify: bool,
    dent: u8,
    spaces: u8,
}

impl Generator {
    pub fn new(minify: bool, spaces: u8) -> Self {
        Generator {
            code: String::new(),
            minify,
            dent: 0,
            spaces,
        }
    }

    fn value(self) -> String {
        self.code
    }

    fn write_json(&mut self, json: &JsonValue) {
        match json {
            JsonValue::Null => self.write("null"),
            JsonValue::Boolen(b) => match b {
                true => self.write("true"),
                false => self.write("false"),
            },
            JsonValue::String(s) => self.write(&format!("{:?}", s)),
            JsonValue::Number(n) => self.write(n.to_string().as_str()),
            JsonValue::Array(array) => self.write_array(array),
            JsonValue::Object(object) => self.write_object(object),
        }
    }

    fn write(&mut self, s: &str) {
        self.code.push_str(s);
    }

    // fn indent(&mut self) {
    //     self.dent += 1;
    // }

    // fn dedent(&mut self) {
    //     if self.dent > 0 {
    //         self.dent -= 1;
    //     }
    // }

    fn ln(&mut self) {
        self.code.push('\n')
    }

    fn new_line(&mut self, tab: Tab) {
        match tab {
            Tab::Stay => (),
            Tab::Left => {
                if self.dent > 0 {
                    self.dent -= 1
                }
            }
            Tab::Right => self.dent += 1,
        }
        if !self.minify {
            self.ln();
            for _ in 0..(self.dent * self.spaces) {
                self.write(" ");
            }
        }
    }

    // [1,2,3]
    // [
    //     1,
    //     2
    // ]
    fn write_array(&mut self, array: &Vec<JsonValue>) {
        let mut first = true;
        self.write("[");

        for item in array {
            if first {
                first = false;
                self.new_line(Tab::Right);
            } else {
                self.write(",");
                if !self.minify {
                    self.write(" ");
                };

                self.new_line(Tab::Stay);
            };
            self.write_json(item);
        }

        self.new_line(Tab::Left);
        self.write("]");
    }

    // {
    //     key: value,
    //     abc: {
    //              123
    //          },
    // }
    fn write_object(&mut self, object: &BTreeMap<String, JsonValue>) {
        let mut first = true;
        self.write("{");

        for (key, value) in object.iter() {
            if first {
                first = false;
                self.new_line(Tab::Right);
            } else {
                self.write(",");
                self.new_line(Tab::Stay);
            };
            self.write(&format!("{:?}", key));
            self.write(":");
            if !self.minify {
                self.write(" ");
            };
            self.write_json(value);
        }
        self.new_line(Tab::Left);
        self.write("}");
    }
}

#[cfg(test)]
mod tests {
    use std::os::unix::prelude::OsStringExt;

    use super::*;
    use crate::{array, object, parse};

    #[test]
    fn temp() {
        println!("{}", u8::MAX);
    }

    #[test]
    fn indent_spaces() {
        let mut gen = Generator::new(false, 4);
        gen.write("abcd");
        gen.new_line(Tab::Right);
        gen.write("1234");
        gen.new_line(Tab::Right);
        gen.write("xyz");

        gen.write("abcd");
        gen.new_line(Tab::Left);
        gen.write("1234");
        gen.new_line(Tab::Left);
        gen.write("xyz");
        gen.new_line(Tab::Left);
        gen.write("xyz");

        println!("{}", gen.code);
    }

    #[test]
    fn write_array() {
        let mut gen = Generator::new(false, 4);
        let str = r#"[ 1, 2, 3, "a", [ "b", "c" ] ]"#;
        let json = parse(str).unwrap();
        gen.write_json(&json);
        let ret = gen.value();
        println!("stringify\n {}", ret);
    }

    #[test]
    fn write_object() {
        let mut gen = Generator::new(false, 4);
        let str = r#"{
    "a": "abc",
    "b": 123,
    "more": {
        "phone": null
    }
}"#;

        let json = object! {
            "a"=>"abc",
            "b"=>123,
            "more"=> object! {
                "phone"=>JsonValue::Null
            }
        };
        gen.write_json(&json);
        let ret = gen.value();
        println!("json\n {:?}", json);
        println!("stringify\n {}", ret);
        println!("str\n {}", str);
        assert_eq!(ret, str);
    }

    #[test]
    fn test_stringify() {
        let json = object! {
            "code"=>200,
            "success"=>true,
            "payload"=>object!{
                "features"=>array!["awesfome   fasfaf  ","easyAPI  ","lowLearningCurve"]
            }
        };
        let s = r#"{"code":200,"payload":{"features":["awesfome   fasfaf  ","easyAPI  ","lowLearningCurve"]},"success":true}"#;
        
        let ret = stringify(json);
        println!("stringify {}", ret);
        assert_eq!(ret, s);
    }
}
