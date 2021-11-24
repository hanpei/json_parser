use std::{collections::BTreeMap, fmt::Display};

use crate::generator::Generator;


#[derive(Debug, PartialEq)]
pub enum JsonValue {
    Null,
    Boolen(bool),
    String(String),
    Number(f64),
    Array(Vec<JsonValue>),
    Object(BTreeMap<String, JsonValue>),
}

impl JsonValue {
    pub fn dump(&self) -> String {
        let mut gen = Generator::new(true, 0);
        gen.write_json(self);
        gen.value()
    }
}

impl Display for JsonValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            JsonValue::String(ref value)  => value.fmt(f),
            JsonValue::Number(ref value)  => value.fmt(f),
            JsonValue::Boolen(ref value) => value.fmt(f),
            JsonValue::Null               => f.write_str("null"),
            _                             => f.write_str(&self.dump())
        }
    }
}

impl From<Vec<JsonValue>> for JsonValue {
    fn from(v: Vec<JsonValue>) -> Self {
        JsonValue::Array(v)
    }
}

impl From<BTreeMap<String, JsonValue>> for JsonValue {
    fn from(val: BTreeMap<String, JsonValue>) -> JsonValue {
        JsonValue::Object(val)
    }
}

impl<'a> From<&'a str> for JsonValue {
    fn from(s: &'a str) -> Self {
        JsonValue::String(s.to_string())
    }
}

macro_rules! impl_from_num_for_json {
    ($($t: ident), *) => {
      $(
        impl From<$t> for JsonValue {
            fn from(value: $t) -> JsonValue {
                JsonValue::Number(value as f64)
            }
        }
      )*
    };
  }

macro_rules! implement {
    ($from:ty, $to:ident) => {
        impl From<$from> for JsonValue {
            fn from(value: $from) -> JsonValue {
                JsonValue::$to(value)
            }
        }
    };
}

impl_from_num_for_json!(i8, i16, i32, i64, isize);
implement!(bool, Boolen);
implement!(String, String);
