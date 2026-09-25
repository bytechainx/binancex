#![allow(clippy::unwrap_used)]
//! 标准章节与行为验收逐项映射；内联样本全部为合成。
// SPEC-MAP: S-1 | 合成夹具 | synthetic_metadata_stays_outside_wire_payload
// SPEC-MAP: S-2 | 冻结合同 | frozen_wire_names_and_nested_shapes_are_enforced
// SPEC-MAP: S-3 | 当前开放点与行为 | open_points_do_not_authorize_unseen_market_keys
// SPEC-MAP: S-4 | 结构检查与语义检查 | structure_and_supported_semantics_are_distinct
// SPEC-MAP: S-5 | 测试与验收 | acceptance_keeps_authorization_fail_closed

use binancex::parse::{options::parse_options_exchange_info, spot::*, usdm::*};
use binancex::value::usdm::UsdmInsuranceBalance;
use binancex::{current_authorization, BinanceErrorKind, Date, Decimal, Sign};
use serde_json::json;

#[test]
fn synthetic_metadata_stays_outside_wire_payload() {
    let fixture = json!({
        "_synthetic":true,
        "_note":"合成夹具，不代表真实源证据或授权",
        "payload":{"symbol":"SYNTH","referencePrice":"-0"}
    });
    assert_eq!(fixture["_synthetic"], true);
    assert!(!fixture["_note"].as_str().unwrap().is_empty());
    assert!(parse_spot_reference_price(&fixture["payload"].to_string()).is_ok());
    assert_eq!(
        parse_spot_reference_price(&fixture.to_string())
            .unwrap_err()
            .kind(),
        BinanceErrorKind::UnknownField
    );
}

#[test]
fn frozen_wire_names_and_nested_shapes_are_enforced() {
    let response = parse_options_exchange_info(r#"{"optionAssets":[{"name":"SYNTH"}]}"#).unwrap();
    assert_eq!(
        response.option_assets.unwrap()[0].name.as_deref(),
        Some("SYNTH")
    );
    assert_eq!(
        parse_options_exchange_info(r#"{"option_assets":[{"name":"SYNTH"}]}"#)
            .unwrap_err()
            .kind(),
        BinanceErrorKind::UnknownField
    );
    assert_eq!(
        parse_options_exchange_info(r#"{"optionAssets":[["SYNTH"]]}"#)
            .unwrap_err()
            .kind(),
        BinanceErrorKind::SchemaMismatch
    );
}

#[test]
fn usdm_convert_from_is_base_accepts_optional_boolean_only() {
    let with_field =
        parse_usdm_convert_exchange_info(r#"[{"fromAsset":"SYNTH","fromIsBase":true}]"#).unwrap();
    assert_eq!(with_field[0].from_is_base, Some(true));

    let without_field = parse_usdm_convert_exchange_info(r#"[{"fromAsset":"SYNTH"}]"#).unwrap();
    assert_eq!(without_field[0].from_is_base, None);

    assert_eq!(
        parse_usdm_convert_exchange_info(r#"[{"fromIsBase":"true"}]"#)
            .unwrap_err()
            .kind(),
        BinanceErrorKind::SchemaMismatch
    );
}

#[test]
fn spot_exchange_info_filter_fields_match_official_definitions() {
    let response = json!({
        "symbols": [{
            "symbol": "BTCUSDT",
            "filters": [
                {"filterType":"PRICE_FILTER","priceExponent":8,"minPrice":"0","maxPrice":"1","tickSize":"0.1"},
                {"filterType":"PERCENT_PRICE","multiplierUp":"1.3","multiplierDown":"0.7","avgPriceMins":5},
                {"filterType":"PERCENT_PRICE_BY_SIDE","bidMultiplierUp":"1.2","bidMultiplierDown":"0.2","askMultiplierUp":"5","askMultiplierDown":"0.8","avgPriceMins":1},
                {"filterType":"LOT_SIZE","minQty":"0.1","maxQty":"10","stepSize":"0.1"},
                {"filterType":"MIN_NOTIONAL","minNotional":"1","applyToMarket":true,"avgPriceMins":5},
                {"filterType":"NOTIONAL","minNotional":"1","applyMinToMarket":false,"maxNotional":"100","applyMaxToMarket":false,"avgPriceMins":5},
                {"filterType":"ICEBERG_PARTS","limit":10},
                {"filterType":"MARKET_LOT_SIZE","minQty":"0.1","maxQty":"10","stepSize":"0.1"},
                {"filterType":"MAX_NUM_ORDERS","maxNumOrders":25},
                {"filterType":"MAX_NUM_ALGO_ORDERS","maxNumAlgoOrders":5},
                {"filterType":"MAX_NUM_ICEBERG_ORDERS","maxNumIcebergOrders":5},
                {"filterType":"MAX_POSITION","maxPosition":"10"},
                {"filterType":"TRAILING_DELTA","minTrailingAboveDelta":10,"maxTrailingAboveDelta":2000,"minTrailingBelowDelta":10,"maxTrailingBelowDelta":2000},
                {"filterType":"MAX_NUM_ORDER_AMENDS","maxNumOrderAmends":10},
                {"filterType":"MAX_NUM_ORDER_LISTS","maxNumOrderLists":20},
                {"filterType":"T_PLUS_SELL","endTime":1750000000000_i64}
            ]
        }],
        "exchangeFilters": [
            {"filterType":"EXCHANGE_MAX_NUM_ORDERS","maxNumOrders":1000},
            {"filterType":"EXCHANGE_MAX_NUM_ALGO_ORDERS","maxNumAlgoOrders":200},
            {"filterType":"EXCHANGE_MAX_NUM_ICEBERG_ORDERS","maxNumIcebergOrders":10000},
            {"filterType":"EXCHANGE_MAX_NUM_ORDER_LISTS","maxNumOrderLists":20}
        ]
    });

    let parsed = parse_spot_exchange_info(&response.to_string()).unwrap();
    assert_eq!(
        parsed.symbols.unwrap()[0]
            .filters
            .as_ref()
            .unwrap()
            .last()
            .unwrap()
            .end_time,
        Some(1_750_000_000_000)
    );
    assert_eq!(
        parse_spot_exchange_info(
            r#"{"symbols":[{"filters":[{"filterType":"PRICE_FILTER","newField":1}]}]}"#
        )
        .unwrap_err()
        .kind(),
        BinanceErrorKind::UnknownField
    );
    for raw in [
        r#"{"symbols":[{"symbol":"BTCUSDT","filters":[{"filterType":"PRICE_FILTER","maxNumOrders":1}]}]}"#,
        r#"{"symbols":[{"symbol":"BTCUSDT","filters":[{"filterType":"T_PLUS_SELL","endTime":"1750000000000"}]}]}"#,
        r#"{"exchangeFilters":[{"filterType":"EXCHANGE_MAX_NUM_ORDERS","maxNumAlgoOrders":1}]}"#,
        r#"{"symbols":[{"filters":[{"filterType":"UNSEEN_FILTER"}]}]}"#,
        r#"{"exchangeFilters":[{"maxNumOrders":1}]}"#,
    ] {
        assert_eq!(
            parse_spot_exchange_info(raw).unwrap_err().kind(),
            BinanceErrorKind::SchemaMismatch
        );
    }
    assert!(parse_spot_exchange_info(
        r#"{"symbols":[{"symbol":"BTCUSDT","filters":[{"filterType":"T_PLUS_SELL"}]}]}"#
    )
    .is_ok());
}

#[test]
fn usdm_exchange_info_filter_union_accepts_documented_mixed_sample() {
    // 合成夹具：保持官方 Exchange Information 示例中的混合字段组合可解析。
    let parsed = parse_usdm_exchange_info(
        r#"{"symbols":[{"symbol":"BTCUSDT","filters":[{"filterType":"PRICE_FILTER","maxPrice":"300","minPrice":"0.0001","tickSize":"0.0001","maxQty":"10000000","minQty":"1","stepSize":"1","limit":200,"notional":"5.0","multiplierUp":"1.1500","multiplierDown":"0.8500","multiplierDecimal":"4"}]}]}"#,
    )
    .unwrap();
    let filter = &parsed.symbols.as_ref().unwrap()[0]
        .filters
        .as_ref()
        .unwrap()[0];

    assert_eq!(filter.filter_type.as_deref(), Some("PRICE_FILTER"));
    assert_eq!(filter.limit, Some(200));
    assert_eq!(filter.max_qty.as_deref(), Some("10000000"));
    assert_eq!(filter.multiplier_decimal.as_deref(), Some("4"));
}

#[test]
fn spot_reference_price_calculation_fields_follow_calculation_type() {
    assert!(parse_spot_reference_price_calculation(
        r#"{"calculationType":"ARITHMETIC_MEAN","bucketCount":10,"bucketWidthMs":1000}"#
    )
    .is_ok());
    assert!(parse_spot_reference_price_calculation(
        r#"{"calculationType":"EXTERNAL","externalCalculationId":42}"#
    )
    .is_ok());
    for raw in [
        r#"{"calculationType":"ARITHMETIC_MEAN","bucketCount":10,"externalCalculationId":42}"#,
        r#"{"calculationType":"EXTERNAL","externalCalculationId":42,"bucketWidthMs":1000}"#,
        r#"{"calculationType":"UNKNOWN"}"#,
        r#"{"bucketCount":10}"#,
    ] {
        assert_eq!(
            parse_spot_reference_price_calculation(raw)
                .unwrap_err()
                .kind(),
            BinanceErrorKind::SchemaMismatch
        );
    }
}

#[test]
fn variant_two_array_responses_parse_object_items() {
    // 合成数组样本只验证已实现的元素类型，不把它们当作官方 Variant 2 字段证据。
    type ArrayCase = (&'static str, &'static str, fn(&str) -> bool);
    let cases: &[ArrayCase] = &[
        ("BN-USDM-REST-001", r#"[{"symbol":"SYNTH"}]"#, |raw| {
            parse_usdm_adl_risk(raw).is_ok()
        }),
        ("BN-USDM-REST-013", r#"[{"symbol":"SYNTH"}]"#, |raw| {
            parse_usdm_premium_index(raw).is_ok()
        }),
        ("BN-USDM-REST-015", r#"[{"symbol":"SYNTH"}]"#, |raw| {
            parse_usdm_asset_index(raw).is_ok()
        }),
        (
            "BN-USDM-REST-023",
            r#"[{"symbols":[],"assets":[]}]"#,
            |raw| parse_usdm_insurance_balance(raw).is_ok(),
        ),
        ("BN-USDM-REST-026", r#"[{"symbol":"SYNTH"}]"#, |raw| {
            parse_usdm_book_ticker(raw).is_ok()
        }),
        ("BN-USDM-REST-027/028", r#"[{"symbol":"SYNTH"}]"#, |raw| {
            parse_usdm_ticker_price(raw).is_ok()
        }),
        ("BN-USDM-REST-031", r#"[{"symbol":"SYNTH"}]"#, |raw| {
            parse_usdm_ticker24hr(raw).is_ok()
        }),
        ("BN-SPOT-REST-012", r#"[{"symbol":"SYNTH"}]"#, |raw| {
            parse_spot_ticker(raw).is_ok()
        }),
        ("BN-SPOT-REST-013", r#"[{"symbol":"SYNTH"}]"#, |raw| {
            parse_spot_ticker24hr(raw).is_ok()
        }),
        ("BN-SPOT-REST-014", r#"[{"symbol":"SYNTH"}]"#, |raw| {
            parse_spot_book_ticker(raw).is_ok()
        }),
        ("BN-SPOT-REST-015", r#"[{"symbol":"SYNTH"}]"#, |raw| {
            parse_spot_ticker_price(raw).is_ok()
        }),
        ("BN-SPOT-REST-016", r#"[{"symbol":"SYNTH"}]"#, |raw| {
            parse_spot_trading_day(raw).is_ok()
        }),
    ];
    for (endpoint_id, raw, parse) in cases {
        assert!(parse(raw), "{endpoint_id} 拒绝合成数组形态");
    }
}

#[test]
fn open_points_do_not_authorize_unseen_market_keys() {
    assert!(parse_usdm_trading_schedule(r#"{"marketSchedules":{"FX":{"sessions":[]}}}"#).is_ok());
    assert_eq!(
        parse_usdm_trading_schedule(r#"{"marketSchedules":{"NEW_MARKET":{"sessions":[]}}}"#)
            .unwrap_err()
            .kind(),
        BinanceErrorKind::UnknownField
    );
    // OpenAPI 已展开 assets 项字段，但 symbols 与资产快照的业务关系仍未确认。
    assert!(parse_usdm_insurance_balance(r#"{"assets":[{"asset":"SYNTH"}]}"#).is_ok());
    assert_eq!(
        parse_usdm_insurance_balance(r#"{"assets":[{"futureField":"SYNTH"}]}"#)
            .unwrap_err()
            .kind(),
        BinanceErrorKind::UnknownField
    );
}

#[test]
fn usdm_insurance_balance_assets_follow_official_openapi_fields() {
    let object = parse_usdm_insurance_balance(
        r#"{"symbols":["SYNTH"],"assets":[{"asset":"USDT","marginBalance":"1.25","updateTime":1700000000000}]}"#,
    )
    .unwrap();
    let object = match object {
        UsdmInsuranceBalance::Object(object) => Some(object),
        UsdmInsuranceBalance::Array(_) => None,
    };
    assert!(object.is_some());
    let object = object.unwrap();
    let asset = &object.assets.as_ref().unwrap()[0];
    assert_eq!(asset.asset.as_deref(), Some("USDT"));
    assert_eq!(asset.margin_balance.as_deref(), Some("1.25"));
    assert_eq!(asset.update_time, Some(1_700_000_000_000));

    let array = parse_usdm_insurance_balance(
        r#"[{"symbols":[],"assets":[{"asset":"USDC","marginBalance":"2.5","updateTime":1700000000001}]}]"#,
    )
    .unwrap();
    let array = match array {
        UsdmInsuranceBalance::Array(array) => Some(array),
        UsdmInsuranceBalance::Object(_) => None,
    };
    assert!(array.is_some());
    let array = array.unwrap();
    assert_eq!(
        array[0].assets.as_ref().unwrap()[0].asset.as_deref(),
        Some("USDC")
    );
}

#[test]
fn structure_and_supported_semantics_are_distinct() {
    assert_eq!(
        parse_spot_reference_price(r#"{"symbol":"SYNTH"}"#)
            .unwrap()
            .reference_price,
        None
    );
    assert_eq!(Decimal::new("-0.0e999").unwrap().sign(), Sign::Zero);
    assert_eq!(
        parse_spot_avg_price(r#"{"price":"1"}"#).unwrap_err().kind(),
        BinanceErrorKind::Missing
    );
    let row = json!([1, "1", "1", "1", "1", "1", 2, "1", 1, "1", "1", "0"]);
    assert_eq!(
        parse_spot_kline(&json!([row, row]).to_string())
            .unwrap_err()
            .kind(),
        BinanceErrorKind::IdentityConflict
    );
}

#[test]
fn acceptance_keeps_authorization_fail_closed() {
    let today = Date::new(2026, 9, 23).unwrap();
    assert!(!current_authorization(today, &[]).is_authorized());
    assert!(include_str!("../Cargo.toml").contains("production_decision = \"NO-GO\""));
    assert_eq!(
        parse_spot_reference_price(r#"{"symbol":"SYNTH","unverifiedField":true}"#)
            .unwrap_err()
            .kind(),
        BinanceErrorKind::UnknownField
    );
}
