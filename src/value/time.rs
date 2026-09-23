//! 时间：`TimePrecision` / `Date` / `Interval` / `PitEligibility`。

use crate::error::{BinanceError, BinanceErrorKind, BinanceResult};
use std::fmt;

/// 时间精度。由来源决定，不跨族复制。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TimePrecision {
    /// 毫秒
    Ms,
    /// 秒
    S,
    /// 微秒
    Us,
}

impl fmt::Display for TimePrecision {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Ms => "ms",
            Self::S => "s",
            Self::Us => "us",
        })
    }
}

/// 民用日历日值对象（不引 `chrono` / `time`）。
///
/// `Date::new` 校验年月日合法性（含闰年）。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Date {
    year: i32,
    month: u8,
    day: u8,
}

fn is_leap(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
}

fn days_in_month(year: i32, month: u8) -> u8 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            if is_leap(year) {
                29
            } else {
                28
            }
        }
        _ => 0,
    }
}

impl Date {
    /// 构造（越界拒绝）。
    ///
    /// # Errors
    ///
    /// - [`BinanceErrorKind::Invalid`]：月份不在 1..=12、日期超出当月天数
    pub fn new(year: i32, month: u8, day: u8) -> BinanceResult<Self> {
        if !(1..=12).contains(&month) {
            return Err(BinanceError::new(
                BinanceErrorKind::Invalid,
                format!("月份 {month} 不在 1..=12"),
            ));
        }
        let max = days_in_month(year, month);
        if day == 0 || day > max {
            return Err(BinanceError::new(
                BinanceErrorKind::Invalid,
                format!("日期 {day} 超出 {year} 年 {month} 月的 1..={max}"),
            ));
        }
        Ok(Self { year, month, day })
    }

    /// 年。
    #[must_use]
    pub fn year(&self) -> i32 {
        self.year
    }

    /// 月。
    #[must_use]
    pub fn month(&self) -> u8 {
        self.month
    }

    /// 日。
    #[must_use]
    pub fn day(&self) -> u8 {
        self.day
    }
}

impl fmt::Display for Date {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:04}-{:02}-{:02}", self.year, self.month, self.day)
    }
}

/// 原生离散周期（含 Spot 的 1s、3d 等）。
///
/// 域允许集不支持时登记差异，先修合同再启用，不静默裁掉。
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Interval(String);

impl Interval {
    /// 构造（非空校验）。
    ///
    /// # Errors
    ///
    /// - [`BinanceErrorKind::Invalid`]：空串
    pub fn new(s: &str) -> BinanceResult<Self> {
        if s.is_empty() {
            return Err(BinanceError::new(BinanceErrorKind::Invalid, "周期为空"));
        }
        Ok(Self(s.to_owned()))
    }

    /// 原始字符串。
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for Interval {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// publication 资格。Binance 公开 REST 属当前/近期观测，无官方 vintage 面。
///
/// 恒 [`PitEligibility::NotEligible`]——语义权威见契约 §5。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PitEligibility {
    /// 恒不适用
    NotEligible,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn date_valid() {
        assert!(Date::new(2026, 9, 23).is_ok());
        assert!(Date::new(2024, 2, 29).is_ok());
    }

    #[test]
    fn date_invalid() {
        assert!(Date::new(2026, 13, 1).is_err());
        assert!(Date::new(2026, 0, 1).is_err());
        assert!(Date::new(2026, 2, 30).is_err());
        assert!(Date::new(2023, 2, 29).is_err());
    }

    #[test]
    fn pit_eligibility_always_not_eligible() {
        let p = PitEligibility::NotEligible;
        assert_eq!(p, PitEligibility::NotEligible);
    }
}
