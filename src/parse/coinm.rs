//! COINM 族冻结响应的离线解析入口。
//!
//! 入口直接解析对应的完整响应形状，未知字段使整次解析失败。

#![forbid(unsafe_code)]

use crate::error::BinanceResult;
use crate::parse::{deserialize_strict, validate_unique_response_ids};
use crate::value::coinm::*;

/// 解析 `CoinmExchangeInfo` 的完整响应。
///
/// # Errors
///
/// 未知字段返回 `UnknownField`；非法 JSON 或响应形状返回 `Invalid`。
pub fn parse_coinm_exchange_info(input: &str) -> BinanceResult<CoinmExchangeInfo> {
    crate::parse::deserialize_strict(input)
}

/// 解析 `CoinmAggTrade` 的完整响应。
///
/// # Errors
///
/// 未知字段返回 `UnknownField`；非法 JSON 或响应形状返回 `Invalid`。
/// 本地要求 `a`，同一响应批内缺失、null 或重复分别返回 Missing、SchemaMismatch 或 IdentityConflict。
pub fn parse_coinm_agg_trade(input: &str) -> BinanceResult<CoinmAggTrade> {
    let response: CoinmAggTrade = deserialize_strict(input)?;
    validate_unique_response_ids(
        input,
        response.iter().map(|item| item.a),
        "a",
        "COINM 聚合成交",
    )?;
    Ok(response)
}

/// 解析 `CoinmTrade` 的完整响应。
///
/// # Errors
///
/// 未知字段返回 `UnknownField`；非法 JSON 或响应形状返回 `Invalid`。
pub fn parse_coinm_trade(input: &str) -> BinanceResult<CoinmTrade> {
    crate::parse::deserialize_strict(input)
}

/// 解析 `CoinmContinuousKline` 的完整响应。
///
/// # Errors
///
/// 未知字段返回 `UnknownField`；非法 JSON 或响应形状返回 `Invalid`。
/// 批内开盘时间重复返回 `IdentityConflict`，整批失败。
pub fn parse_coinm_continuous_kline(input: &str) -> BinanceResult<CoinmContinuousKline> {
    parse_coinm_kline(input).map(|rows| CoinmContinuousKline(rows.0))
}

/// 解析 `CoinmKline` 的完整响应。
///
/// # Errors
///
/// 未知字段返回 `UnknownField`；非法 JSON 或响应形状返回 `Invalid`。
/// 批内开盘时间重复返回 `IdentityConflict`，整批失败。
pub fn parse_coinm_kline(input: &str) -> BinanceResult<CoinmKline> {
    let rows: Vec<crate::value::KlineRow> = crate::parse::deserialize_strict(input)?;
    let mut open_times = std::collections::HashSet::new();
    for row in &rows {
        if !open_times.insert(row.0) {
            return Err(crate::error::BinanceError::new(
                crate::error::BinanceErrorKind::IdentityConflict,
                "K 线批次包含重复开盘时间",
            ));
        }
    }
    Ok(CoinmKline(rows))
}

/// 解析 `CoinmIndexPriceKline` 的完整响应。
///
/// # Errors
///
/// 未知字段返回 `UnknownField`；非法 JSON 或响应形状返回 `Invalid`。
/// 批内开盘时间重复返回 `IdentityConflict`，整批失败。
pub fn parse_coinm_index_price_kline(input: &str) -> BinanceResult<CoinmIndexPriceKline> {
    parse_coinm_kline(input).map(|rows| CoinmIndexPriceKline(rows.0))
}

/// 解析 `CoinmMarkPriceKline` 的完整响应。
///
/// # Errors
///
/// 未知字段返回 `UnknownField`；非法 JSON 或响应形状返回 `Invalid`。
/// 批内开盘时间重复返回 `IdentityConflict`，整批失败。
pub fn parse_coinm_mark_price_kline(input: &str) -> BinanceResult<CoinmMarkPriceKline> {
    parse_coinm_kline(input).map(|rows| CoinmMarkPriceKline(rows.0))
}

/// 解析 `CoinmPremiumIndex` 的完整响应。
///
/// # Errors
///
/// 未知字段返回 `UnknownField`；非法 JSON 或响应形状返回 `Invalid`。
pub fn parse_coinm_premium_index(input: &str) -> BinanceResult<CoinmPremiumIndex> {
    crate::parse::deserialize_strict(input)
}

/// 解析 `CoinmPremiumIndexKline` 的完整响应。
///
/// # Errors
///
/// 未知字段返回 `UnknownField`；非法 JSON 或响应形状返回 `Invalid`。
/// 批内开盘时间重复返回 `IdentityConflict`，整批失败。
pub fn parse_coinm_premium_index_kline(input: &str) -> BinanceResult<CoinmPremiumIndexKline> {
    parse_coinm_kline(input).map(|rows| CoinmPremiumIndexKline(rows.0))
}

/// 解析 `CoinmFundingRate` 的完整响应。
///
/// # Errors
///
/// 未知字段返回 `UnknownField`；非法 JSON 或响应形状返回 `Invalid`。
pub fn parse_coinm_funding_rate(input: &str) -> BinanceResult<CoinmFundingRate> {
    crate::parse::deserialize_strict(input)
}

/// 解析 `CoinmFundingInfo` 的完整响应。
///
/// # Errors
///
/// 未知字段返回 `UnknownField`；非法 JSON 或响应形状返回 `Invalid`。
pub fn parse_coinm_funding_info(input: &str) -> BinanceResult<CoinmFundingInfo> {
    crate::parse::deserialize_strict(input)
}

/// 解析 `CoinmOpenInterest` 的完整响应。
///
/// # Errors
///
/// 未知字段返回 `UnknownField`；非法 JSON 或响应形状返回 `Invalid`。
pub fn parse_coinm_open_interest(input: &str) -> BinanceResult<CoinmOpenInterest> {
    crate::parse::deserialize_strict(input)
}

/// 解析 `CoinmOpenInterestHist` 的完整响应。
///
/// # Errors
///
/// 未知字段返回 `UnknownField`；非法 JSON 或响应形状返回 `Invalid`。
pub fn parse_coinm_open_interest_hist(input: &str) -> BinanceResult<CoinmOpenInterestHist> {
    crate::parse::deserialize_strict(input)
}

/// 解析 `CoinmBookTicker` 的完整响应。
///
/// # Errors
///
/// 未知字段返回 `UnknownField`；非法 JSON 或响应形状返回 `Invalid`。
pub fn parse_coinm_book_ticker(input: &str) -> BinanceResult<CoinmBookTicker> {
    crate::parse::deserialize_strict(input)
}

/// 解析 `CoinmBookSnapshot` 的完整响应。
///
/// # Errors
///
/// 未知字段返回 `UnknownField`；非法 JSON 或响应形状返回 `Invalid`。
pub fn parse_coinm_book_snapshot(input: &str) -> BinanceResult<CoinmBookSnapshot> {
    let response = crate::parse::deserialize_strict(input)?;
    crate::parse::require_non_null_field(input, "lastUpdateId")?;
    Ok(response)
}

/// 解析 `CoinmTickerPrice` 的完整响应。
///
/// # Errors
///
/// 未知字段返回 `UnknownField`；非法 JSON 或响应形状返回 `Invalid`。
pub fn parse_coinm_ticker_price(input: &str) -> BinanceResult<CoinmTickerPrice> {
    crate::parse::deserialize_strict(input)
}

/// 解析 `CoinmTicker24hr` 的完整响应。
///
/// # Errors
///
/// 未知字段返回 `UnknownField`；非法 JSON 或响应形状返回 `Invalid`。
pub fn parse_coinm_ticker24hr(input: &str) -> BinanceResult<CoinmTicker24hr> {
    crate::parse::deserialize_strict(input)
}

/// 解析 `CoinmTakerBuySellVol` 的完整响应。
///
/// # Errors
///
/// 未知字段返回 `UnknownField`；非法 JSON 或响应形状返回 `Invalid`。
pub fn parse_coinm_taker_buy_sell_vol(input: &str) -> BinanceResult<CoinmTakerBuySellVol> {
    crate::parse::deserialize_strict(input)
}

/// 解析 `CoinmGlobalLongShortAccountRatio` 的完整响应。
///
/// # Errors
///
/// 未知字段返回 `UnknownField`；非法 JSON 或响应形状返回 `Invalid`。
pub fn parse_coinm_global_long_short_account_ratio(
    input: &str,
) -> BinanceResult<CoinmGlobalLongShortAccountRatio> {
    crate::parse::deserialize_strict(input)
}

/// 解析 `CoinmTopLongShortAccountRatio` 的完整响应。
///
/// # Errors
///
/// 未知字段返回 `UnknownField`；非法 JSON 或响应形状返回 `Invalid`。
pub fn parse_coinm_top_long_short_account_ratio(
    input: &str,
) -> BinanceResult<CoinmTopLongShortAccountRatio> {
    crate::parse::deserialize_strict(input)
}

/// 解析 `CoinmTopLongShortPositionRatio` 的完整响应。
///
/// # Errors
///
/// 未知字段返回 `UnknownField`；非法 JSON 或响应形状返回 `Invalid`。
pub fn parse_coinm_top_long_short_position_ratio(
    input: &str,
) -> BinanceResult<CoinmTopLongShortPositionRatio> {
    crate::parse::deserialize_strict(input)
}

/// 解析 `CoinmBasis` 的完整响应。
///
/// # Errors
///
/// 未知字段返回 `UnknownField`；非法 JSON 或响应形状返回 `Invalid`。
pub fn parse_coinm_basis(input: &str) -> BinanceResult<CoinmBasis> {
    crate::parse::deserialize_strict(input)
}

/// 解析 `CoinmConstituents` 的完整响应。
///
/// # Errors
///
/// 未知字段返回 `UnknownField`；非法 JSON 或响应形状返回 `Invalid`。
pub fn parse_coinm_constituents(input: &str) -> BinanceResult<CoinmConstituents> {
    crate::parse::deserialize_strict(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn kline_batches_require_unique_open_times() {
        // 合成数据：仅验证批内身份，不对应真实行情。
        let first = r#"[1,"1","2","0","1","3",2,"3",1,"1","1","0"]"#;
        let second = r#"[3,"1","2","0","1","3",4,"3",1,"1","1","0"]"#;
        let unique = format!("[{first},{second}]");
        let duplicate = format!("[{first},{second},{first}]");
        type KlineParser = fn(&str) -> BinanceResult<Vec<crate::value::KlineRow>>;
        let parsers: [KlineParser; 5] = [
            |input| parse_coinm_continuous_kline(input).map(|rows| rows.0),
            |input| parse_coinm_kline(input).map(|rows| rows.0),
            |input| parse_coinm_index_price_kline(input).map(|rows| rows.0),
            |input| parse_coinm_mark_price_kline(input).map(|rows| rows.0),
            |input| parse_coinm_premium_index_kline(input).map(|rows| rows.0),
        ];
        for parse in parsers {
            let rows = parse(&unique).expect("不同开盘时间应成功");
            assert_eq!(rows.len(), 2);
            assert_eq!((rows[0].0, rows[1].0), (1, 3));
            assert_eq!(
                parse(&duplicate)
                    .expect_err("重复开盘时间应整批失败")
                    .kind(),
                crate::error::BinanceErrorKind::IdentityConflict,
            );
        }
    }
}
