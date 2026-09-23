#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::unreachable
)]

//! AI 提出的边界用例，经 Codex 按所列不变量逐条核对；内联数据均为合成。
// AIDD: exponent_sign_does_not_change_zero | 来源=AI | 复核=Codex/2026-09-23 | 依据=Decimal 有效数字决定零与符号 | 结论=保留
// AIDD: nested_unknown_field_rejects_complete_batch | 来源=AI | 复核=Codex/2026-09-23 | 依据=未知字段整批原子失败 | 结论=保留
// AIDD: missing_reference_is_distinct_from_negative_zero | 来源=AI | 复核=Codex/2026-09-23 | 依据=参考价格缺失不得代填零 | 结论=保留
// AIDD: authorization_conflict_is_order_independent | 来源=AI | 复核=Codex/2026-09-23 | 依据=冲突范围 fail-closed 不挑证据子集 | 结论=保留
// AIDD: json_number_is_precise_and_internal_tag_is_rejected | 来源=AI | 复核=Codex/2026-09-23 | 依据=JSON number 无损且禁止伪造保留对象 | 结论=保留
// AIDD: new_market_key_is_not_accepted_by_map_flexibility | 来源=AI | 复核=Codex/2026-09-23 | 依据=USDM 034 新键补证后放行 | 结论=保留
// AIDD: average_price_missing_null_and_wrong_type_are_distinct | 来源=AI | 复核=Codex/2026-09-23 | 依据=选定关键字段缺键 Missing、空值或错型 Invalid | 结论=保留

use binancex::parse::{spot::*, usdm::*};
use binancex::{current_authorization, registered_evidence, BinanceErrorKind, Date, Decimal, Sign};
use serde_json::json;

#[test]
fn exponent_sign_does_not_change_zero() {
    for raw in ["-0e-9999", "0e9999", "+0.000e+12"] {
        let value = Decimal::new(raw).unwrap();
        assert_eq!(value.sign(), Sign::Zero);
        assert_eq!(value.as_str(), raw);
    }
    assert_eq!(Decimal::new("-1e-9999").unwrap().sign(), Sign::Negative);
    assert!(Decimal::new("1e--2").is_err());
}

#[test]
fn nested_unknown_field_rejects_complete_batch() {
    assert_eq!(parse_spot_exchange_info(r#"{"symbols":[{"symbol":"SYNTH"},{"symbol":"SECOND","filters":[{"filterType":"PRICE_FILTER","newBound":"1"}]}]}"#).unwrap_err().kind(), BinanceErrorKind::UnknownField);
    assert_eq!(
        parse_spot_exchange_info(
            r#"{"symbols":[{"symbol":"SYNTH","filters":[["PRICE_FILTER"]]}]}"#
        )
        .unwrap_err()
        .kind(),
        BinanceErrorKind::SchemaMismatch
    );
}

#[test]
fn missing_reference_is_distinct_from_negative_zero() {
    let absent = parse_spot_reference_price(r#"{"symbol":"SYNTH"}"#).unwrap();
    let null = parse_spot_reference_price(r#"{"symbol":"SYNTH","referencePrice":null}"#).unwrap();
    let zero = parse_spot_reference_price(r#"{"symbol":"SYNTH","referencePrice":"-0"}"#).unwrap();
    assert_eq!(absent.reference_price, None);
    assert_eq!(null.reference_price, None);
    assert_eq!(zero.reference_price.unwrap().as_str(), "-0");
}

#[test]
fn authorization_conflict_is_order_independent() {
    let today = Date::new(2026, 9, 23).unwrap();
    let evidence = |scope| {
        registered_evidence(
            &json!({
                "evidence_id":"SYNTH", "signed_by":"synthetic-owner", "scope":scope,
                "valid_from":"2026-09-23", "valid_until":"2026-09-23"
            })
            .to_string(),
        )
        .unwrap()
    };
    let a = evidence("offline:spot");
    let b = evidence("offline:usdm");
    assert!(current_authorization(today, std::slice::from_ref(&a)).is_authorized());
    assert!(!current_authorization(today, &[a.clone(), b.clone()]).is_authorized());
    assert!(!current_authorization(today, &[b, a]).is_authorized());
    assert!(!current_authorization(today, &[]).is_authorized());
}

#[test]
fn json_number_is_precise_and_internal_tag_is_rejected() {
    let response = parse_usdm_delivery_price(
        r#"[{"deliveryPrice":123456789012345678901234567890.123456789}]"#,
    )
    .unwrap();
    assert_eq!(
        response[0].delivery_price.as_ref().unwrap().to_string(),
        "123456789012345678901234567890.123456789"
    );
    assert_eq!(
        parse_usdm_delivery_price(r#"[{"deliveryPrice":{"$serde_json::private::Number":"1"}}]"#)
            .unwrap_err()
            .kind(),
        BinanceErrorKind::SchemaMismatch
    );
    assert_eq!(
        parse_spot_trade(r#"[{"id":9223372036854775808}]"#)
            .unwrap_err()
            .kind(),
        BinanceErrorKind::LossyNumeric
    );
}

#[test]
fn new_market_key_is_not_accepted_by_map_flexibility() {
    assert_eq!(
        parse_usdm_trading_schedule(
            r#"{"marketSchedules":{"FX":{"sessions":[]},"UNVERIFIED":{"sessions":[]}}}"#
        )
        .unwrap_err()
        .kind(),
        BinanceErrorKind::UnknownField
    );
}

#[test]
fn average_price_missing_null_and_wrong_type_are_distinct() {
    for raw in [r#"{"price":"1"}"#, r#"{"closeTime":1}"#] {
        assert_eq!(
            parse_spot_avg_price(raw).unwrap_err().kind(),
            BinanceErrorKind::Missing
        );
    }
    for raw in [
        r#"{"price":null,"closeTime":1}"#,
        r#"{"price":"1","closeTime":null}"#,
        r#"{"price":1,"closeTime":1}"#,
        r#"{"price":"1","closeTime":"1"}"#,
    ] {
        assert_eq!(
            parse_spot_avg_price(raw).unwrap_err().kind(),
            BinanceErrorKind::Invalid
        );
    }
    assert!(parse_spot_avg_price(r#"{"price":"1","closeTime":1}"#).is_ok());
}
