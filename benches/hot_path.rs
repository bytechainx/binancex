//! 离线平均价格响应解析微基准；`--quick` 缩短本地自检。

#![forbid(unsafe_code)]

use std::hint::black_box;
use std::time::Instant;

use binancex::parse::spot::parse_spot_avg_price;

/// 合成夹具：元数据不属于源响应，解析器只接收 payload。
const SAMPLE: &str = r#"{
    "_synthetic": true,
    "_note": "微基准合成样本，非真实源数据，不构成源证据",
    "payload": {"mins": 5, "price": "12345.6700", "closeTime": 1720000000000}
}"#;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let fixture: serde_json::Value = serde_json::from_str(SAMPLE)?;
    let payload = fixture
        .get("payload")
        .ok_or("合成夹具缺少 payload")?
        .to_string();
    let n: u32 = if std::env::args().any(|arg| arg == "--quick") {
        200
    } else {
        5_000
    };

    for _ in 0..10 {
        black_box(parse_spot_avg_price(black_box(&payload))?);
    }
    let start = Instant::now();
    for _ in 0..n {
        black_box(parse_spot_avg_price(black_box(&payload))?);
    }
    let elapsed = start.elapsed();
    println!(
        "离线平均价格解析：迭代={n} 总耗时={elapsed:?} 单次耗时={:?}",
        elapsed / n
    );
    Ok(())
}
