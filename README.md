# Karya JSON

[![Crates.io](https://img.shields.io/crates/v/karya_json.svg)](https://crates.io/crates/karya_json)
[![Documentation](https://docs.rs/karya_json/badge.svg)](https://docs.rs/karya_json)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A high-performance, focused JSON serialization and deserialization library for Rust. Designed as a faster alternative to Serde when working specifically with JSON.

## Why Karya JSON?

While Serde is an excellent and versatile serialization framework that supports many formats, Karya JSON focuses exclusively on JSON processing with these advantages:

- **Speed-focused**: Built from the ground up for optimal JSON performance
- **Lightweight**: Smaller footprint with fewer dependencies
- **Simple API**: Streamlined interface designed specifically for JSON
- **Lower compile times**: Reduced macro complexity means faster builds

## Installation

Add this to your `Cargo.toml`:

```
[dependencies]
karya_json = "0.1.0"
```

## Quick Start

Karya JSON provides a simple API for parsing JSON strings and working with JSON data:

1. Parse a JSON string into a `JsonValue`:
   - Use `JsonParser::new(json_string)` to create a parser
   - Call `parser.parse()` to get a `JsonValue`

2. Access data from the parsed JSON:
   - Use pattern matching to work with different JSON types
   - Access object fields, array elements, and primitive values

3. Convert Rust data to JSON:
   - Create `JsonValue` instances for your data
   - Use the `Display` implementation to get the JSON string

## Features

- **RFC 8259 Compliant**: Fully compliant with the JSON specification (RFC 8259)
- **Parsing**: Convert JSON strings into Rust data structures
- **Serialization**: Convert Rust data structures into JSON strings
- **Type Safety**: Strong typing for JSON values
- **Error Handling**: Detailed error messages for parsing and serialization issues
- **Unicode Support**: Complete Unicode support including surrogate pairs

## Error Handling

Karya JSON provides detailed error types for both serialization and deserialization:

- `DeserializeError::InvalidJson`: When the JSON syntax is invalid
- `DeserializeError::MissingField`: When a required field is missing
- `DeserializeError::TypeMismatch`: When a value has an unexpected type
- `DeserializeError::InvalidValue`: When a value is invalid for its context

## Benchmarks

The project includes a benchmark package that compares the performance and memory usage of Karya JSON against Serde JSON:

```bash
# Run all benchmarks
cargo bench -p karya_json_benchmarks

# Run a specific benchmark group
cargo bench -p karya_json_benchmarks -- "String to Value"
```

The benchmarks measure:
- String to Value conversion (parsing)
- Value to String conversion (stringifying)
- Memory usage during these operations

See the [karya_json_benchmarks/README.md](karya_json_benchmarks/README.md) file for more details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the LICENSE file for details.
