//! Options 族离线严格解析入口。

use std::collections::HashSet;

use crate::error::{BinanceError, BinanceErrorKind, BinanceResult};
use crate::value::options::*;

/// 解析 `OptionsExchangeInfo` 冻结响应。
///
/// # Errors
///
/// 未知字段返回 `UnknownField`；JSON、字段类型或根形态错误返回 `Invalid`。
pub fn parse_options_exchange_info(input: &str) -> BinanceResult<OptionsExchangeInfo> {
    crate::parse::deserialize_strict(input)
}

/// 解析 `OptionsExerciseHistory` 冻结响应。
///
/// # Errors
///
/// 未知字段返回 `UnknownField`；JSON、字段类型或根形态错误返回 `Invalid`。
pub fn parse_options_exercise_history(input: &str) -> BinanceResult<OptionsExerciseHistory> {
    crate::parse::deserialize_strict(input)
}

/// 解析 `OptionsIndex` 冻结响应。
///
/// # Errors
///
/// 未知字段返回 `UnknownField`；JSON、字段类型或根形态错误返回 `Invalid`。
pub fn parse_options_index(input: &str) -> BinanceResult<OptionsIndex> {
    crate::parse::deserialize_strict(input)
}

/// 解析 `OptionsKline` 冻结响应。
///
/// # Errors
///
/// 开盘时间重复返回 `IdentityConflict`，整批原子失败；
/// JSON、字段类型或根形态错误返回 `Invalid`。
pub fn parse_options_kline(input: &str) -> BinanceResult<OptionsKline> {
    let rows: Vec<crate::value::KlineRow> = crate::parse::deserialize_strict(input)?;
    let mut open_times = HashSet::new();
    if rows.iter().any(|row| !open_times.insert(row.0)) {
        return Err(BinanceError::new(
            BinanceErrorKind::IdentityConflict,
            "Options Kline 批内开盘时间重复",
        ));
    }
    Ok(OptionsKline(rows))
}

/// 解析 `OptionsOpenInterest` 冻结响应。
///
/// # Errors
///
/// 未知字段返回 `UnknownField`；JSON、字段类型或根形态错误返回 `Invalid`。
pub fn parse_options_open_interest(input: &str) -> BinanceResult<OptionsOpenInterest> {
    crate::parse::deserialize_strict(input)
}

/// 解析 `OptionsMark` 冻结响应。
///
/// # Errors
///
/// 未知字段返回 `UnknownField`；JSON、字段类型或根形态错误返回 `Invalid`。
pub fn parse_options_mark(input: &str) -> BinanceResult<OptionsMark> {
    crate::parse::deserialize_strict(input)
}

/// 解析 `OptionsBlockTrade` 冻结响应。
///
/// # Errors
///
/// 未知字段返回 `UnknownField`；JSON、字段类型或根形态错误返回 `Invalid`。
pub fn parse_options_block_trade(input: &str) -> BinanceResult<OptionsBlockTrade> {
    crate::parse::deserialize_strict(input)
}

/// 解析 `OptionsTrade` 冻结响应。
///
/// # Errors
///
/// 未知字段返回 `UnknownField`；JSON、字段类型或根形态错误返回 `Invalid`。
pub fn parse_options_trade(input: &str) -> BinanceResult<OptionsTrade> {
    crate::parse::deserialize_strict(input)
}

/// 解析 `OptionsTicker` 冻结响应。
///
/// # Errors
///
/// 未知字段返回 `UnknownField`；JSON、字段类型或根形态错误返回 `Invalid`。
pub fn parse_options_ticker(input: &str) -> BinanceResult<OptionsTicker> {
    crate::parse::deserialize_strict(input)
}

/// 离线解析 `OptionsBookSnapshot` 的冻结深度快照结构。
///
/// # Errors
///
/// 未知字段返回 `UnknownField`；非法 JSON 或响应形状返回 `Invalid`。
pub fn parse_options_book_snapshot(input: &str) -> BinanceResult<OptionsBookSnapshot> {
    crate::parse::deserialize_strict(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn kline_rejects_duplicate_open_times() {
        // 合成样本：相同开盘时间、不同价格也构成同一批内身份冲突。
        let input = r#"[
            [1,"10","11","9","10","2",2,"20",3,"1","10","0"],
            [1,"20","21","19","20","2",2,"40",3,"1","20","0"]
        ]"#;
        assert_eq!(
            parse_options_kline(input).unwrap_err().kind(),
            BinanceErrorKind::IdentityConflict
        );
    }

    #[test]
    fn kline_preserves_distinct_open_times() {
        // 合成样本：不同开盘时间保持输入顺序，不删除或重排记录。
        let input = r#"[
            [2,"10","11","9","10","2",3,"20",3,"1","10","0"],
            [1,"10","11","9","10","2",2,"20",3,"1","10","0"]
        ]"#;
        let rows = parse_options_kline(input).unwrap();
        assert_eq!(rows.len(), 2);
        assert_eq!((rows[0].0, rows[1].0), (2, 1));
    }
}
