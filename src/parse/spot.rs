//! Spot 族冻结响应的离线解析入口。

#![forbid(unsafe_code)]

use crate::error::{BinanceError, BinanceErrorKind, BinanceResult};
use crate::value::spot::*;
use std::collections::HashSet;

use super::{
    deserialize_strict, validate_unique_nested_string_ids, validate_unique_response_ids,
    validate_unique_response_string_ids,
};

fn reject_explicit_nulls(input: &str) -> BinanceResult<()> {
    fn contains_null(value: &serde_json::Value) -> bool {
        match value {
            serde_json::Value::Null => true,
            serde_json::Value::Array(values) => values.iter().any(contains_null),
            serde_json::Value::Object(values) => values.values().any(contains_null),
            _ => false,
        }
    }

    let value: serde_json::Value = serde_json::from_str(input).map_err(super::classify_error)?;
    if contains_null(&value) {
        return Err(BinanceError::new(
            BinanceErrorKind::SchemaMismatch,
            "Spot 响应含合同未声明可空的 null 字段",
        ));
    }
    Ok(())
}

/// 解析 `SpotExchangeInfo` 的冻结响应结构。
///
/// 缺少非必填字段时保留为空；未知字段或非法形态返回错误。
pub fn parse_spot_exchange_info(input: &str) -> BinanceResult<SpotExchangeInfo> {
    let response: SpotExchangeInfo = deserialize_strict(input)?;
    reject_explicit_nulls(input)?;
    validate_exchange_info_filters(&response)?;
    validate_unique_nested_string_ids(
        input,
        "symbols",
        response
            .symbols
            .iter()
            .flatten()
            .map(|item| item.symbol.clone()),
        "symbol",
        "Spot exchangeInfo 标的",
    )?;
    Ok(response)
}

fn validate_exchange_info_filters(response: &SpotExchangeInfo) -> BinanceResult<()> {
    const SYMBOL_VARIANTS: &[(&str, &[&str])] = &[
        (
            "PRICE_FILTER",
            &["priceExponent", "minPrice", "maxPrice", "tickSize"],
        ),
        (
            "PERCENT_PRICE",
            &["multiplierUp", "multiplierDown", "avgPriceMins"],
        ),
        (
            "PERCENT_PRICE_BY_SIDE",
            &[
                "bidMultiplierUp",
                "bidMultiplierDown",
                "askMultiplierUp",
                "askMultiplierDown",
                "avgPriceMins",
            ],
        ),
        ("LOT_SIZE", &["minQty", "maxQty", "stepSize"]),
        (
            "MIN_NOTIONAL",
            &["minNotional", "applyToMarket", "avgPriceMins"],
        ),
        (
            "NOTIONAL",
            &[
                "minNotional",
                "applyMinToMarket",
                "maxNotional",
                "applyMaxToMarket",
                "avgPriceMins",
            ],
        ),
        ("ICEBERG_PARTS", &["limit"]),
        ("MARKET_LOT_SIZE", &["minQty", "maxQty", "stepSize"]),
        ("MAX_NUM_ORDERS", &["maxNumOrders"]),
        ("MAX_NUM_ALGO_ORDERS", &["maxNumAlgoOrders"]),
        ("MAX_NUM_ICEBERG_ORDERS", &["maxNumIcebergOrders"]),
        ("MAX_POSITION", &["maxPosition"]),
        (
            "TRAILING_DELTA",
            &[
                "minTrailingAboveDelta",
                "maxTrailingAboveDelta",
                "minTrailingBelowDelta",
                "maxTrailingBelowDelta",
            ],
        ),
        ("MAX_NUM_ORDER_AMENDS", &["maxNumOrderAmends"]),
        ("MAX_NUM_ORDER_LISTS", &["maxNumOrderLists"]),
    ];
    const EXCHANGE_VARIANTS: &[(&str, &[&str])] = &[
        ("EXCHANGE_MAX_NUM_ORDERS", &["maxNumOrders"]),
        ("EXCHANGE_MAX_NUM_ALGO_ORDERS", &["maxNumAlgoOrders"]),
        ("EXCHANGE_MAX_NUM_ICEBERG_ORDERS", &["maxNumIcebergOrders"]),
        ("EXCHANGE_MAX_NUM_ORDER_LISTS", &["maxNumOrderLists"]),
    ];

    for symbol in response.symbols.iter().flatten() {
        for filter in symbol.filters.iter().flatten() {
            validate_filter_variant(
                filter.filter_type.as_deref(),
                &[
                    ("priceExponent", filter.price_exponent.is_some()),
                    ("minPrice", filter.min_price.is_some()),
                    ("maxPrice", filter.max_price.is_some()),
                    ("tickSize", filter.tick_size.is_some()),
                    ("multiplierUp", filter.multiplier_up.is_some()),
                    ("multiplierDown", filter.multiplier_down.is_some()),
                    ("avgPriceMins", filter.avg_price_mins.is_some()),
                    ("bidMultiplierUp", filter.bid_multiplier_up.is_some()),
                    ("bidMultiplierDown", filter.bid_multiplier_down.is_some()),
                    ("askMultiplierUp", filter.ask_multiplier_up.is_some()),
                    ("askMultiplierDown", filter.ask_multiplier_down.is_some()),
                    ("minQty", filter.min_qty.is_some()),
                    ("maxQty", filter.max_qty.is_some()),
                    ("stepSize", filter.step_size.is_some()),
                    ("minNotional", filter.min_notional.is_some()),
                    ("maxNotional", filter.max_notional.is_some()),
                    ("applyToMarket", filter.apply_to_market.is_some()),
                    ("applyMinToMarket", filter.apply_min_to_market.is_some()),
                    ("applyMaxToMarket", filter.apply_max_to_market.is_some()),
                    ("limit", filter.limit.is_some()),
                    ("maxNumOrders", filter.max_num_orders.is_some()),
                    ("maxNumAlgoOrders", filter.max_num_algo_orders.is_some()),
                    (
                        "maxNumIcebergOrders",
                        filter.max_num_iceberg_orders.is_some(),
                    ),
                    ("maxPosition", filter.max_position.is_some()),
                    (
                        "minTrailingAboveDelta",
                        filter.min_trailing_above_delta.is_some(),
                    ),
                    (
                        "maxTrailingAboveDelta",
                        filter.max_trailing_above_delta.is_some(),
                    ),
                    (
                        "minTrailingBelowDelta",
                        filter.min_trailing_below_delta.is_some(),
                    ),
                    (
                        "maxTrailingBelowDelta",
                        filter.max_trailing_below_delta.is_some(),
                    ),
                    ("maxNumOrderAmends", filter.max_num_order_amends.is_some()),
                    ("maxNumOrderLists", filter.max_num_order_lists.is_some()),
                    ("endTime", filter.end_time.is_some()),
                ],
                SYMBOL_VARIANTS,
                true,
            )?;
        }
    }

    for filter in response.exchange_filters.iter().flatten() {
        validate_filter_variant(
            filter.filter_type.as_deref(),
            &[
                ("maxNumOrders", filter.max_num_orders.is_some()),
                ("maxNumAlgoOrders", filter.max_num_algo_orders.is_some()),
                (
                    "maxNumIcebergOrders",
                    filter.max_num_iceberg_orders.is_some(),
                ),
                ("maxNumOrderLists", filter.max_num_order_lists.is_some()),
            ],
            EXCHANGE_VARIANTS,
            false,
        )?;
    }
    Ok(())
}

fn validate_filter_variant(
    filter_type: Option<&str>,
    fields: &[(&str, bool)],
    variants: &[(&str, &[&str])],
    allow_unresolved_t_plus_sell: bool,
) -> BinanceResult<()> {
    let mismatch = || {
        BinanceError::new(
            BinanceErrorKind::SchemaMismatch,
            "过滤器判别值与字段组合不符合响应合同",
        )
    };
    let filter_type = filter_type.ok_or_else(mismatch)?;
    let Some((_, allowed)) = variants.iter().find(|(tag, _)| *tag == filter_type) else {
        if allow_unresolved_t_plus_sell && filter_type == "T_PLUS_SELL" {
            return Ok(());
        }
        return Err(mismatch());
    };
    if fields
        .iter()
        .any(|(field, present)| *present && !allowed.contains(field))
    {
        return Err(mismatch());
    }
    Ok(())
}

/// 解析 `SpotExecutionRules` 的冻结响应结构。
///
/// 缺少非必填字段时保留为空；未知字段或非法形态返回错误。
pub fn parse_spot_execution_rules(input: &str) -> BinanceResult<SpotExecutionRules> {
    deserialize_strict(input)
}

/// 解析 `SpotAggTrade` 的冻结响应结构。
///
/// 缺少本地关键字段 `a` 返回 Missing，显式 null 返回 SchemaMismatch；
/// 同一响应批内重复 `a` 返回 IdentityConflict。此规则不表示源方保证唯一性。
pub fn parse_spot_agg_trade(input: &str) -> BinanceResult<SpotAggTrade> {
    let response: SpotAggTrade = deserialize_strict(input)?;
    validate_unique_response_ids(input, response.iter().map(|item| item.a), "a", "聚合成交")?;
    Ok(response)
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
/// 缺少本地关键字段 `id` 返回 Missing，显式 null 返回 SchemaMismatch；
/// 同一响应批内重复 `id` 返回 IdentityConflict。此规则不表示源方保证唯一性。
pub fn parse_spot_trade(input: &str) -> BinanceResult<SpotTrade> {
    let response: SpotTrade = deserialize_strict(input)?;
    validate_unique_response_ids(input, response.iter().map(|item| item.id), "id", "现货成交")?;
    Ok(response)
}

/// 解析 `SpotBlockTrade` 的冻结响应结构。
///
/// 缺少本地关键字段 `id` 返回 `Missing`，显式 null 返回 `SchemaMismatch`；
/// 同一响应批内重复 `id` 返回 `IdentityConflict`。此规则不表示源方保证唯一性。
pub fn parse_spot_block_trade(input: &str) -> BinanceResult<SpotBlockTrade> {
    let response: SpotBlockTrade = deserialize_strict(input)?;
    validate_unique_response_ids(
        input,
        response.iter().map(|item| item.id),
        "id",
        "现货大宗成交",
    )?;
    Ok(response)
}

/// 解析 `SpotKline` 的冻结响应结构。
///
/// 开盘时间为批内身份键；重复时返回 `IdentityConflict`，整批原子失败。
pub fn parse_spot_kline(input: &str) -> BinanceResult<SpotKline> {
    let rows: Vec<crate::value::KlineRow> = deserialize_strict(input)?;
    let mut open_times = HashSet::new();
    if rows.iter().any(|row| !open_times.insert(row.0)) {
        return Err(BinanceError::new(
            BinanceErrorKind::IdentityConflict,
            "Spot Kline 批内开盘时间重复",
        ));
    }
    Ok(SpotKline(rows))
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
/// `symbol` 和 `price` 是业务关键字段；缺失或空值、未知字段及非法形态均返回错误。
pub fn parse_spot_ticker_price(input: &str) -> BinanceResult<SpotTickerPrice> {
    match input.trim_start().as_bytes().first() {
        Some(b'{') => {
            let item: Box<SpotTickerPriceItem> = deserialize_strict(input)?;
            validate_spot_ticker_price_item(&item)?;
            Ok(SpotTickerPrice::Object(item))
        }
        Some(b'[') => {
            let items: Vec<SpotTickerPriceItem> = deserialize_strict(input)?;
            for item in &items {
                validate_spot_ticker_price_item(item)?;
            }
            validate_unique_response_string_ids(
                input,
                items.iter().map(|item| item.symbol.clone()),
                "symbol",
                "Spot ticker price",
            )?;
            Ok(SpotTickerPrice::Array(items))
        }
        _ => deserialize_strict(input),
    }
}

fn validate_spot_ticker_price_item(item: &SpotTickerPriceItem) -> BinanceResult<()> {
    if item
        .symbol
        .as_deref()
        .map_or(true, |value| value.trim().is_empty())
    {
        return Err(BinanceError::new(
            BinanceErrorKind::Missing,
            "Spot Ticker Price 缺少非空 symbol",
        ));
    }
    let price = item
        .price
        .as_deref()
        .filter(|value| !value.trim().is_empty())
        .ok_or_else(|| {
            BinanceError::new(
                BinanceErrorKind::Missing,
                "Spot Ticker Price 缺少非空 price",
            )
        })?;
    crate::value::validate_decimal(price)
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

/// 离线解析 SpotBookSnapshot 的冻结深度快照结构。
///
/// # Errors
///
/// 未知字段返回 `UnknownField`；JSON 语法错误返回 `Invalid`；结构、字段类型或档位形态错误返回 `SchemaMismatch`。
pub fn parse_spot_book_snapshot(input: &str) -> BinanceResult<SpotBookSnapshot> {
    let response = deserialize_strict(input)?;
    super::require_non_null_field(input, "lastUpdateId")?;
    Ok(response)
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
///
/// # Errors
///
/// 未知字段返回 `UnknownField`；`calculationType` 缺失、未登记或条件字段与判别值不匹配时返回 `SchemaMismatch`。
pub fn parse_spot_reference_price_calculation(
    input: &str,
) -> BinanceResult<SpotReferencePriceCalculation> {
    let response: SpotReferencePriceCalculation = deserialize_strict(input)?;
    reject_explicit_nulls(input)?;
    let valid = match response.calculation_type.as_deref() {
        Some("ARITHMETIC_MEAN") => response.external_calculation_id.is_none(),
        Some("EXTERNAL") => response.bucket_count.is_none() && response.bucket_width_ms.is_none(),
        _ => false,
    };
    if !valid {
        return Err(BinanceError::new(
            BinanceErrorKind::SchemaMismatch,
            "calculationType 与条件响应字段不匹配",
        ));
    }
    Ok(response)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spot_branch_payloads_reject_explicit_null_fields() {
        assert_eq!(
            parse_spot_exchange_info(
                r#"{"symbols":[{"filters":[{"filterType":"PRICE_FILTER","maxNumOrders":null}]}]}"#
            )
            .unwrap_err()
            .kind(),
            BinanceErrorKind::SchemaMismatch
        );
        assert_eq!(
            parse_spot_reference_price_calculation(
                r#"{"calculationType":"EXTERNAL","bucketCount":null}"#
            )
            .unwrap_err()
            .kind(),
            BinanceErrorKind::SchemaMismatch
        );
        assert_eq!(
            parse_spot_exchange_info(r#"{"unregisteredField":null}"#)
                .unwrap_err()
                .kind(),
            BinanceErrorKind::UnknownField
        );
    }

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
