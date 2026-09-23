//! 共享内核门面：子模块声明 + `pub use` + `validate*` 入口。
//!
//! 内核实现分布在 `src/value/` 各文件；布局以
//! `specs/binancex/contracts/source-library-contract.md` §8 为唯一权威。

pub mod identity;
pub mod numeric;
pub mod time;

pub use identity::{DataSeriesId, EndpointId, Instrument, Subject};
pub use numeric::{Quantity, QuantityUnit, Sign};
pub use time::{Date, Interval, PitEligibility, TimePrecision};

use crate::error::BinanceResult;

/// 校验字符串为无损十进制数值（非空、仅含 `[0-9.+-]`、至多一个小数点）。
///
/// # Errors
///
/// - [`BinanceErrorKind::Invalid`]：空串或含非法字符
/// - [`BinanceErrorKind::LossyNumeric`]：多个小数点
pub fn validate_decimal(input: &str) -> BinanceResult<()> {
    if input.is_empty() {
        return Err(crate::error::BinanceError::new(
            crate::error::BinanceErrorKind::Invalid,
            "十进制数值为空",
        ));
    }
    let dot_count = input.chars().filter(|&c| c == '.').count();
    if dot_count > 1 {
        return Err(crate::error::BinanceError::new(
            crate::error::BinanceErrorKind::LossyNumeric,
            "十进制数值含多个小数点",
        ));
    }
    let valid = input
        .chars()
        .all(|c| c.is_ascii_digit() || c == '.' || c == '+' || c == '-');
    if !valid {
        return Err(crate::error::BinanceError::new(
            crate::error::BinanceErrorKind::Invalid,
            "十进制数值含非法字符",
        ));
    }
    Ok(())
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
