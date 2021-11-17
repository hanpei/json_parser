mod parser;
mod value;
mod tokenizer;
mod error;
mod macros;
mod generator;

use error::JsonError;

pub use parser::parse;
pub type JsonResult<T> = Result<T, JsonError>;