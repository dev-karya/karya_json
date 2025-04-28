#[derive(Debug)]
pub enum SerializeError {
    InvalidType(String),
    InvalidValue(String),
    InvalidStructure(String),
}

#[derive(Debug)]
pub enum DeserializeError {
    InvalidJson(String),
    MissingField(String),
    TypeMismatch(String),
    InvalidValue(String),
}
