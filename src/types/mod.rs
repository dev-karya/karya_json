//! Core JSON types and parsing functionality for the Karya JSON library.
//!
//! This module provides the fundamental types and functions for working with JSON data:
//! - `JsonValue`: An enum representing all possible JSON value types
//! - `JsonParser`: A parser for converting JSON strings into `JsonValue` instances
//!
//! # Examples
//!
//! ```
//! use karya_json::types::{JsonParser, JsonValue};
//!
//! // Parse a JSON string
//! let json_str = r#"{"name": "Alice", "age": 30}"#;
//! let mut parser = JsonParser::new(json_str.to_string());
//! let value = parser.parse().expect("Failed to parse JSON");
//!
//! // Work with the parsed data
//! if let JsonValue::Obj(map) = value {
//!     if let Some(JsonValue::Str(name)) = map.get("name") {
//!         assert_eq!(name, "Alice");
//!     }
//!     if let Some(JsonValue::Int(age)) = map.get("age") {
//!         assert_eq!(*age, 30);
//!     }
//! }
//! ```

/// Error types for serialization and deserialization
pub mod error;

use crate::types::error::DeserializeError;
use std::collections::HashMap;

/// Represents a JSON value.
///
/// This enum can represent any valid JSON data type:
/// - `Int`: A 64-bit signed integer
/// - `Float`: A 64-bit floating point number
/// - `Bool`: A boolean value (true or false)
/// - `Str`: A UTF-8 encoded string
/// - `Arr`: An ordered array of JSON values
/// - `Obj`: A key-value map where keys are strings and values are JSON values
/// - `Null`: The JSON null value
///
/// # Examples
///
/// Creating a JSON object:
/// ```
/// use karya_json::types::JsonValue;
/// use std::collections::HashMap;
///
/// let mut map = HashMap::new();
/// map.insert("name".to_string(), JsonValue::Str("Alice".to_string()));
/// map.insert("age".to_string(), JsonValue::Int(30));
///
/// let json_obj = JsonValue::Obj(map);
/// ```
#[derive(Debug)]
pub enum JsonValue {
    /// A 64-bit signed integer
    Int(i64),
    /// A 64-bit floating point number
    Float(f64),
    /// A boolean value (true or false)
    Bool(bool),
    /// A UTF-8 encoded string
    Str(String),
    /// An ordered array of JSON values
    Arr(Vec<JsonValue>),
    /// A key-value map where keys are strings and values are JSON values
    Obj(HashMap<String, JsonValue>),
    /// The JSON null value
    Null,
}

use std::fmt;

/// Implements the Display trait for JsonValue to enable serialization to JSON strings.
///
/// This implementation follows the JSON specification for formatting:
/// - Strings are enclosed in double quotes with proper escaping
/// - Arrays are enclosed in square brackets with comma-separated values
/// - Objects are enclosed in curly braces with comma-separated key-value pairs
/// - Numbers, booleans, and null are formatted according to JSON standards
///
/// # Examples
///
/// ```
/// use karya_json::types::JsonValue;
///
/// let json_value = JsonValue::Bool(true);
/// assert_eq!(format!("{}", json_value), "true");
///
/// let json_value = JsonValue::Str("Hello, world!".to_string());
/// assert_eq!(format!("{}", json_value), "\"Hello, world!\"");
/// ```
impl fmt::Display for JsonValue {
    /// Formats the JsonValue as a JSON string.
    ///
    /// This method is automatically called when using string formatting macros
    /// like `format!`, `println!`, etc. with a JsonValue.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            JsonValue::Int(n) => write!(f, "{}", n),
            JsonValue::Float(n) => write!(f, "{}", n),
            JsonValue::Bool(b) => write!(f, "{}", b),
            JsonValue::Str(s) => write!(f, "\"{}\"", s.replace('\"', "\\\"").replace('\n', "\\n")),
            JsonValue::Arr(arr) => {
                write!(f, "[")?;
                for (i, val) in arr.iter().enumerate() {
                    if i > 0 {
                        write!(f, ",")?;
                    }
                    write!(f, "{}", val)?;
                }
                write!(f, "]")
            }
            JsonValue::Obj(map) => {
                write!(f, "{{")?;
                for (i, (key, val)) in map.iter().enumerate() {
                    if i > 0 {
                        write!(f, ",")?;
                    }
                    write!(f, "\"{}\":{}", key, val)?;
                }
                write!(f, "}}")
            }
            JsonValue::Null => write!(f, "null"),
        }
    }
}

/// A parser for converting JSON strings into `JsonValue` instances.
///
/// The `JsonParser` provides methods for parsing JSON strings according to the
/// JSON specification. It handles all valid JSON data types and provides detailed
/// error messages for invalid JSON.
///
/// # Examples
///
/// ```
/// use karya_json::types::{JsonParser, JsonValue};
///
/// let json_str = r#"{"name": "Alice", "age": 30, "is_active": true}"#;
/// let mut parser = JsonParser::new(json_str.to_string());
/// let result = parser.parse();
///
/// assert!(result.is_ok());
/// ```
#[derive(Debug, Clone)]
pub struct JsonParser {
    /// The input JSON string as a vector of characters
    input: Vec<char>,
    /// The current position in the input
    position: usize,
}

impl JsonParser {
    /// Creates a new JSON parser for the given input string.
    ///
    /// # Arguments
    ///
    /// * `input` - The JSON string to parse
    ///
    /// # Returns
    ///
    /// A new `JsonParser` instance initialized with the input string
    ///
    /// # Examples
    ///
    /// ```
    /// use karya_json::types::JsonParser;
    ///
    /// let parser = JsonParser::new(r#"{"key": "value"}"#.to_string());
    /// ```
    pub fn new(input: String) -> Self {
        Self {
            input: input.chars().collect(),
            position: 0,
        }
    }

    /// Parses the input JSON string into a `JsonValue`.
    ///
    /// This is the main entry point for parsing JSON. It parses the entire input
    /// string and returns a `JsonValue` representing the parsed JSON data.
    ///
    /// # Returns
    ///
    /// * `Ok(JsonValue)` - If the input is valid JSON
    /// * `Err(DeserializeError)` - If the input is invalid JSON
    ///
    /// # Errors
    ///
    /// Returns a `DeserializeError` if:
    /// * The input is not valid JSON
    /// * There are unexpected trailing characters after the JSON value
    ///
    /// # Examples
    ///
    /// ```
    /// use karya_json::types::{JsonParser, JsonValue};
    ///
    /// let mut parser = JsonParser::new(r#"{"name": "Alice"}"#.to_string());
    /// let result = parser.parse();
    ///
    /// assert!(result.is_ok());
    /// ```
    pub fn parse(&mut self) -> Result<JsonValue, DeserializeError> {
        self.skip_whitespace();
        let value = self.parse_value()?;
        self.skip_whitespace();

        // Ensure we've consumed all inputs
        if self.position < self.input.len() {
            return Err(DeserializeError::InvalidJson(
                "Unexpected trailing characters".to_string(),
            ));
        }
        Ok(value)
    }

    // Core parsing methods
    fn parse_value(&mut self) -> Result<JsonValue, DeserializeError> {
        self.skip_whitespace();

        match self.peek_char() {
            Some('"') => self.parse_string().map(JsonValue::Str),
            Some('-') | Some('0'..='9') => self.parse_number(),
            Some('t') | Some('f') => self.parse_boolean().map(JsonValue::Bool),
            Some('n') => self.parse_null().map(|_| JsonValue::Null),
            Some('[') => self.parse_array().map(JsonValue::Arr),
            Some('{') => self.parse_object().map(JsonValue::Obj),
            Some(c) => Err(DeserializeError::InvalidJson(format!(
                "Unexpected character: {}",
                c
            ))),
            None => Err(DeserializeError::InvalidJson(
                "Unexpected end of input".to_string(),
            )),
        }
    }

    // String parsing
    fn parse_string(&mut self) -> Result<String, DeserializeError> {
        self.expect_char('"')?;
        let mut result = String::new();
        let mut is_escaped = false;

        while let Some(c) = self.next_char() {
            match (is_escaped, c) {
                (true, '"') => {
                    result.push('"');
                    is_escaped = false;
                }
                (true, '\\') => {
                    result.push('\\');
                    is_escaped = false;
                }
                (true, '/') => {
                    result.push('/');
                    is_escaped = false;
                }
                (true, 'b') => {
                    result.push('\u{0008}');
                    is_escaped = false;
                }
                (true, 'f') => {
                    result.push('\u{000C}');
                    is_escaped = false;
                }
                (true, 'n') => {
                    result.push('\n');
                    is_escaped = false;
                }
                (true, 'r') => {
                    result.push('\r');
                    is_escaped = false;
                }
                (true, 't') => {
                    result.push('\t');
                    is_escaped = false;
                }
                (true, 'u') => {
                    result.push(self.parse_unicode_escape()?);
                    is_escaped = false;
                }
                (true, _) => {
                    return Err(DeserializeError::InvalidJson(format!(
                        "Invalid escape sequence: \\{}",
                        c
                    )));
                }
                (false, '"') => {
                    return Ok(result);
                }
                (false, '\\') => {
                    is_escaped = true;
                }
                (false, c) if c.is_control() => {
                    return Err(DeserializeError::InvalidJson(
                        "Control characters are not allowed in strings".to_string(),
                    ));
                }
                (false, c) => {
                    result.push(c);
                }
            }
        }

        Err(DeserializeError::InvalidJson(
            "Unterminated string".to_string(),
        ))
    }

    // Number parsing
    fn parse_number(&mut self) -> Result<JsonValue, DeserializeError> {
        let mut number_str = String::new();
        let mut has_decimal = false;
        let mut has_exponent = false;
        let mut is_negative = false;

        // Handle negative numbers
        if self.peek_char() == Some('-') {
            is_negative = true;
            number_str.push(self.next_char().unwrap());
        }

        // Parse integer part
        match self.peek_char() {
            Some('0') => {
                number_str.push(self.next_char().unwrap());
            }
            Some('1'..='9') => {
                while let Some(c) = self.peek_char() {
                    if !c.is_ascii_digit() {
                        break;
                    }
                    number_str.push(self.next_char().unwrap());
                }
            }
            _ => {
                return Err(DeserializeError::InvalidJson(
                    "Invalid number format".to_string(),
                ));
            }
        }

        // Parse decimal part
        if self.peek_char() == Some('.') {
            has_decimal = true;
            number_str.push(self.next_char().unwrap());

            let mut has_digits = false;
            while let Some(c) = self.peek_char() {
                if !c.is_ascii_digit() {
                    break;
                }
                has_digits = true;
                number_str.push(self.next_char().unwrap());
            }

            if !has_digits {
                return Err(DeserializeError::InvalidJson(
                    "Expected digits after decimal point".to_string(),
                ));
            }
        }

        // Parse exponent
        if let Some('e' | 'E') = self.peek_char() {
            has_exponent = true;
            number_str.push(self.next_char().unwrap());

            // Handle exponent sign
            match self.peek_char() {
                Some('+' | '-') => number_str.push(self.next_char().unwrap()),
                _ => {}
            }

            let mut has_digits = false;
            while let Some(c) = self.peek_char() {
                if !c.is_ascii_digit() {
                    break;
                }
                has_digits = true;
                number_str.push(self.next_char().unwrap());
            }

            if !has_digits {
                return Err(DeserializeError::InvalidJson(
                    "Expected digits in exponent".to_string(),
                ));
            }
        }

        // If it's an integer with no decimal or exponent, parse as i64
        if !has_decimal && !has_exponent {
            match number_str.parse::<i64>() {
                Ok(int_value) => Ok(JsonValue::Int(int_value)),
                Err(_) => {
                    // If i64 parsing fails, try f64 as fallback
                    number_str
                        .parse::<f64>()
                        .map(JsonValue::Float)
                        .map_err(|_| DeserializeError::InvalidJson(format!("Invalid number: {}", number_str)))
                }
            }
        } else {
            // Parse as float for decimal or exponent numbers
            number_str
                .parse::<f64>()
                .map(JsonValue::Float)
                .map_err(|_| DeserializeError::InvalidJson(format!("Invalid number: {}", number_str)))
        }
    }

    // Boolean parsing
    fn parse_boolean(&mut self) -> Result<bool, DeserializeError> {
        match self.peek_char() {
            Some('t') => {
                self.expect_literal("true")?;
                Ok(true)
            }
            Some('f') => {
                self.expect_literal("false")?;
                Ok(false)
            }
            _ => Err(DeserializeError::InvalidJson(
                "Expected boolean value".to_string(),
            )),
        }
    }

    // Null parsing
    fn parse_null(&mut self) -> Result<(), DeserializeError> {
        self.expect_literal("null")
    }

    // Array parsing
    fn parse_array(&mut self) -> Result<Vec<JsonValue>, DeserializeError> {
        self.expect_char('[')?;
        let mut array = Vec::new();
        self.skip_whitespace();

        if self.peek_char() == Some(']') {
            self.next_char();
            return Ok(array);
        }

        loop {
            self.skip_whitespace();
            array.push(self.parse_value()?);
            self.skip_whitespace();

            match self.next_char() {
                Some(',') => continue,
                Some(']') => break,
                Some(c) => {
                    return Err(DeserializeError::InvalidJson(format!(
                        "Expected ',' or ']', found '{}'",
                        c
                    )));
                }
                None => {
                    return Err(DeserializeError::InvalidJson(
                        "Unterminated array".to_string(),
                    ));
                }
            }
        }

        Ok(array)
    }

    // Object parsing
    fn parse_object(&mut self) -> Result<HashMap<String, JsonValue>, DeserializeError> {
        self.expect_char('{')?;
        let mut object = HashMap::new();
        self.skip_whitespace();

        if self.peek_char() == Some('}') {
            self.next_char();
            return Ok(object);
        }

        loop {
            self.skip_whitespace();
            let key = self.parse_string()?;
            self.skip_whitespace();

            self.expect_char(':')?;
            self.skip_whitespace();

            let value = self.parse_value()?;
            object.insert(key, value);
            self.skip_whitespace();

            match self.next_char() {
                Some(',') => continue,
                Some('}') => break,
                Some(c) => {
                    return Err(DeserializeError::InvalidJson(format!(
                        "Expected ',' or '}}', found '{}'",
                        c
                    )));
                }
                None => {
                    return Err(DeserializeError::InvalidJson(
                        "Unterminated object".to_string(),
                    ));
                }
            }
        }

        Ok(object)
    }

    // Helper methods
    fn parse_unicode_escape(&mut self) -> Result<char, DeserializeError> {
        let mut code_point = 0u32;
        for _ in 0..4 {
            code_point = code_point * 16
                + match self.next_char() {
                    Some(c) => c.to_digit(16).ok_or_else(|| {
                        DeserializeError::InvalidJson(format!(
                            "Invalid Unicode escape sequence: {}",
                            c
                        ))
                    })?,
                    None => {
                        return Err(DeserializeError::InvalidJson(
                            "Unexpected end of Unicode escape sequence".to_string(),
                        ));
                    }
                };
        }

        char::from_u32(code_point).ok_or_else(|| {
            DeserializeError::InvalidJson(format!("Invalid Unicode code point: {}", code_point))
        })
    }

    fn expect_char(&mut self, expected: char) -> Result<(), DeserializeError> {
        match self.next_char() {
            Some(c) if c == expected => Ok(()),
            Some(c) => Err(DeserializeError::InvalidJson(format!(
                "Expected '{}', found '{}'",
                expected, c
            ))),
            None => Err(DeserializeError::InvalidJson(format!(
                "Expected '{}', found end of input",
                expected
            ))),
        }
    }

    fn expect_literal(&mut self, literal: &str) -> Result<(), DeserializeError> {
        for expected in literal.chars() {
            match self.next_char() {
                Some(c) if c == expected => continue,
                Some(c) => {
                    return Err(DeserializeError::InvalidJson(format!(
                        "Expected '{}', found '{}'",
                        expected, c
                    )));
                }
                None => {
                    return Err(DeserializeError::InvalidJson(format!(
                        "Expected '{}', found end of input",
                        expected
                    )));
                }
            }
        }
        Ok(())
    }

    fn peek_char(&self) -> Option<char> {
        self.input.get(self.position).copied()
    }

    fn next_char(&mut self) -> Option<char> {
        let c = self.peek_char();
        if c.is_some() {
            self.position += 1;
        }
        c
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.peek_char() {
            if !c.is_whitespace() {
                break;
            }
            self.position += 1;
        }
    }
}

// Unit tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_string() {
        let mut parser = JsonParser::new(r#""Hello, World!""#.to_string());
        assert_eq!(parser.parse_string().unwrap(), "Hello, World!");
    }

    #[test]
    fn test_parse_number() {
        // Test integer parsing
        let mut parser = JsonParser::new("123".to_string());
        match parser.parse_number().unwrap() {
            JsonValue::Int(n) => assert_eq!(n, 123),
            _ => panic!("Expected JsonValue::Int"),
        }

        // Test float parsing
        let mut parser = JsonParser::new("123.456".to_string());
        match parser.parse_number().unwrap() {
            JsonValue::Float(n) => assert_eq!(n, 123.456),
            _ => panic!("Expected JsonValue::Float"),
        }

        // Test negative float with exponent
        let mut parser = JsonParser::new("-123.456e-10".to_string());
        match parser.parse_number().unwrap() {
            JsonValue::Float(n) => assert_eq!(n, -123.456e-10),
            _ => panic!("Expected JsonValue::Float"),
        }

        // Test integer with exponent (should be a float)
        let mut parser = JsonParser::new("123e2".to_string());
        match parser.parse_number().unwrap() {
            JsonValue::Float(n) => assert_eq!(n, 12300f64),
            _ => panic!("Expected JsonValue::Float"),
        }
    }

    #[test]
    fn test_parse_boolean() {
        let mut parser = JsonParser::new("true".to_string());
        assert_eq!(parser.parse_boolean().unwrap(), true);

        let mut parser = JsonParser::new("false".to_string());
        assert_eq!(parser.parse_boolean().unwrap(), false);
    }

    #[test]
    fn test_parse_null() {
        let mut parser = JsonParser::new("null".to_string());
        assert!(parser.parse_null().is_ok());
    }

    #[test]
    fn test_parse_array() {
        let mut parser = JsonParser::new("[1, 2, 3]".to_string());
        let array = parser.parse_array().unwrap();
        assert_eq!(array.len(), 3);
    }

    #[test]
    fn test_parse_object() {
        let mut parser = JsonParser::new(r#"{"name": "John", "age": 30}"#.to_string());
        let object = parser.parse_object().unwrap();
        assert_eq!(object.len(), 2);
    }

    #[test]
    fn test_complex_json() {
        let json = r#"
        {
            "name": "John Doe",
            "age": 30,
            "is_student": false,
            "grades": [85, 90, 92],
            "address": {
                "street": "123 Main St",
                "city": "Anytown"
            },
            "phone": null,
            "score": 98.6,
            "exp": 1.23e4
        }"#;

        let mut parser = JsonParser::new(json.to_string());
        let result = parser.parse();
        assert!(result.is_ok());

        // Verify that the parsed result contains correctly typed numbers
        if let Ok(JsonValue::Obj(obj)) = result {
            // Verify integer
            if let Some(JsonValue::Int(age)) = obj.get("age") {
                assert_eq!(*age, 30);
            } else {
                panic!("Expected age to be an integer");
            }

            // Verify float
            if let Some(JsonValue::Float(score)) = obj.get("score") {
                assert_eq!(*score, 98.6);
            } else {
                panic!("Expected score to be a float");
            }

            // Verify exponent
            if let Some(JsonValue::Float(exp)) = obj.get("exp") {
                assert_eq!(*exp, 12300.0);
            } else {
                panic!("Expected exp to be a float");
            }
        } else {
            panic!("Failed to parse complex JSON");
        }
    }
}
