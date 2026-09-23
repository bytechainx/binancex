//! 标准章节与行为验收逐项映射；内联样本全部为合成。
// SPEC-MAP: S-1 | 合成夹具 | synthetic_metadata_stays_outside_wire_payload
// SPEC-MAP: S-2 | 冻结合同 | frozen_wire_names_and_nested_shapes_are_enforced
// SPEC-MAP: S-3 | 当前开放点与行为 | open_points_do_not_authorize_unseen_market_keys
// SPEC-MAP: S-4 | 结构检查与语义检查 | structure_and_supported_semantics_are_distinct
// SPEC-MAP: S-5 | 测试与验收 | acceptance_keeps_authorization_fail_closed

use binancex::parse::{options::parse_options_exchange_info, spot::*, usdm::*};
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
fn open_points_do_not_authorize_unseen_market_keys() {
    assert!(parse_usdm_trading_schedule(r#"{"marketSchedules":{"FX":{"sessions":[]}}}"#).is_ok());
    assert_eq!(
        parse_usdm_trading_schedule(r#"{"marketSchedules":{"NEW_MARKET":{"sessions":[]}}}"#)
            .unwrap_err()
            .kind(),
        BinanceErrorKind::UnknownField
    );
    // assets 字段未展开；空结构可读不代表资产语义完整。
    assert!(parse_usdm_insurance_balance(r#"{"assets":[{}]}"#).is_ok());
    assert_eq!(
        parse_usdm_insurance_balance(r#"{"assets":[{"asset":"SYNTH"}]}"#)
            .unwrap_err()
            .kind(),
        BinanceErrorKind::UnknownField
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
