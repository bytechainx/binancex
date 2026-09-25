//! 非 Kline 批内身份策略的合成负例。

use binancex::parse::spot::{parse_spot_agg_trade, parse_spot_trade};
use binancex::BinanceErrorKind;

#[test]
fn spot_aggregate_trade_requires_unique_local_ids_per_response() {
    assert!(parse_spot_agg_trade(r#"[{"a":1},{"a":2}]"#).is_ok());
    assert!(parse_spot_agg_trade("[]").unwrap().is_empty());

    assert_eq!(
        parse_spot_agg_trade(r#"[{"a":1},{"a":1}]"#)
            .unwrap_err()
            .kind(),
        BinanceErrorKind::IdentityConflict
    );
    assert_eq!(
        parse_spot_agg_trade(r#"[{"p":"1"}]"#).unwrap_err().kind(),
        BinanceErrorKind::Missing
    );
    assert_eq!(
        parse_spot_agg_trade(r#"[{"a":null}]"#).unwrap_err().kind(),
        BinanceErrorKind::SchemaMismatch
    );
}

#[test]
fn spot_trade_requires_unique_local_ids_per_response() {
    assert!(parse_spot_trade(r#"[{"id":1},{"id":2}]"#).is_ok());

    assert_eq!(
        parse_spot_trade(r#"[{"id":1},{"id":1}]"#)
            .unwrap_err()
            .kind(),
        BinanceErrorKind::IdentityConflict
    );
    assert_eq!(
        parse_spot_trade(r#"[{"price":"1"}]"#).unwrap_err().kind(),
        BinanceErrorKind::Missing
    );
    assert_eq!(
        parse_spot_trade(r#"[{"id":null}]"#).unwrap_err().kind(),
        BinanceErrorKind::SchemaMismatch
    );
}
