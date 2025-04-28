//! # Karya JSON Benchmarks
//!
//! This crate contains benchmarks for comparing the performance and memory usage
//! of Karya JSON against Serde JSON.
//!
//! The benchmarks measure:
//!
//! 1. String to Value conversion (parsing)
//! 2. Value to String conversion (stringifying)
//! 3. Memory usage during these operations
//!
//! See the README.md file for more information on running the benchmarks
//! and interpreting the results.

// This crate is primarily for benchmarks, so there's no public API.
// The actual benchmark code is in the benches directory.