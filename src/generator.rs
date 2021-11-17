use std::collections::BTreeMap;

use crate::value::JsonValue;

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
            JsonValue::String(s) => self.write(s),
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
                self.new_line(Tab::Stay);
            }
            self.write_json(item);
        }

        self.new_line(Tab::Left);
        self.write("]");
    }

    fn write_object(&self, object: &BTreeMap<String, JsonValue>) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse;

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
    fn gen_array() {
        let mut gen = Generator::new(false, 4);
        let str = r#"[ 1, 2, 3, "a", [ "b", "c" ] ]"#;
        let json = parse(str).unwrap();
        gen.write_json(&json);
        let ret = gen.value();
        println!("stringify\n {}", ret);
    }
}
