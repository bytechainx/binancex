//! 共享内核门面：子模块声明 + `pub use` + `validate*` 入口。
//!
//! 内核实现分布在 `src/value/` 各文件；布局以
//! `specs/binancex/contracts/source-library-contract.md` §8 为唯一权威。

pub mod coinm;
pub mod identity;
pub mod numeric;
pub mod options;
pub mod spot;
pub mod time;
pub mod usdm;
pub mod whitelist;

pub use identity::{DataSeriesId, EndpointId, Instrument, Subject};
pub use numeric::{Decimal, Quantity, QuantityUnit, Sign};
pub use time::{Date, Interval, PitEligibility, TimePrecision};
pub use whitelist::WhitelistSnapshot;

use crate::error::BinanceResult;

/// 通过 [`Decimal::new`] 校验完整十进制语法，支持可选符号、小数与科学计数。
///
/// # Errors
///
/// - [`crate::error::BinanceErrorKind::Invalid`]：不符合完整十进制语法
pub fn validate_decimal(input: &str) -> BinanceResult<()> {
    numeric::Decimal::new(input).map(|_| ())
}

/// 校验字符串为 64 位小写 hex（SHA-256 摘要格式）。
///
/// # Errors
///
/// - [`BinanceErrorKind::Invalid`]：长度非 64 或含非 hex 字符
pub fn validate_sha256_hex(input: &str) -> BinanceResult<()> {
    if input.len() != 64
        || !input
            .chars()
            .all(|c| c.is_ascii_hexdigit() && !c.is_ascii_uppercase())
    {
        return Err(crate::error::BinanceError::new(
            crate::error::BinanceErrorKind::Invalid,
            "SHA-256 摘要须为 64 位小写 hex",
        ));
    }
    Ok(())
}
