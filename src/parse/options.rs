//! Options 族离线严格解析入口。

use std::collections::HashSet;

use crate::error::{BinanceError, BinanceErrorKind, BinanceResult};
use crate::parse::{deserialize_strict, validate_unique_nested_string_ids};
use crate::value::options::*;

fn validate_unique_symbol_scoped_ids(
    input: &str,
    id_field: &str,
    label: &str,
) -> BinanceResult<()> {
    let raw: serde_json::Value = deserialize_strict(input)?;
    let rows = raw.as_array().ok_or_else(|| {
        BinanceError::new(BinanceErrorKind::SchemaMismatch, "期权成交响应必须是数组")
    })?;
    let mut identities = HashSet::new();
    for row in rows {
        let object = row.as_object().ok_or_else(|| {
            BinanceError::new(BinanceErrorKind::SchemaMismatch, "期权成交项必须是对象")
        })?;
        let symbol = match object.get("symbol") {
            None => {
                return Err(BinanceError::new(
                    BinanceErrorKind::Missing,
                    format!("{label} 缺少本地身份字段 symbol"),
                ));
            }
            Some(serde_json::Value::String(symbol)) if !symbol.trim().is_empty() => symbol,
            Some(serde_json::Value::Null) => {
                return Err(BinanceError::new(
                    BinanceErrorKind::SchemaMismatch,
                    format!("{label} 本地身份字段 symbol 不得为 null"),
                ));
            }
            Some(_) => {
                return Err(BinanceError::new(
                    BinanceErrorKind::SchemaMismatch,
                    format!("{label} 本地身份字段 symbol 必须为非空字符串"),
                ));
            }
        };
        let id = match object.get(id_field) {
            None => {
                return Err(BinanceError::new(
                    BinanceErrorKind::Missing,
                    format!("{label} 缺少本地身份字段 {id_field}"),
                ));
            }
            Some(serde_json::Value::Number(id)) => id.as_i64().ok_or_else(|| {
                BinanceError::new(
                    BinanceErrorKind::SchemaMismatch,
                    format!("{label} 本地身份字段 {id_field} 必须为 int64"),
                )
            })?,
            Some(serde_json::Value::Null) => {
                return Err(BinanceError::new(
                    BinanceErrorKind::SchemaMismatch,
                    format!("{label} 本地身份字段 {id_field} 不得为 null"),
                ));
            }
            Some(_) => {
                return Err(BinanceError::new(
                    BinanceErrorKind::SchemaMismatch,
                    format!("{label} 本地身份字段 {id_field} 必须为 int64"),
                ));
            }
        };
        if !identities.insert((symbol, id)) {
            return Err(BinanceError::new(
                BinanceErrorKind::IdentityConflict,
                format!("{label} 同一 symbol 内存在重复 {id_field}"),
            ));
        }
    }
    Ok(())
}

/// 解析 `OptionsExchangeInfo` 冻结响应。
///
/// # Errors
///
/// 未知字段返回 `UnknownField`；JSON、字段类型或根形态错误返回 `Invalid`。
pub fn parse_options_exchange_info(input: &str) -> BinanceResult<OptionsExchangeInfo> {
    let response: OptionsExchangeInfo = deserialize_strict(input)?;
    validate_unique_nested_string_ids(
        input,
        "optionSymbols",
        response
            .option_symbols
            .iter()
            .flatten()
            .map(|item| item.symbol.clone()),
        "symbol",
        "Options exchangeInfo 标的",
    )?;
    Ok(response)
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
/// 本地身份字段 `symbol+id` 缺失返回 `Missing`，null 或空 symbol 返回
/// `SchemaMismatch`；同一响应批内重复组合返回 `IdentityConflict`。此规则不表示源方保证唯一性。
/// 未知字段返回 `UnknownField`；JSON、字段类型或根形态错误返回 `Invalid`。
pub fn parse_options_block_trade(input: &str) -> BinanceResult<OptionsBlockTrade> {
    let response: OptionsBlockTrade = crate::parse::deserialize_strict(input)?;
    validate_unique_symbol_scoped_ids(input, "id", "期权大宗成交")?;
    Ok(response)
}

/// 解析 `OptionsTrade` 冻结响应。
///
/// # Errors
///
/// 本地身份字段 `symbol+tradeId` 缺失返回 `Missing`，null 或空 symbol 返回
/// `SchemaMismatch`；同一响应批内重复组合返回 `IdentityConflict`。此规则不表示源方保证唯一性。
/// 未知字段返回 `UnknownField`；JSON、字段类型或根形态错误返回 `Invalid`。
pub fn parse_options_trade(input: &str) -> BinanceResult<OptionsTrade> {
    let response: OptionsTrade = crate::parse::deserialize_strict(input)?;
    validate_unique_symbol_scoped_ids(input, "tradeId", "期权成交")?;
    Ok(response)
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
/// 未知字段返回 `UnknownField`；非法 JSON 返回 `Invalid`；类型或响应形状错误返回
/// `SchemaMismatch`；缺少 `lastUpdateId` 返回 `Missing`；游标为 `null` 或无法无损表示时
/// 返回 `SchemaMismatch` 或 `LossyNumeric`。
pub fn parse_options_book_snapshot(input: &str) -> BinanceResult<OptionsBookSnapshot> {
    let response = crate::parse::deserialize_strict(input)?;
    crate::parse::require_non_null_field(input, "lastUpdateId")?;
    Ok(response)
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
