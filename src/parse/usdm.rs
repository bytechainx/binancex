//! USDM 冻结响应结构的离线解析入口。
//!
//! forms 入口先确定 JSON 根形态，再反序列化具体结构，以保留未知字段错误类别。

use super::deserialize_strict;
use crate::error::{BinanceError, BinanceErrorKind, BinanceResult};
use crate::value::usdm::*;

/// 离线解析 UsdmExchangeInfo 的冻结响应结构。
///
/// # Errors
///
/// 未知字段返回 UnknownField；非法 JSON 返回 Invalid。
/// 响应结构或字段类型不符返回 SchemaMismatch；数值无法无损承载返回 LossyNumeric。
pub fn parse_usdm_exchange_info(input: &str) -> BinanceResult<UsdmExchangeInfo> {
    deserialize_strict(input)
}

/// 离线解析 UsdmIndexInfo 的冻结响应结构。
///
/// # Errors
///
/// 未知字段返回 UnknownField；非法 JSON 返回 Invalid。
/// 响应结构或字段类型不符返回 SchemaMismatch；数值无法无损承载返回 LossyNumeric。
pub fn parse_usdm_index_info(input: &str) -> BinanceResult<UsdmIndexInfo> {
    deserialize_strict(input)
}

/// 离线解析 UsdmAggTrade 的冻结响应结构。
///
/// # Errors
///
/// 未知字段返回 UnknownField；非法 JSON 返回 Invalid。
/// 响应结构或字段类型不符返回 SchemaMismatch；数值无法无损承载返回 LossyNumeric。
pub fn parse_usdm_agg_trade(input: &str) -> BinanceResult<UsdmAggTrade> {
    deserialize_strict(input)
}

/// 离线解析 UsdmTrade 的冻结响应结构。
///
/// # Errors
///
/// 未知字段返回 UnknownField；非法 JSON 返回 Invalid。
/// 响应结构或字段类型不符返回 SchemaMismatch；数值无法无损承载返回 LossyNumeric。
pub fn parse_usdm_trade(input: &str) -> BinanceResult<UsdmTrade> {
    deserialize_strict(input)
}

/// 离线解析 UsdmContinuousKline 的冻结响应结构。
///
/// # Errors
///
/// 未知字段返回 UnknownField；非法 JSON 返回 Invalid。
/// 响应结构或字段类型不符返回 SchemaMismatch；数值无法无损承载返回 LossyNumeric。
/// 批内开盘时间重复返回 IdentityConflict，整批失败。
pub fn parse_usdm_continuous_kline(input: &str) -> BinanceResult<UsdmContinuousKline> {
    parse_kline_rows(input).map(UsdmContinuousKline)
}

/// 离线解析 UsdmKline 的冻结响应结构。
///
/// # Errors
///
/// 未知字段返回 UnknownField；非法 JSON 返回 Invalid。
/// 响应结构或字段类型不符返回 SchemaMismatch；数值无法无损承载返回 LossyNumeric。
/// 批内开盘时间重复返回 IdentityConflict，整批失败。
pub fn parse_usdm_kline(input: &str) -> BinanceResult<UsdmKline> {
    parse_kline_rows(input).map(UsdmKline)
}

/// 离线解析 UsdmIndexPriceKline 的冻结响应结构。
///
/// # Errors
///
/// 未知字段返回 UnknownField；非法 JSON 返回 Invalid。
/// 响应结构或字段类型不符返回 SchemaMismatch；数值无法无损承载返回 LossyNumeric。
/// 批内开盘时间重复返回 IdentityConflict，整批失败。
pub fn parse_usdm_index_price_kline(input: &str) -> BinanceResult<UsdmIndexPriceKline> {
    parse_kline_rows(input).map(UsdmIndexPriceKline)
}

/// 离线解析 UsdmMarkPriceKline 的冻结响应结构。
///
/// # Errors
///
/// 未知字段返回 UnknownField；非法 JSON 返回 Invalid。
/// 响应结构或字段类型不符返回 SchemaMismatch；数值无法无损承载返回 LossyNumeric。
/// 批内开盘时间重复返回 IdentityConflict，整批失败。
pub fn parse_usdm_mark_price_kline(input: &str) -> BinanceResult<UsdmMarkPriceKline> {
    parse_kline_rows(input).map(UsdmMarkPriceKline)
}

/// 离线解析 UsdmPremiumIndex 的冻结响应结构。
///
/// # Errors
///
/// 未知字段返回 UnknownField；非法 JSON 返回 Invalid。
/// 响应结构或字段类型不符返回 SchemaMismatch；数值无法无损承载返回 LossyNumeric。
/// 根形态不是对象或数组时返回 Invalid。
pub fn parse_usdm_premium_index(input: &str) -> BinanceResult<UsdmPremiumIndex> {
    match input.bytes().find(|byte| !byte.is_ascii_whitespace()) {
        Some(b'{') => deserialize_strict(input).map(UsdmPremiumIndex::Object),
        Some(b'[') => deserialize_strict(input).map(UsdmPremiumIndex::Array),
        _ => Err(BinanceError::new(
            BinanceErrorKind::Invalid,
            "响应根必须为对象或数组",
        )),
    }
}

/// 离线解析 UsdmPremiumIndexKline 的冻结响应结构。
///
/// # Errors
///
/// 未知字段返回 UnknownField；非法 JSON 返回 Invalid。
/// 响应结构或字段类型不符返回 SchemaMismatch；数值无法无损承载返回 LossyNumeric。
/// 批内开盘时间重复返回 IdentityConflict，整批失败。
pub fn parse_usdm_premium_index_kline(input: &str) -> BinanceResult<UsdmPremiumIndexKline> {
    parse_kline_rows(input).map(UsdmPremiumIndexKline)
}

/// 离线解析 UsdmFundingRate 的冻结响应结构。
///
/// # Errors
///
/// 未知字段返回 UnknownField；非法 JSON 返回 Invalid。
/// 响应结构或字段类型不符返回 SchemaMismatch；数值无法无损承载返回 LossyNumeric。
pub fn parse_usdm_funding_rate(input: &str) -> BinanceResult<UsdmFundingRate> {
    deserialize_strict(input)
}

/// 离线解析 UsdmFundingInfo 的冻结响应结构。
///
/// # Errors
///
/// 未知字段返回 UnknownField；非法 JSON 返回 Invalid。
/// 响应结构或字段类型不符返回 SchemaMismatch；数值无法无损承载返回 LossyNumeric。
pub fn parse_usdm_funding_info(input: &str) -> BinanceResult<UsdmFundingInfo> {
    deserialize_strict(input)
}

/// 离线解析 UsdmOpenInterest 的冻结响应结构。
///
/// # Errors
///
/// 未知字段返回 UnknownField；非法 JSON 返回 Invalid。
/// 响应结构或字段类型不符返回 SchemaMismatch；数值无法无损承载返回 LossyNumeric。
pub fn parse_usdm_open_interest(input: &str) -> BinanceResult<UsdmOpenInterest> {
    deserialize_strict(input)
}

/// 离线解析 UsdmOpenInterestHist 的冻结响应结构。
///
/// # Errors
///
/// 未知字段返回 UnknownField；非法 JSON 返回 Invalid。
/// 响应结构或字段类型不符返回 SchemaMismatch；数值无法无损承载返回 LossyNumeric。
pub fn parse_usdm_open_interest_hist(input: &str) -> BinanceResult<UsdmOpenInterestHist> {
    deserialize_strict(input)
}

/// 离线解析 UsdmBookSnapshot 的冻结响应结构。
///
/// # Errors
///
/// 未知字段返回 UnknownField；非法 JSON 返回 Invalid。
/// 响应结构或字段类型不符返回 SchemaMismatch；数值无法无损承载返回 LossyNumeric。
pub fn parse_usdm_book_snapshot(input: &str) -> BinanceResult<UsdmBookSnapshot> {
    let response = deserialize_strict(input)?;
    super::require_non_null_field(input, "lastUpdateId")?;
    Ok(response)
}

/// 离线解析 UsdmBookTicker 的冻结响应结构。
///
/// # Errors
///
/// 未知字段返回 UnknownField；非法 JSON 返回 Invalid。
/// 响应结构或字段类型不符返回 SchemaMismatch；数值无法无损承载返回 LossyNumeric。
/// 根形态不是对象或数组时返回 Invalid。
pub fn parse_usdm_book_ticker(input: &str) -> BinanceResult<UsdmBookTicker> {
    match input.bytes().find(|byte| !byte.is_ascii_whitespace()) {
        Some(b'{') => deserialize_strict(input).map(UsdmBookTicker::Object),
        Some(b'[') => deserialize_strict(input).map(UsdmBookTicker::Array),
        _ => Err(BinanceError::new(
            BinanceErrorKind::Invalid,
            "响应根必须为对象或数组",
        )),
    }
}

/// 离线解析 UsdmTickerPrice 的冻结响应结构。
///
/// # Errors
///
/// 未知字段返回 UnknownField；非法 JSON 返回 Invalid。
/// 响应结构或字段类型不符返回 SchemaMismatch；数值无法无损承载返回 LossyNumeric。
/// 根形态不是对象或数组时返回 Invalid。
pub fn parse_usdm_ticker_price(input: &str) -> BinanceResult<UsdmTickerPrice> {
    match input.bytes().find(|byte| !byte.is_ascii_whitespace()) {
        Some(b'{') => deserialize_strict(input).map(UsdmTickerPrice::Object),
        Some(b'[') => deserialize_strict(input).map(UsdmTickerPrice::Array),
        _ => Err(BinanceError::new(
            BinanceErrorKind::Invalid,
            "响应根必须为对象或数组",
        )),
    }
}

/// 离线解析 UsdmTicker24hr 的冻结响应结构。
///
/// # Errors
///
/// 未知字段返回 UnknownField；非法 JSON 返回 Invalid。
/// 响应结构或字段类型不符返回 SchemaMismatch；数值无法无损承载返回 LossyNumeric。
/// 根形态不是对象或数组时返回 Invalid。
pub fn parse_usdm_ticker24hr(input: &str) -> BinanceResult<UsdmTicker24hr> {
    match input.bytes().find(|byte| !byte.is_ascii_whitespace()) {
        Some(b'{') => deserialize_strict(input).map(UsdmTicker24hr::Object),
        Some(b'[') => deserialize_strict(input).map(UsdmTicker24hr::Array),
        _ => Err(BinanceError::new(
            BinanceErrorKind::Invalid,
            "响应根必须为对象或数组",
        )),
    }
}

/// 离线解析 UsdmTakerLongShortRatio 的冻结响应结构。
///
/// # Errors
///
/// 未知字段返回 UnknownField；非法 JSON 返回 Invalid。
/// 响应结构或字段类型不符返回 SchemaMismatch；数值无法无损承载返回 LossyNumeric。
pub fn parse_usdm_taker_long_short_ratio(input: &str) -> BinanceResult<UsdmTakerLongShortRatio> {
    deserialize_strict(input)
}

/// 离线解析 UsdmGlobalLongShortAccountRatio 的冻结响应结构。
///
/// # Errors
///
/// 未知字段返回 UnknownField；非法 JSON 返回 Invalid。
/// 响应结构或字段类型不符返回 SchemaMismatch；数值无法无损承载返回 LossyNumeric。
pub fn parse_usdm_global_long_short_account_ratio(
    input: &str,
) -> BinanceResult<UsdmGlobalLongShortAccountRatio> {
    deserialize_strict(input)
}

/// 离线解析 UsdmTopLongShortAccountRatio 的冻结响应结构。
///
/// # Errors
///
/// 未知字段返回 UnknownField；非法 JSON 返回 Invalid。
/// 响应结构或字段类型不符返回 SchemaMismatch；数值无法无损承载返回 LossyNumeric。
pub fn parse_usdm_top_long_short_account_ratio(
    input: &str,
) -> BinanceResult<UsdmTopLongShortAccountRatio> {
    deserialize_strict(input)
}

/// 离线解析 UsdmTopLongShortPositionRatio 的冻结响应结构。
///
/// # Errors
///
/// 未知字段返回 UnknownField；非法 JSON 返回 Invalid。
/// 响应结构或字段类型不符返回 SchemaMismatch；数值无法无损承载返回 LossyNumeric。
pub fn parse_usdm_top_long_short_position_ratio(
    input: &str,
) -> BinanceResult<UsdmTopLongShortPositionRatio> {
    deserialize_strict(input)
}

/// 离线解析 UsdmAssetIndex 的冻结响应结构。
///
/// # Errors
///
/// 未知字段返回 UnknownField；非法 JSON 返回 Invalid。
/// 响应结构或字段类型不符返回 SchemaMismatch；数值无法无损承载返回 LossyNumeric。
/// 根形态不是对象或数组时返回 Invalid。
pub fn parse_usdm_asset_index(input: &str) -> BinanceResult<UsdmAssetIndex> {
    match input.bytes().find(|byte| !byte.is_ascii_whitespace()) {
        Some(b'{') => deserialize_strict(input).map(UsdmAssetIndex::Object),
        Some(b'[') => deserialize_strict(input).map(UsdmAssetIndex::Array),
        _ => Err(BinanceError::new(
            BinanceErrorKind::Invalid,
            "响应根必须为对象或数组",
        )),
    }
}

/// 离线解析 UsdmConstituents 的冻结响应结构。
///
/// # Errors
///
/// 未知字段返回 UnknownField；非法 JSON 返回 Invalid。
/// 响应结构或字段类型不符返回 SchemaMismatch；数值无法无损承载返回 LossyNumeric。
pub fn parse_usdm_constituents(input: &str) -> BinanceResult<UsdmConstituents> {
    deserialize_strict(input)
}

/// 离线解析 UsdmInsuranceBalance 的冻结响应结构。
///
/// # Errors
///
/// 未知字段返回 UnknownField；非法 JSON 返回 Invalid。
/// 响应结构或字段类型不符返回 SchemaMismatch；数值无法无损承载返回 LossyNumeric。
/// 根形态不是对象或数组时返回 Invalid。
pub fn parse_usdm_insurance_balance(input: &str) -> BinanceResult<UsdmInsuranceBalance> {
    match input.bytes().find(|byte| !byte.is_ascii_whitespace()) {
        Some(b'{') => deserialize_strict(input).map(UsdmInsuranceBalance::Object),
        Some(b'[') => deserialize_strict(input).map(UsdmInsuranceBalance::Array),
        _ => Err(BinanceError::new(
            BinanceErrorKind::Invalid,
            "响应根必须为对象或数组",
        )),
    }
}

/// 离线解析 UsdmBasis 的冻结响应结构。
///
/// # Errors
///
/// 未知字段返回 UnknownField；非法 JSON 返回 Invalid。
/// 响应结构或字段类型不符返回 SchemaMismatch；数值无法无损承载返回 LossyNumeric。
pub fn parse_usdm_basis(input: &str) -> BinanceResult<UsdmBasis> {
    deserialize_strict(input)
}

/// 离线解析 UsdmDeliveryPrice 的冻结响应结构。
///
/// # Errors
///
/// 未知字段返回 UnknownField；非法 JSON 返回 Invalid。
/// 响应结构或字段类型不符返回 SchemaMismatch；数值无法无损承载返回 LossyNumeric。
/// 伪造为数字的嵌套对象返回 SchemaMismatch。
pub fn parse_usdm_delivery_price(input: &str) -> BinanceResult<UsdmDeliveryPrice> {
    let rows = deserialize_strict(input)?;
    validate_delivery_price_wire(input)?;
    Ok(rows)
}

// 本入口每行只有标量字段；严格解析通过后仍需拒绝伪装成 Number 的嵌套对象。
// 直接检查 wire 层级，避免 arbitrary_precision 将保留键对象归一化为数字。
fn validate_delivery_price_wire(input: &str) -> BinanceResult<()> {
    let mut in_string = false;
    let mut escaped = false;
    let mut in_object = false;
    for byte in input.bytes() {
        if in_string {
            if escaped {
                escaped = false;
            } else if byte == b'\\' {
                escaped = true;
            } else if byte == b'"' {
                in_string = false;
            }
            continue;
        }
        match byte {
            b'"' => in_string = true,
            b'{' if in_object => {
                return Err(BinanceError::new(
                    BinanceErrorKind::SchemaMismatch,
                    "交割价字段必须为原始 JSON 数字，不能为对象",
                ));
            }
            b'{' => in_object = true,
            b'}' => in_object = false,
            _ => {}
        }
    }
    Ok(())
}

/// 离线解析 UsdmTradingSchedule 的冻结响应结构。
///
/// 仅接受冻结合同已观测的市场键；新键须补齐证据后再放行。
///
/// # Errors
///
/// 未知字段或市场键返回 UnknownField；非法 JSON 返回 Invalid。
/// 响应结构或字段类型不符返回 SchemaMismatch；数值无法无损承载返回 LossyNumeric。
pub fn parse_usdm_trading_schedule(input: &str) -> BinanceResult<UsdmTradingSchedule> {
    let schedule: UsdmTradingSchedule = deserialize_strict(input)?;
    if schedule.market_schedules.as_ref().is_some_and(|markets| {
        markets.keys().any(|key| {
            !matches!(
                key.as_str(),
                "EQUITY" | "COMMODITY" | "KR_EQUITY" | "HK_EQUITY" | "CN_EQUITY" | "FX"
            )
        })
    }) {
        return Err(BinanceError::new(
            BinanceErrorKind::UnknownField,
            "marketSchedules 含尚未获得证据覆盖的市场键",
        ));
    }
    Ok(schedule)
}

/// 离线解析 UsdmConvertExchangeInfo 的冻结响应结构。
///
/// # Errors
///
/// 未知字段返回 UnknownField；非法 JSON 返回 Invalid。
/// 响应结构或字段类型不符返回 SchemaMismatch；数值无法无损承载返回 LossyNumeric。
pub fn parse_usdm_convert_exchange_info(input: &str) -> BinanceResult<UsdmConvertExchangeInfo> {
    deserialize_strict(input)
}

/// 离线解析 UsdmAdlRisk 的冻结响应结构。
///
/// # Errors
///
/// 未知字段返回 UnknownField；非法 JSON 返回 Invalid。
/// 响应结构或字段类型不符返回 SchemaMismatch；数值无法无损承载返回 LossyNumeric。
/// 根形态不是对象或数组时返回 Invalid。
pub fn parse_usdm_adl_risk(input: &str) -> BinanceResult<UsdmAdlRisk> {
    match input.bytes().find(|byte| !byte.is_ascii_whitespace()) {
        Some(b'{') => deserialize_strict(input).map(UsdmAdlRisk::Object),
        Some(b'[') => deserialize_strict(input).map(UsdmAdlRisk::Array),
        _ => Err(BinanceError::new(
            BinanceErrorKind::Invalid,
            "响应根必须为对象或数组",
        )),
    }
}

// 五种 K 线共用同形元组，位置 0 为合同声明的开盘时间身份。
fn parse_kline_rows(input: &str) -> BinanceResult<Vec<crate::value::KlineRow>> {
    let rows: Vec<crate::value::KlineRow> = deserialize_strict(input)?;
    let mut open_times = std::collections::HashSet::new();
    if rows.iter().any(|row| !open_times.insert(row.0)) {
        return Err(BinanceError::new(
            BinanceErrorKind::IdentityConflict,
            "K 线批内存在重复开盘时间",
        ));
    }
    Ok(rows)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn delivery_price_rejects_forged_number_objects() {
        for input in [
            r#"[{"deliveryPrice":{"$serde_json::private::Number":"1"}}]"#,
            r#"[{"delivery\u0050rice":{"$serde_json::private::N\u0075mber":"1"}}]"#,
        ] {
            let error = parse_usdm_delivery_price(input).unwrap_err();
            assert_eq!(error.kind(), BinanceErrorKind::SchemaMismatch);
        }
    }

    #[test]
    fn delivery_price_preserves_arbitrary_precision() {
        for number in [
            "123456789012345678901234567890.123456789012345678901234567890",
            "1e+9999",
        ] {
            let rows =
                parse_usdm_delivery_price(&format!("[{{\"deliveryPrice\":{number}}}]")).unwrap();
            assert_eq!(rows[0].delivery_price.as_ref().unwrap().to_string(), number);
        }
    }

    #[test]
    fn delivery_price_wire_scan_ignores_string_braces_and_escapes() {
        assert!(validate_delivery_price_wire(r#"[{"{\\\"}":"}\\\"{"}]"#).is_ok());
        let error = validate_delivery_price_wire(r#"[{"}\\\"{":{"value":1}}]"#).unwrap_err();
        assert_eq!(error.kind(), BinanceErrorKind::SchemaMismatch);
    }

    #[test]
    fn kline_entries_require_unique_open_times() {
        let first = r#"[1,"1","1","1","1","1",2,"1",1,"1","1","0"]"#;
        let different_values = r#"[1,"9","9","9","9","9",3,"9",9,"9","9","0"]"#;
        let second = r#"[2,"1","1","1","1","1",3,"1",1,"1","1","0"]"#;
        type KlineParser = fn(&str) -> BinanceResult<Vec<crate::value::KlineRow>>;
        let parsers: [KlineParser; 5] = [
            |input| parse_usdm_continuous_kline(input).map(|rows| rows.0),
            |input| parse_usdm_kline(input).map(|rows| rows.0),
            |input| parse_usdm_index_price_kline(input).map(|rows| rows.0),
            |input| parse_usdm_mark_price_kline(input).map(|rows| rows.0),
            |input| parse_usdm_premium_index_kline(input).map(|rows| rows.0),
        ];
        for parse in parsers {
            let error = parse(&format!("[{first},{different_values}]")).unwrap_err();
            assert_eq!(error.kind(), BinanceErrorKind::IdentityConflict);
            let rows = parse(&format!("[{second},{first}]")).unwrap();
            assert_eq!(rows.len(), 2);
            assert_eq!((rows[0].0, rows[1].0), (2, 1));
            assert!(parse("[]").unwrap().is_empty());
        }
    }

    #[test]
    fn trading_schedule_accepts_observed_market_keys() {
        let parsed = parse_usdm_trading_schedule(
            r#"{"marketSchedules":{"EQUITY":{"sessions":[]},"COMMODITY":{"sessions":[]},"KR_EQUITY":{"sessions":[]},"HK_EQUITY":{"sessions":[]},"CN_EQUITY":{"sessions":[]},"FX":{"sessions":[]}}}"#,
        )
        .unwrap();
        assert_eq!(parsed.market_schedules.unwrap().len(), 6);
        assert!(parse_usdm_trading_schedule(r#"{"updateTime":1}"#).is_ok());
        assert!(parse_usdm_trading_schedule(r#"{"marketSchedules":{}}"#).is_ok());
    }

    #[test]
    fn trading_schedule_rejects_unknown_market_atomically() {
        let error = parse_usdm_trading_schedule(
            r#"{"marketSchedules":{"EQUITY":{"sessions":[]},"UNOBSERVED_MARKET":{"sessions":[]}}}"#,
        )
        .unwrap_err();
        assert_eq!(error.kind(), BinanceErrorKind::UnknownField);
    }
}
