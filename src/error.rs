use std::error;

pub type Result<T> = std::result::Result<T, Box<dyn error::Error>>;
