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
    /// 从字符串构造，接受可选符号、小数点与科学计数指数。
    ///
    /// 有效数字部分至少含一位数字；指数若出现，须含至少一位整数数字。
    ///
    /// # Errors
    ///
    /// - [`BinanceErrorKind::Invalid`]：不符合十进制语法
    pub fn new(input: &str) -> BinanceResult<Self> {
        let unsigned = input.strip_prefix(['+', '-']).unwrap_or(input);
        let (mantissa, exponent) = unsigned
            .split_once(['e', 'E'])
            .map_or((unsigned, None), |(value, exponent)| {
                (value, Some(exponent))
            });
        let digits = |value: &str| value.bytes().all(|byte| byte.is_ascii_digit());
        let valid_mantissa = match mantissa.split_once('.') {
            Some((whole, fraction)) => {
                !(whole.is_empty() && fraction.is_empty()) && digits(whole) && digits(fraction)
            }
            None => !mantissa.is_empty() && digits(mantissa),
        };
        let valid_exponent = exponent.map_or(true, |value| {
            let value = value.strip_prefix(['+', '-']).unwrap_or(value);
            !value.is_empty() && digits(value)
        });
        if !valid_mantissa || !valid_exponent {
            return Err(BinanceError::new(
                BinanceErrorKind::Invalid,
                "十进制数值格式无效",
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
        // 指数不改变零与正负号，只检查有效数字部分。
        let mantissa = self.0.split(['e', 'E']).next().unwrap_or(&self.0);
        if !mantissa.bytes().any(|byte| matches!(byte, b'1'..=b'9')) {
            Sign::Zero
        } else if mantissa.starts_with('-') {
            Sign::Negative
        } else {
            Sign::Positive
        }
    }
}

impl<'de> serde::Deserialize<'de> for Decimal {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let raw = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::new(&raw).map_err(serde::de::Error::custom)
    }
}

impl serde::Serialize for Decimal {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
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
    /// 构造（`sign` 由已校验的 `Decimal` 派生）。
    #[must_use]
    pub fn new(value: Decimal, unit: QuantityUnit) -> Self {
        let sign = value.sign();
        Self { value, unit, sign }
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
        let value = Decimal::new(&raw.value).map_err(serde::de::Error::custom)?;
        Ok(Quantity::new(value, raw.unit))
    }
}

impl serde::Serialize for Quantity {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        #[derive(serde::Serialize)]
        struct Raw<'a> {
            value: &'a str,
            unit: QuantityUnit,
        }

        serde::Serialize::serialize(
            &Raw {
                value: self.value.as_str(),
                unit: self.unit,
            },
            serializer,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn public_decimal_validator_uses_constructor_syntax() {
        for input in ["1e3", "-0", "+001.2300E-999", ".5"] {
            assert!(crate::value::validate_decimal(input).is_ok());
        }
        for input in ["+", "1..2", "1e", ""] {
            let error = crate::value::validate_decimal(input).unwrap_err();
            assert_eq!(error.kind(), BinanceErrorKind::Invalid);
            assert_eq!(error.kind(), Decimal::new(input).unwrap_err().kind());
        }
    }

    #[test]
    fn negative_quantity_preserves_sign() {
        let q = Quantity::new(
            Decimal::new("-4.70443515").unwrap(),
            QuantityUnit::BaseAsset,
        );
        assert_eq!(q.sign(), Sign::Negative);
        assert_eq!(q.value().as_str(), "-4.70443515");
    }

    #[test]
    fn zero_quantity_is_zero() {
        let q = Quantity::new(Decimal::new("0").unwrap(), QuantityUnit::BaseAsset);
        assert_eq!(q.sign(), Sign::Zero);
    }

    #[test]
    fn negative_zero_is_zero_but_preserves_representation() {
        let q = Quantity::new(Decimal::new("-0").unwrap(), QuantityUnit::BaseAsset);
        assert_eq!(q.sign(), Sign::Zero);
        assert_eq!(q.value().as_str(), "-0");
    }

    #[test]
    fn different_units_are_not_equal() {
        let a = Quantity::new(Decimal::new("1").unwrap(), QuantityUnit::BaseAsset);
        let b = Quantity::new(Decimal::new("1").unwrap(), QuantityUnit::QuoteAsset);
        assert_ne!(a, b);
    }

    #[test]
    fn invalid_decimal_rejected() {
        assert!(Decimal::new("").is_err());
        assert!(Decimal::new("abc").is_err());
    }

    #[test]
    fn decimal_deserialization_preserves_wire_string() {
        let value: Decimal = serde_json::from_str(r#""-0""#).unwrap();
        assert_eq!(value.as_str(), "-0");
        assert_eq!(value.sign(), Sign::Zero);
    }

    #[test]
    fn decimal_deserialization_validates_syntax_without_normalizing() {
        for (input, sign) in [
            ("-4.70443515", Sign::Negative),
            ("-0.000e+999", Sign::Zero),
            ("+001.2300E-999", Sign::Positive),
            (".5", Sign::Positive),
            ("1.", Sign::Positive),
            ("123456789012345678901234567890.00001", Sign::Positive),
        ] {
            let json = serde_json::to_string(input).unwrap();
            let parsed: Decimal = serde_json::from_str(&json).unwrap();
            assert_eq!(parsed, Decimal::new(input).unwrap());
            assert_eq!(parsed.as_str(), input);
            assert_eq!(parsed.sign(), sign);
            assert_eq!(
                Quantity::new(Decimal::new(input).unwrap(), QuantityUnit::BaseAsset).sign(),
                sign
            );
        }
        for input in [
            "", "+", "-", ".", "1e", "1..2", "1e+", "e2", "--1", "1-2", "1e2e3", " 1", "NaN",
            "１２",
        ] {
            let json = serde_json::to_string(input).unwrap();
            assert!(Decimal::new(input).is_err(), "构造器须拒绝：{input}");
            assert!(
                serde_json::from_str::<Decimal>(&json).is_err(),
                "反序列化须拒绝：{input}"
            );
            let quantity = format!(r#"{{"value":{json},"unit":"BaseAsset"}}"#);
            assert!(serde_json::from_str::<Quantity>(&quantity).is_err());
        }
        for input in ["123", "null", "true", "[]", "{}"] {
            assert!(serde_json::from_str::<Decimal>(input).is_err());
        }
    }

    #[test]
    fn quantity_serialization_round_trip_preserves_value_and_unit() {
        for value in ["-0", "+001.2300E-999"] {
            let quantity = Quantity::new(Decimal::new(value).unwrap(), QuantityUnit::Contracts);
            let json = serde_json::to_string(&quantity).unwrap();
            let round_trip: Quantity = serde_json::from_str(&json).unwrap();
            assert_eq!(round_trip.value().as_str(), value);
            assert_eq!(round_trip.unit(), QuantityUnit::Contracts);
            assert_eq!(json, format!(r#"{{"value":"{value}","unit":"Contracts"}}"#));
        }
    }
}
