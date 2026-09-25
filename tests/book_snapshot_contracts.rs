//! 验证四族深度快照的冻结响应结构与本地同步游标策略。

#![allow(clippy::unwrap_used)]

// TDD-PROBE: parse::spot::parse_spot_book_snapshot | 合法 tuple 深度与未知字段 | 红=non_usdm_book_snapshots_preserve_depth_and_reject_unknown_fields | 绿=non_usdm_book_snapshots_preserve_depth_and_reject_unknown_fields
// TDD-PROBE: parse::coinm::parse_coinm_book_snapshot | COINM 深度快照字段与档位 | 红=non_usdm_book_snapshots_preserve_depth_and_reject_unknown_fields | 绿=non_usdm_book_snapshots_preserve_depth_and_reject_unknown_fields
// TDD-PROBE: parse::options::parse_options_book_snapshot | Options 深度快照字段与档位 | 红=non_usdm_book_snapshots_preserve_depth_and_reject_unknown_fields | 绿=non_usdm_book_snapshots_preserve_depth_and_reject_unknown_fields

use binancex::parse::usdm;
use binancex::parse::{coinm, options, spot};
use binancex::BinanceErrorKind;

#[test]
fn non_usdm_book_snapshots_preserve_depth_and_reject_unknown_fields() {
    let spot = spot::parse_spot_book_snapshot(
        r#"{"lastUpdateId":7,"bids":[["1.25","2"]],"asks":[["1.50","3"]]}"#,
    )
    .unwrap();
    assert_eq!(spot.last_update_id, Some(7));
    assert_eq!(
        spot.bids.as_deref(),
        Some(&[("1.25".into(), "2".into())][..])
    );

    let coinm = coinm::parse_coinm_book_snapshot(
        r#"{"lastUpdateId":8,"symbol":"BTCUSD_PERP","pair":"BTCUSD","E":9,"T":10,"bids":[["1","2"]],"asks":[]}"#,
    )
    .unwrap();
    assert_eq!(coinm.symbol.as_deref(), Some("BTCUSD_PERP"));
    assert_eq!(coinm.t, Some(10));

    let options = options::parse_options_book_snapshot(
        r#"{"bids":[["1","2"]],"asks":[],"T":11,"lastUpdateId":12}"#,
    )
    .unwrap();
    assert_eq!(options.last_update_id, Some(12));
    assert_eq!(
        options.bids.as_deref(),
        Some(&[("1".into(), "2".into())][..])
    );

    assert_eq!(
        spot::parse_spot_book_snapshot(r#"{"unexpected":1}"#)
            .unwrap_err()
            .kind(),
        BinanceErrorKind::UnknownField
    );
}

#[test]
fn all_book_snapshots_require_a_non_null_update_cursor() {
    for kind in [
        spot::parse_spot_book_snapshot(r#"{"bids":[],"asks":[]}"#)
            .unwrap_err()
            .kind(),
        usdm::parse_usdm_book_snapshot(r#"{"bids":[],"asks":[]}"#)
            .unwrap_err()
            .kind(),
        coinm::parse_coinm_book_snapshot(r#"{"bids":[],"asks":[]}"#)
            .unwrap_err()
            .kind(),
        options::parse_options_book_snapshot(r#"{"bids":[],"asks":[]}"#)
            .unwrap_err()
            .kind(),
    ] {
        assert_eq!(kind, BinanceErrorKind::Missing);
    }

    for kind in [
        spot::parse_spot_book_snapshot(r#"{"lastUpdateId":null}"#)
            .unwrap_err()
            .kind(),
        usdm::parse_usdm_book_snapshot(r#"{"lastUpdateId":null}"#)
            .unwrap_err()
            .kind(),
        coinm::parse_coinm_book_snapshot(r#"{"lastUpdateId":null}"#)
            .unwrap_err()
            .kind(),
        options::parse_options_book_snapshot(r#"{"lastUpdateId":null}"#)
            .unwrap_err()
            .kind(),
    ] {
        assert_eq!(kind, BinanceErrorKind::SchemaMismatch);
    }
}
