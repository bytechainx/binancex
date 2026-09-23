//! 数值与单位：`Quantity` / `Sign` / `QuantityUnit`。
//!
//! 形状以 `specs/binancex/contracts/source-library-contract.md` §2.1 为唯一权威。

use crate::error::{BinanceError, BinanceErrorKind, BinanceResult};
use std::fmt;

/// 符号。`value` 为唯一符号真相；`sign` 为派生只读视图。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Sign {
    /// 正数
    Positive,
    /// 负数
    Negative,
    /// 零（含负零 `-0`——按数值判零，原始表示保留）
    Zero,
}

impl fmt::Display for Sign {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Positive => "Positive",
            Self::Negative => "Negative",
            Self::Zero => "Zero",
        })
    }
}

/// 数量单位。跨源语义归类（非官方词表）。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum QuantityUnit {
    /// 基础资产（如 BTC 侧数量）
    BaseAsset,
    /// 报价资产（如 USDT 侧数量 / quoteQty）
    QuoteAsset,
    /// 合约张数（COIN-M）
    Contracts,
}

impl fmt::Display for QuantityUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::BaseAsset => "BaseAsset",
            Self::QuoteAsset => "QuoteAsset",
            Self::Contracts => "Contracts",
        })
    }
}

/// 无损十进制数值——字符串承载（不经 f64 中转）。
///
/// 原始表示保留（含负零、科学计数等）；仅做格式校验，不做算术。
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Decimal(String);

impl Decimal {
    /// 从字符串构造（校验格式：非空、仅 `[0-9.+-eE]`）。
    ///
    /// # Errors
    ///
    /// - [`BinanceErrorKind::Invalid`]：空串或含非法字符
    pub fn new(input: &str) -> BinanceResult<Self> {
        if input.is_empty() {
            return Err(BinanceError::new(BinanceErrorKind::Invalid, "十进制为空"));
        }
        let valid = input
            .chars()
            .all(|c| c.is_ascii_digit() || matches!(c, '.' | '+' | '-' | 'e' | 'E'));
        if !valid {
            return Err(BinanceError::new(
                BinanceErrorKind::Invalid,
                "十进制含非法字符",
            ));
        }
        Ok(Self(input.to_owned()))
    }

    /// 原始字符串表示。
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// 判零（按数值；`-0` 与 `0` 均为 [`Sign::Zero`]）。
    #[must_use]
    pub fn sign(&self) -> Sign {
        let t = self.0.trim_start_matches('+');
        if t.starts_with('-') {
            // -0 按数值判零
            let body = t.strip_prefix('-').unwrap_or(t);
            let stripped = body.replace(['.', '0'], "");
            if stripped.is_empty() {
                Sign::Zero
            } else {
                Sign::Negative
            }
        } else {
            let stripped = t.replace(['.', '0'], "");
            if stripped.is_empty() {
                Sign::Zero
            } else {
                Sign::Positive
            }
        }
    }
}

impl fmt::Display for Decimal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// 数量值对象。
///
/// `value` 为唯一符号真相（原始符号保留，不归一、不取 `abs`）；
/// `sign` 为构造派生的只读视图——不一致状态不可构造。
///
/// 构造唯一入口 [`Quantity::new`]；字段私有 + 访问器。
/// 反序列化必须经由构造不变量（`serde` 反序列化走 `new`）。
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Quantity {
    value: Decimal,
    unit: QuantityUnit,
    sign: Sign,
}

impl Quantity {
    /// 构造（`sign` 由 `value` 派生）。
    ///
    /// # Errors
    ///
    /// 继承 [`Decimal::new`] 的错误。
    pub fn new(value: &str, unit: QuantityUnit) -> BinanceResult<Self> {
        let d = Decimal::new(value)?;
        let sign = d.sign();
        Ok(Self {
            value: d,
            unit,
            sign,
        })
    }

    /// 数值（原始表示）。
    #[must_use]
    pub fn value(&self) -> &Decimal {
        &self.value
    }

    /// 单位。
    #[must_use]
    pub fn unit(&self) -> QuantityUnit {
        self.unit
    }

    /// 符号（派生只读视图）。
    #[must_use]
    pub fn sign(&self) -> Sign {
        self.sign
    }
}

impl fmt::Display for Quantity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.value, self.unit)
    }
}

impl<'de> serde::Deserialize<'de> for Quantity {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        /// 中间结构——确保字段经 `Quantity::new` 构造不变量。
        #[derive(serde::Deserialize)]
        struct Raw {
            value: String,
            unit: QuantityUnit,
        }
        let raw = Raw::deserialize(deserializer)?;
        Quantity::new(&raw.value, raw.unit).map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn negative_quantity_preserves_sign() {
        let q = Quantity::new("-4.70443515", QuantityUnit::BaseAsset).unwrap();
        assert_eq!(q.sign(), Sign::Negative);
        assert_eq!(q.value().as_str(), "-4.70443515");
    }

    #[test]
    fn zero_quantity_is_zero() {
        let q = Quantity::new("0", QuantityUnit::BaseAsset).unwrap();
        assert_eq!(q.sign(), Sign::Zero);
    }

    #[test]
    fn negative_zero_is_zero_but_preserves_representation() {
        let q = Quantity::new("-0", QuantityUnit::BaseAsset).unwrap();
        assert_eq!(q.sign(), Sign::Zero);
        assert_eq!(q.value().as_str(), "-0");
    }

    #[test]
    fn different_units_are_not_equal() {
        let a = Quantity::new("1", QuantityUnit::BaseAsset).unwrap();
        let b = Quantity::new("1", QuantityUnit::QuoteAsset).unwrap();
        assert_ne!(a, b);
    }

    #[test]
    fn invalid_decimal_rejected() {
        assert!(Quantity::new("", QuantityUnit::BaseAsset).is_err());
        assert!(Quantity::new("abc", QuantityUnit::BaseAsset).is_err());
    }
}
