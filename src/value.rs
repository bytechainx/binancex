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

/// 一个冻结 K 线行；具体产品族和端点由外层名义类型区分。
pub type KlineRow = (
    i64,
    String,
    String,
    String,
    String,
    String,
    i64,
    String,
    i64,
    String,
    String,
    String,
);

macro_rules! typed_kline {
    ($name:ident, $documentation:literal) => {
        #[doc = $documentation]
        #[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
        #[serde(transparent)]
        pub struct $name(#[doc = "按冻结合同顺序排列的 K 线行。"] pub Vec<$crate::value::KlineRow>);

        impl std::ops::Deref for $name {
            type Target = [$crate::value::KlineRow];

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
    };
}

pub(crate) use typed_kline;

pub use identity::{DataSeriesEntity, DataSeriesId, EndpointId, Instrument, Subject};
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
