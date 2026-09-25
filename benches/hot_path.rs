#![allow(clippy::expect_used)]
//! Spot 合成响应的离线解析微基准。

use std::hint::black_box;
use std::time::Instant;

use binancex::parse::spot::parse_spot_reference_price;

fn main() {
    const ITERATIONS: u32 = 10_000;
    let input = r#"{"symbol":"SYNTH","referencePrice":"-0"}"#;
    assert!(parse_spot_reference_price(input).is_ok());

    let start = Instant::now();
    for _ in 0..ITERATIONS {
        let value = parse_spot_reference_price(black_box(input)).expect("合成响应解析");
        black_box(value);
    }
    let elapsed = start.elapsed();
    println!(
        "spot_reference_price: {ITERATIONS} 次，单次 {:?}",
        elapsed / ITERATIONS
    );
}
