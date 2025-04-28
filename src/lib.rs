//! # Karya JSON
//!
//! A high-performance, focused JSON serialization and deserialization library for Rust.
//! Designed as a faster alternative to Serde when working specifically with JSON.
//!
//! ## Overview
//!
//! Karya JSON provides a simple yet powerful API for working with JSON data in Rust.
//! It focuses exclusively on JSON processing with these advantages:
//!
//! - **Speed-focused**: Built from the ground up for optimal JSON performance
//! - **Lightweight**: Smaller footprint with fewer dependencies
//! - **Simple API**: Streamlined interface designed specifically for JSON
//! - **Lower compile times**: Reduced macro complexity means faster builds
//!
//! ## Quick Start
//!
//! ```rust
//! use karya_json::types::{JsonParser, JsonValue};
//!
//! // Parse a JSON string
//! let json_str = r#"{"name": "Alice", "age": 30, "is_active": true}"#;
//! let mut parser = JsonParser::new(json_str.to_string());
//! let value = parser.parse().expect("Failed to parse JSON");
//!
//! // Access data from the parsed JSON
//! if let JsonValue::Obj(map) = value {
//!     if let Some(JsonValue::Str(name)) = map.get("name") {
//!         println!("Name: {}", name);
//!     }
//! }
//! ```
//!
//! ## Error Handling
//!
//! Karya JSON provides detailed error types for both serialization and deserialization:
//!
//! - `DeserializeError::InvalidJson`: When the JSON syntax is invalid
//! - `DeserializeError::MissingField`: When a required field is missing
//! - `DeserializeError::TypeMismatch`: When a value has an unexpected type
//! - `DeserializeError::InvalidValue`: When a value is invalid for its context
//!
//! ## Modules
//!
//! - `types`: Core JSON types and parsing functionality

/// Core JSON types and parsing functionality
pub mod types;
