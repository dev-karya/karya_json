use criterion::{black_box, criterion_group, criterion_main, Criterion};
use karya_json::types::{JsonParser, JsonValue};
use memory_stats::memory_stats;
use serde_json::Value as SerdeValue;
use std::collections::HashMap;

// Sample JSON data for benchmarking
const SMALL_JSON: &str = r#"{"name":"John Doe","age":30,"is_active":true}"#;
const MEDIUM_JSON: &str = r#"
{
    "id": 123456,
    "name": "Product Name",
    "description": "This is a sample product description that is longer than the small JSON example.",
    "price": 99.99,
    "in_stock": true,
    "tags": ["electronics", "gadgets", "new"],
    "dimensions": {
        "width": 10.5,
        "height": 15.2,
        "depth": 3.0
    },
    "reviews": [
        {"user": "user1", "rating": 5, "comment": "Great product!"},
        {"user": "user2", "rating": 4, "comment": "Good value for money."}
    ]
}
"#;

// Function to measure memory usage
fn measure_memory<F>(f: F) -> (usize, usize)
where
    F: FnOnce(),
{
    let before = memory_stats().map(|stats| stats.physical_mem).unwrap_or(0);
    f();
    let after = memory_stats().map(|stats| stats.physical_mem).unwrap_or(0);
    (before, after)
}

// Benchmark String to Value conversion using karya-json
fn bench_karya_json_parse(c: &mut Criterion) {
    let mut group = c.benchmark_group("String to Value - Parse");

    group.bench_function("karya_json - small", |b| {
        b.iter(|| {
            let mut parser = JsonParser::new(black_box(SMALL_JSON.to_string()));
            let _value = parser.parse().unwrap();
        })
    });

    group.bench_function("karya_json - medium", |b| {
        b.iter(|| {
            let mut parser = JsonParser::new(black_box(MEDIUM_JSON.to_string()));
            let _value = parser.parse().unwrap();
        })
    });

    group.finish();
}

// Benchmark Value to String conversion using karya-json
fn bench_karya_json_stringify(c: &mut Criterion) {
    let mut group = c.benchmark_group("Value to String - Stringify");

    // Create a JsonValue for small JSON
    let mut small_map = HashMap::new();
    small_map.insert("name".to_string(), JsonValue::Str("John Doe".to_string()));
    small_map.insert("age".to_string(), JsonValue::Int(30));
    small_map.insert("is_active".to_string(), JsonValue::Bool(true));
    let small_value = JsonValue::Obj(small_map);

    // Parse medium JSON to get a JsonValue
    let mut parser = JsonParser::new(MEDIUM_JSON.to_string());
    let medium_value = parser.parse().unwrap();

    group.bench_function("karya_json - small", |b| {
        b.iter(|| {
            let _json_string = black_box(format!("{}", small_value));
        })
    });

    group.bench_function("karya_json - medium", |b| {
        b.iter(|| {
            let _json_string = black_box(format!("{}", medium_value));
        })
    });

    group.finish();
}

// Benchmark String to Value conversion using serde_json
fn bench_serde_json_parse(c: &mut Criterion) {
    let mut group = c.benchmark_group("String to Value - Parse");

    group.bench_function("serde_json - small", |b| {
        b.iter(|| {
            let _value: SerdeValue = serde_json::from_str(black_box(SMALL_JSON)).unwrap();
        })
    });

    group.bench_function("serde_json - medium", |b| {
        b.iter(|| {
            let _value: SerdeValue = serde_json::from_str(black_box(MEDIUM_JSON)).unwrap();
        })
    });

    group.finish();
}

// Benchmark Value to String conversion using serde_json
fn bench_serde_json_stringify(c: &mut Criterion) {
    let mut group = c.benchmark_group("Value to String - Stringify");

    // Parse JSON to get SerdeValue
    let small_value: SerdeValue = serde_json::from_str(SMALL_JSON).unwrap();
    let medium_value: SerdeValue = serde_json::from_str(MEDIUM_JSON).unwrap();

    group.bench_function("serde_json - small", |b| {
        b.iter(|| {
            let _json_string = black_box(serde_json::to_string(&small_value).unwrap());
        })
    });

    group.bench_function("serde_json - medium", |b| {
        b.iter(|| {
            let _json_string = black_box(serde_json::to_string(&medium_value).unwrap());
        })
    });

    group.finish();
}

// Benchmark memory usage
fn bench_memory_usage(c: &mut Criterion) {
    let mut group = c.benchmark_group("Memory Usage");

    // Memory usage for karya_json parsing - small JSON
    group.bench_function("karya_json parse - small", |b| {
        b.iter(|| {
            let (before, after) = measure_memory(|| {
                for _ in 0..1000 {
                    let mut parser = JsonParser::new(SMALL_JSON.to_string());
                    black_box(parser.parse().unwrap());
                }
            });
            black_box((after - before) / 1000);
        })
    });

    // Memory usage for karya_json parsing - medium JSON
    group.bench_function("karya_json parse - medium", |b| {
        b.iter(|| {
            let (before, after) = measure_memory(|| {
                for _ in 0..100 {  // Using fewer iterations for larger JSON
                    let mut parser = JsonParser::new(MEDIUM_JSON.to_string());
                    black_box(parser.parse().unwrap());
                }
            });
            black_box((after - before) / 100);
        })
    });

    // Memory usage for serde_json parsing - small JSON
    group.bench_function("serde_json parse - small", |b| {
        b.iter(|| {
            let (before, after) = measure_memory(|| {
                for _ in 0..1000 {
                    black_box(serde_json::from_str::<SerdeValue>(SMALL_JSON).unwrap());
                }
            });
            black_box((after - before) / 1000);
        })
    });

    // Memory usage for serde_json parsing - medium JSON
    group.bench_function("serde_json parse - medium", |b| {
        b.iter(|| {
            let (before, after) = measure_memory(|| {
                for _ in 0..100 {  // Using fewer iterations for larger JSON
                    black_box(serde_json::from_str::<SerdeValue>(MEDIUM_JSON).unwrap());
                }
            });
            black_box((after - before) / 100);
        })
    });

    // Memory usage for karya_json stringify - small JSON
    group.bench_function("karya_json stringify - small", |b| {
        b.iter(|| {
            let mut small_map = HashMap::new();
            small_map.insert("name".to_string(), JsonValue::Str("John Doe".to_string()));
            small_map.insert("age".to_string(), JsonValue::Int(30));
            small_map.insert("is_active".to_string(), JsonValue::Bool(true));
            let small_value = JsonValue::Obj(small_map);

            let (before, after) = measure_memory(|| {
                for _ in 0..1000 {
                    black_box(format!("{}", small_value));
                }
            });
            black_box((after - before) / 1000);
        })
    });

    // Memory usage for karya_json stringify - medium JSON
    group.bench_function("karya_json stringify - medium", |b| {
        b.iter(|| {
            let mut parser = JsonParser::new(MEDIUM_JSON.to_string());
            let medium_value = parser.parse().unwrap();

            let (before, after) = measure_memory(|| {
                for _ in 0..100 {  // Using fewer iterations for larger JSON
                    black_box(format!("{}", medium_value));
                }
            });
            black_box((after - before) / 100);
        })
    });

    // Memory usage for serde_json stringify - small JSON
    group.bench_function("serde_json stringify - small", |b| {
        b.iter(|| {
            let small_value: SerdeValue = serde_json::from_str(SMALL_JSON).unwrap();

            let (before, after) = measure_memory(|| {
                for _ in 0..1000 {
                    black_box(serde_json::to_string(&small_value).unwrap());
                }
            });
            black_box((after - before) / 1000);
        })
    });

    // Memory usage for serde_json stringify - medium JSON
    group.bench_function("serde_json stringify - medium", |b| {
        b.iter(|| {
            let medium_value: SerdeValue = serde_json::from_str(MEDIUM_JSON).unwrap();

            let (before, after) = measure_memory(|| {
                for _ in 0..100 {  // Using fewer iterations for larger JSON
                    black_box(serde_json::to_string(&medium_value).unwrap());
                }
            });
            black_box((after - before) / 100);
        })
    });

    group.finish();
}


// Combine all benchmarks
criterion_group!(
    benches,
    bench_karya_json_parse,
    bench_serde_json_parse,
    bench_karya_json_stringify,
    bench_serde_json_stringify,
    bench_memory_usage
);
criterion_main!(benches);
