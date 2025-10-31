use std::error::Error;

mod to_boxed_error;

pub type DynError = Box<dyn Error + Send + 'static>;
pub use to_boxed_error::ToBoxedError;
