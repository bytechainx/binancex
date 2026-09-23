//! Spot 族冻结响应的离线解析入口。

#![forbid(unsafe_code)]

use crate::error::{BinanceError, BinanceErrorKind, BinanceResult};
use crate::value::spot::*;
use std::collections::HashSet;

use super::deserialize_strict;

/// 解析 `SpotExchangeInfo` 的冻结响应结构。
///
/// 缺少非必填字段时保留为空；未知字段或非法形态返回错误。
pub fn parse_spot_exchange_info(input: &str) -> BinanceResult<SpotExchangeInfo> {
    deserialize_strict(input)
}

/// 解析 `SpotExecutionRules` 的冻结响应结构。
///
/// 缺少非必填字段时保留为空；未知字段或非法形态返回错误。
pub fn parse_spot_execution_rules(input: &str) -> BinanceResult<SpotExecutionRules> {
    deserialize_strict(input)
}

/// 解析 `SpotAggTrade` 的冻结响应结构。
///
/// 缺少非必填字段时保留为空；未知字段或非法形态返回错误。
pub fn parse_spot_agg_trade(input: &str) -> BinanceResult<SpotAggTrade> {
    deserialize_strict(input)
}

/// 解析 `SpotAvgPrice` 的冻结响应结构。
///
/// P6 解析策略要求 `price` 与 `closeTime`：缺键返回 `Missing`，
/// 显式空值或错误类型返回 `Invalid`；此策略不改写源合同的必填证据。
pub fn parse_spot_avg_price(input: &str) -> BinanceResult<SpotAvgPrice> {
    let raw: serde_json::Value = deserialize_strict(input)?;
    let object = raw.as_object().ok_or_else(|| {
        BinanceError::new(BinanceErrorKind::SchemaMismatch, "平均价格响应必须是对象")
    })?;
    for key in ["price", "closeTime"] {
        let value = object.get(key).ok_or_else(|| {
            BinanceError::new(BinanceErrorKind::Missing, format!("平均价格响应缺少 {key}"))
        })?;
        let valid = if key == "price" {
            value.is_string()
        } else {
            value
                .as_number()
                .is_some_and(|number| !number.to_string().contains(['.', 'e', 'E']))
        };
        if !valid {
            return Err(BinanceError::new(
                BinanceErrorKind::Invalid,
                format!("平均价格响应的 {key} 不能为空且必须符合字段类型"),
            ));
        }
    }
    deserialize_strict(input)
}

/// 解析 `SpotTrade` 的冻结响应结构。
///
/// 缺少非必填字段时保留为空；未知字段或非法形态返回错误。
pub fn parse_spot_trade(input: &str) -> BinanceResult<SpotTrade> {
    deserialize_strict(input)
}

/// 解析 `SpotBlockTrade` 的冻结响应结构。
///
/// 缺少非必填字段时保留为空；未知字段或非法形态返回错误。
pub fn parse_spot_block_trade(input: &str) -> BinanceResult<SpotBlockTrade> {
    deserialize_strict(input)
}

/// 解析 `SpotKline` 的冻结响应结构。
///
/// 开盘时间为批内身份键；重复时返回 `IdentityConflict`，整批原子失败。
pub fn parse_spot_kline(input: &str) -> BinanceResult<SpotKline> {
    let rows: SpotKline = deserialize_strict(input)?;
    let mut open_times = HashSet::new();
    if rows.iter().any(|row| !open_times.insert(row.0)) {
        return Err(BinanceError::new(
            BinanceErrorKind::IdentityConflict,
            "Spot Kline 批内开盘时间重复",
        ));
    }
    Ok(rows)
}

/// 解析 `SpotUiKline` 的冻结响应结构。
///
/// 缺少非必填字段时保留为空；未知字段或非法形态返回错误。
pub fn parse_spot_ui_kline(input: &str) -> BinanceResult<SpotUiKline> {
    deserialize_strict(input)
}

/// 解析 `SpotTicker` 的冻结响应结构。
///
/// 缺少非必填字段时保留为空；未知字段或非法形态返回错误。
pub fn parse_spot_ticker(input: &str) -> BinanceResult<SpotTicker> {
    match input.trim_start().as_bytes().first() {
        Some(b'{') => deserialize_strict(input).map(SpotTicker::Object),
        Some(b'[') => deserialize_strict(input).map(SpotTicker::Array),
        _ => deserialize_strict(input),
    }
}

/// 解析 `SpotTicker24hr` 的冻结响应结构。
///
/// 缺少非必填字段时保留为空；未知字段或非法形态返回错误。
pub fn parse_spot_ticker24hr(input: &str) -> BinanceResult<SpotTicker24hr> {
    match input.trim_start().as_bytes().first() {
        Some(b'{') => deserialize_strict(input).map(SpotTicker24hr::Object),
        Some(b'[') => deserialize_strict(input).map(SpotTicker24hr::Array),
        _ => deserialize_strict(input),
    }
}

/// 解析 `SpotTickerPrice` 的冻结响应结构。
///
/// 缺少非必填字段时保留为空；未知字段或非法形态返回错误。
pub fn parse_spot_ticker_price(input: &str) -> BinanceResult<SpotTickerPrice> {
    match input.trim_start().as_bytes().first() {
        Some(b'{') => deserialize_strict(input).map(SpotTickerPrice::Object),
        Some(b'[') => deserialize_strict(input).map(SpotTickerPrice::Array),
        _ => deserialize_strict(input),
    }
}

/// 解析 `SpotBookTicker` 的冻结响应结构。
///
/// 缺少非必填字段时保留为空；未知字段或非法形态返回错误。
pub fn parse_spot_book_ticker(input: &str) -> BinanceResult<SpotBookTicker> {
    match input.trim_start().as_bytes().first() {
        Some(b'{') => deserialize_strict(input).map(SpotBookTicker::Object),
        Some(b'[') => deserialize_strict(input).map(SpotBookTicker::Array),
        _ => deserialize_strict(input),
    }
}

/// 解析 `SpotTradingDay` 的冻结响应结构。
///
/// 缺少非必填字段时保留为空；未知字段或非法形态返回错误。
pub fn parse_spot_trading_day(input: &str) -> BinanceResult<SpotTradingDay> {
    match input.trim_start().as_bytes().first() {
        Some(b'{') => deserialize_strict(input).map(SpotTradingDay::Object),
        Some(b'[') => deserialize_strict(input).map(SpotTradingDay::Array),
        _ => deserialize_strict(input),
    }
}

/// 解析 `SpotReferencePrice` 的冻结响应结构。
///
/// 缺少非必填字段时保留为空；未知字段或非法形态返回错误。
pub fn parse_spot_reference_price(input: &str) -> BinanceResult<SpotReferencePrice> {
    deserialize_strict(input)
}

/// 解析 `SpotReferencePriceCalculation` 的冻结响应结构。
///
/// 缺少非必填字段时保留为空；未知字段或非法形态返回错误。
pub fn parse_spot_reference_price_calculation(
    input: &str,
) -> BinanceResult<SpotReferencePriceCalculation> {
    deserialize_strict(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spot_avg_price_requires_price_and_close_time() {
        // 合成样本：语义锚点齐全时解析成功，其他字段保持可选。
        let parsed =
            parse_spot_avg_price(r#"{"price":"-0.0000","closeTime":9007199254740993}"#).unwrap();
        assert_eq!(parsed.price.as_deref(), Some("-0.0000"));
        assert_eq!(parsed.close_time, Some(9_007_199_254_740_993));
        assert_eq!(parsed.mins, None);
        for input in [r#"{"closeTime":1}"#, r#"{"price":"1"}"#, "{}"] {
            assert_eq!(
                parse_spot_avg_price(input).unwrap_err().kind(),
                BinanceErrorKind::Missing
            );
        }
        for input in [
            r#"{"price":null,"closeTime":1}"#,
            r#"{"price":"1","closeTime":null}"#,
            r#"{"price":1,"closeTime":1}"#,
            r#"{"price":"1","closeTime":"1"}"#,
            r#"{"price":"1","closeTime":1.5}"#,
        ] {
            assert_eq!(
                parse_spot_avg_price(input).unwrap_err().kind(),
                BinanceErrorKind::Invalid
            );
        }
        for input in [r#"{"symbol":"SYNTHETIC"}"#, r#"{"referencePrice":null}"#] {
            assert!(parse_spot_reference_price(input)
                .unwrap()
                .reference_price
                .is_none());
        }
    }

    #[test]
    fn spot_kline_rejects_duplicate_open_times() {
        // 合成样本：非真实源数据；非相邻重复且价格不同仍属同一身份。
        let input = r#"[
            [1,"1","1","1","1","1",2,"1",1,"1","1","0"],
            [3,"1","1","1","1","1",4,"1",1,"1","1","0"],
            [1,"2","2","2","2","2",2,"2",2,"2","2","0"]
        ]"#;
        let error = parse_spot_kline(input).unwrap_err();
        assert_eq!(error.kind(), BinanceErrorKind::IdentityConflict);
    }

    #[test]
    fn spot_kline_preserves_distinct_rows_and_empty_batch() {
        // 合成样本：不同开盘时间按输入顺序保留。
        let input = r#"[
            [3,"1","1","1","1","1",4,"1",1,"1","1","0"],
            [1,"1","1","1","1","1",2,"1",1,"1","1","0"]
        ]"#;
        let rows = parse_spot_kline(input).unwrap();
        assert_eq!(rows.iter().map(|row| row.0).collect::<Vec<_>>(), [3, 1]);
        assert!(parse_spot_kline("[]").unwrap().is_empty());
    }
}
