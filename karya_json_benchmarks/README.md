# Karya JSON Benchmarks

This package contains benchmarks for comparing the performance and memory usage of Karya JSON against Serde JSON.

## What's Being Benchmarked

The benchmarks focus exclusively on dynamic types (JsonValue and serde_json::Value) for true impartiality, measuring:

1. **String to Value Conversion (Parsing)**
   - Karya JSON: Using `JsonParser::parse()` to create `JsonValue`
   - Serde JSON: Using `serde_json::from_str()` to create `serde_json::Value`
   - Tested with both small and medium-sized JSON data

2. **Value to String Conversion (Stringifying)**
   - Karya JSON: Using the `Display` implementation (`format!("{}", value)`)
   - Serde JSON: Using `serde_json::to_string()`
   - Tested with both small and medium-sized JSON data

3. **Memory Usage**
   - Measures the memory consumption during parsing and stringifying operations
   - Compares the memory efficiency of both libraries
   - Tested with both small and medium-sized JSON data

## Running the Benchmarks

To run all benchmarks:

```bash
cargo bench -p karya_json_benchmarks
```

To run a specific benchmark group:

```bash
cargo bench -p karya_json_benchmarks -- "String to Value"
```

## Interpreting Results

The benchmark results will show:

- **Time measurements**: Lower is better, indicating faster processing
- **Memory usage**: Lower is better, indicating more efficient memory usage

The results can help you decide which library is more suitable for your specific use case:

- If processing speed is critical, compare the time measurements
- If memory efficiency is important, compare the memory usage metrics
- Both metrics are provided for small and medium-sized JSON to help evaluate performance across different data sizes

## Sample JSON Data

The benchmarks use two sample JSON datasets:

1. **Small JSON**: A simple object with three properties
2. **Medium JSON**: A more complex object with nested arrays and objects

This allows for testing performance across different levels of JSON complexity.
