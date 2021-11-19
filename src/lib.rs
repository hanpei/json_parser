mod parser;
mod value;
mod tokenizer;
mod error;
mod macros;
mod generator;

use error::JsonError;

pub type JsonResult<T> = Result<T, JsonError>;
pub use parser::parse;
pub use generator::stringify;