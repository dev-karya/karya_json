//! Error types for JSON serialization and deserialization.
//!
//! This module defines the error types that can occur during JSON serialization
//! and deserialization operations. These types provide detailed information about
//! what went wrong during the operation.
//!
//! # Examples
//!
//! ```
//! use karya_json::types::{JsonParser, JsonValue};
//! use karya_json::types::error::DeserializeError;
//!
//! let result = JsonParser::new("invalid json".to_string()).parse();
//!
//! if let Err(error) = result {
//!     match error {
//!         DeserializeError::InvalidJson(msg) => println!("Invalid JSON: {}", msg),
//!         _ => println!("Other error occurred"),
//!     }
//! }
//! ```

/// Errors that can occur during JSON serialization.
///
/// This enum represents the various errors that can occur when converting
/// Rust data structures to JSON.
///
/// # Examples
///
/// ```
/// use karya_json::types::error::SerializeError;
///
/// fn handle_serialize_error(error: SerializeError) {
///     match error {
///         SerializeError::InvalidType(msg) => println!("Invalid type: {}", msg),
///         SerializeError::InvalidValue(msg) => println!("Invalid value: {}", msg),
///         SerializeError::InvalidStructure(msg) => println!("Invalid structure: {}", msg),
///     }
/// }
/// ```
#[derive(Debug)]
pub enum SerializeError {
    /// Indicates that a value has a type that cannot be serialized to JSON.
    ///
    /// The string contains a description of the type error.
    InvalidType(String),

    /// Indicates that a value cannot be represented in JSON.
    ///
    /// The string contains a description of the value error.
    InvalidValue(String),

    /// Indicates that the structure of the data is not valid for JSON serialization.
    ///
    /// The string contains a description of the structure error.
    InvalidStructure(String),
}

/// Errors that can occur during JSON deserialization.
///
/// This enum represents the various errors that can occur when parsing
/// JSON into Rust data structures.
///
/// # Examples
///
/// ```
/// use karya_json::types::JsonParser;
/// use karya_json::types::error::DeserializeError;
///
/// let result = JsonParser::new("invalid json".to_string()).parse();
///
/// if let Err(error) = result {
///     match error {
///         DeserializeError::InvalidJson(msg) => println!("Invalid JSON: {}", msg),
///         DeserializeError::MissingField(field) => println!("Missing field: {}", field),
///         DeserializeError::TypeMismatch(msg) => println!("Type mismatch: {}", msg),
///         DeserializeError::InvalidValue(msg) => println!("Invalid value: {}", msg),
///     }
/// }
/// ```
#[derive(Debug)]
pub enum DeserializeError {
    /// Indicates that the JSON string is not valid according to the JSON specification.
    ///
    /// The string contains a description of the syntax error.
    InvalidJson(String),

    /// Indicates that a required field is missing from a JSON object.
    ///
    /// The string contains the name of the missing field.
    MissingField(String),

    /// Indicates that a value has a different type than expected.
    ///
    /// The string contains a description of the type mismatch.
    TypeMismatch(String),

    /// Indicates that a value is not valid in its context.
    ///
    /// The string contains a description of the value error.
    InvalidValue(String),
}
