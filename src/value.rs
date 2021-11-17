use std::collections::BTreeMap;

#[derive(Debug, PartialEq)]
pub enum JsonValue {
    Null,
    Boolen(bool),
    String(String),
    Number(f64),
    Array(Vec<JsonValue>),
    Object(BTreeMap<String, JsonValue>),
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
