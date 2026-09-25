#![allow(clippy::unwrap_used, clippy::unreachable)]
//! 三类测试中的 TDD 契约探针；内联响应全部是合成样本，不代表真实源证据。
//! 主覆盖清单为 92 个入口：79 个族解析器与 13 个共享/值对象入口。
// TDD-PROBE: parse::spot::parse_spot_exchange_info | 变异：绕过结构校验或误拒合法响应 | 红=spot_public_parsers | 绿=spot_public_parsers
// TDD-PROBE: parse::spot::parse_spot_execution_rules | 变异：绕过结构校验或误拒合法响应 | 红=spot_public_parsers | 绿=spot_public_parsers
// TDD-PROBE: parse::spot::parse_spot_agg_trade | 变异：绕过结构校验或误拒合法响应 | 红=spot_public_parsers | 绿=spot_public_parsers
// TDD-PROBE: parse::spot::parse_spot_avg_price | 变异：绕过结构校验或误拒合法响应 | 红=spot_public_parsers | 绿=spot_public_parsers
// TDD-PROBE: parse::spot::parse_spot_trade | 变异：绕过结构校验或误拒合法响应 | 红=spot_public_parsers | 绿=spot_public_parsers
// TDD-PROBE: parse::spot::parse_spot_block_trade | 变异：放行缺失或批内重复 id | 红=spot_block_trade_requires_unique_local_ids_per_response | 绿=spot_block_trade_requires_unique_local_ids_per_response
// TDD-PROBE: parse::spot::parse_spot_kline | 变异：绕过结构校验或误拒合法响应 | 红=spot_public_parsers | 绿=spot_public_parsers
// TDD-PROBE: parse::spot::parse_spot_ui_kline | 变异：绕过结构校验或误拒合法响应 | 红=spot_public_parsers | 绿=spot_public_parsers
// TDD-PROBE: parse::spot::parse_spot_ticker | 变异：绕过结构校验或误拒合法响应 | 红=spot_public_parsers | 绿=spot_public_parsers
// TDD-PROBE: parse::spot::parse_spot_ticker24hr | 变异：绕过结构校验或误拒合法响应 | 红=spot_public_parsers | 绿=spot_public_parsers
// TDD-PROBE: parse::spot::parse_spot_ticker_price | 变异：放行缺失 symbol/price、重复 symbol 或非法 price | 红=spot_ticker_price_requires_symbol_and_price_for_each_item | 绿=spot_ticker_price_requires_symbol_and_price_for_each_item
// TDD-PROBE: parse::spot::parse_spot_book_ticker | 变异：绕过结构校验或误拒合法响应 | 红=spot_public_parsers | 绿=spot_public_parsers
// TDD-PROBE: parse::spot::parse_spot_book_snapshot | 变异：误拒 tuple 深度或放行未知字段 | 红=spot_public_parsers | 绿=spot_public_parsers
// TDD-PROBE: parse::spot::parse_spot_trading_day | 变异：绕过结构校验或误拒合法响应 | 红=spot_public_parsers | 绿=spot_public_parsers
// TDD-PROBE: parse::spot::parse_spot_reference_price | 变异：未知字段放行 | 红=forms_preserve_object_and_array_unknown_field_errors | 绿=forms_preserve_object_and_array_unknown_field_errors
// TDD-PROBE: parse::spot::parse_spot_reference_price_calculation | 变异：未知字段放行 | 红=forms_preserve_object_and_array_unknown_field_errors | 绿=forms_preserve_object_and_array_unknown_field_errors
// TDD-PROBE: parse::usdm::parse_usdm_exchange_info | 变异：绕过结构校验或误拒合法响应 | 红=usdm_public_parsers | 绿=usdm_public_parsers
// TDD-PROBE: parse::usdm::parse_usdm_index_info | 变异：绕过结构校验或误拒合法响应 | 红=usdm_public_parsers | 绿=usdm_public_parsers
// TDD-PROBE: parse::usdm::parse_usdm_agg_trade | 变异：绕过结构校验或误拒合法响应 | 红=usdm_public_parsers | 绿=usdm_public_parsers
// TDD-PROBE: parse::usdm::parse_usdm_trade | 变异：绕过结构校验或误拒合法响应 | 红=usdm_public_parsers | 绿=usdm_public_parsers
// TDD-PROBE: parse::usdm::parse_usdm_continuous_kline | 变异：绕过结构校验或误拒合法响应 | 红=usdm_public_parsers | 绿=usdm_public_parsers
// TDD-PROBE: parse::usdm::parse_usdm_kline | 变异：绕过结构校验或误拒合法响应 | 红=usdm_public_parsers | 绿=usdm_public_parsers
// TDD-PROBE: parse::usdm::parse_usdm_index_price_kline | 变异：绕过结构校验或误拒合法响应 | 红=usdm_public_parsers | 绿=usdm_public_parsers
// TDD-PROBE: parse::usdm::parse_usdm_mark_price_kline | 变异：绕过结构校验或误拒合法响应 | 红=usdm_public_parsers | 绿=usdm_public_parsers
// TDD-PROBE: parse::usdm::parse_usdm_premium_index | 变异：绕过结构校验或误拒合法响应 | 红=usdm_public_parsers | 绿=usdm_public_parsers
// TDD-PROBE: parse::usdm::parse_usdm_premium_index_kline | 变异：绕过结构校验或误拒合法响应 | 红=usdm_public_parsers | 绿=usdm_public_parsers
// TDD-PROBE: parse::usdm::parse_usdm_funding_rate | 变异：放行缺失 symbol/fundingTime 或重复身份三元组 | 红=usdm_funding_rate_requires_unique_local_identities_per_response | 绿=usdm_funding_rate_requires_unique_local_identities_per_response
// TDD-PROBE: parse::usdm::parse_usdm_funding_info | 变异：放行缺失或批内重复 symbol | 红=usdm_funding_info_requires_unique_local_symbols_per_response | 绿=usdm_funding_info_requires_unique_local_symbols_per_response
// TDD-PROBE: parse::usdm::parse_usdm_open_interest | 变异：放行缺失／空白 symbol 或缺失 time | 红=usdm_open_interest_requires_local_symbol_and_time | 绿=usdm_open_interest_requires_local_symbol_and_time
// TDD-PROBE: parse::usdm::parse_usdm_open_interest_hist | 变异：放行缺失 symbol/timestamp 或重复组合 | 红=usdm_open_interest_hist_requires_unique_local_symbol_times_per_response | 绿=usdm_open_interest_hist_requires_unique_local_symbol_times_per_response
// TDD-PROBE: parse::usdm::parse_usdm_book_snapshot | 变异：绕过结构校验或误拒合法响应 | 红=usdm_public_parsers | 绿=usdm_public_parsers
// TDD-PROBE: parse::usdm::parse_usdm_book_ticker | 变异：绕过结构校验或误拒合法响应 | 红=usdm_public_parsers | 绿=usdm_public_parsers
// TDD-PROBE: parse::usdm::parse_usdm_ticker_price | 变异：绕过结构校验或误拒合法响应 | 红=usdm_public_parsers | 绿=usdm_public_parsers
// TDD-PROBE: parse::usdm::parse_usdm_ticker24hr | 变异：绕过结构校验或误拒合法响应 | 红=usdm_public_parsers | 绿=usdm_public_parsers
// TDD-PROBE: parse::usdm::parse_usdm_taker_long_short_ratio | 变异：绕过结构校验或误拒合法响应 | 红=usdm_public_parsers | 绿=usdm_public_parsers
// TDD-PROBE: parse::usdm::parse_usdm_global_long_short_account_ratio | 变异：绕过结构校验或误拒合法响应 | 红=usdm_public_parsers | 绿=usdm_public_parsers
// TDD-PROBE: parse::usdm::parse_usdm_top_long_short_account_ratio | 变异：绕过结构校验或误拒合法响应 | 红=usdm_public_parsers | 绿=usdm_public_parsers
// TDD-PROBE: parse::usdm::parse_usdm_top_long_short_position_ratio | 变异：绕过结构校验或误拒合法响应 | 红=usdm_public_parsers | 绿=usdm_public_parsers
// TDD-PROBE: parse::usdm::parse_usdm_asset_index | 变异：绕过结构校验或误拒合法响应 | 红=usdm_public_parsers | 绿=usdm_public_parsers
// TDD-PROBE: parse::usdm::parse_usdm_constituents | 变异：绕过结构校验或误拒合法响应 | 红=usdm_public_parsers | 绿=usdm_public_parsers
// TDD-PROBE: parse::usdm::parse_usdm_insurance_balance | 变异：绕过结构校验或误拒合法响应 | 红=usdm_public_parsers | 绿=usdm_public_parsers
// TDD-PROBE: parse::usdm::parse_usdm_basis | 变异：绕过结构校验或误拒合法响应 | 红=usdm_public_parsers | 绿=usdm_public_parsers
// TDD-PROBE: parse::usdm::parse_usdm_delivery_price | 变异：绕过结构校验或误拒合法响应 | 红=usdm_public_parsers | 绿=usdm_public_parsers
// TDD-PROBE: parse::usdm::parse_usdm_trading_schedule | 变异：绕过结构校验或误拒合法响应 | 红=usdm_public_parsers | 绿=usdm_public_parsers
// TDD-PROBE: parse::usdm::parse_usdm_convert_exchange_info | 变异：绕过结构校验或误拒合法响应 | 红=usdm_public_parsers | 绿=usdm_public_parsers
// TDD-PROBE: parse::usdm::parse_usdm_adl_risk | 变异：绕过结构校验或误拒合法响应 | 红=usdm_public_parsers | 绿=usdm_public_parsers
// TDD-PROBE: parse::coinm::parse_coinm_exchange_info | 变异：绕过结构校验或误拒合法响应 | 红=coinm_public_parsers | 绿=coinm_public_parsers
// TDD-PROBE: parse::coinm::parse_coinm_agg_trade | 变异：绕过结构校验或误拒合法响应 | 红=coinm_public_parsers | 绿=coinm_public_parsers
// TDD-PROBE: parse::coinm::parse_coinm_trade | 变异：绕过结构校验或误拒合法响应 | 红=coinm_public_parsers | 绿=coinm_public_parsers
// TDD-PROBE: parse::coinm::parse_coinm_continuous_kline | 变异：绕过结构校验或误拒合法响应 | 红=coinm_public_parsers | 绿=coinm_public_parsers
// TDD-PROBE: parse::coinm::parse_coinm_kline | 变异：绕过结构校验或误拒合法响应 | 红=coinm_public_parsers | 绿=coinm_public_parsers
// TDD-PROBE: parse::coinm::parse_coinm_index_price_kline | 变异：绕过结构校验或误拒合法响应 | 红=coinm_public_parsers | 绿=coinm_public_parsers
// TDD-PROBE: parse::coinm::parse_coinm_mark_price_kline | 变异：绕过结构校验或误拒合法响应 | 红=coinm_public_parsers | 绿=coinm_public_parsers
// TDD-PROBE: parse::coinm::parse_coinm_premium_index | 变异：绕过结构校验或误拒合法响应 | 红=coinm_public_parsers | 绿=coinm_public_parsers
// TDD-PROBE: parse::coinm::parse_coinm_premium_index_kline | 变异：绕过结构校验或误拒合法响应 | 红=coinm_public_parsers | 绿=coinm_public_parsers
// TDD-PROBE: parse::coinm::parse_coinm_funding_rate | 变异：放行缺失 symbol/fundingTime 或批内重复组合 | 红=coinm_funding_rate_requires_unique_local_symbol_times_per_response | 绿=coinm_funding_rate_requires_unique_local_symbol_times_per_response
// TDD-PROBE: parse::coinm::parse_coinm_funding_info | 变异：放行缺失或批内重复 symbol | 红=coinm_funding_info_requires_unique_local_symbols_per_response | 绿=coinm_funding_info_requires_unique_local_symbols_per_response
// TDD-PROBE: parse::coinm::parse_coinm_open_interest | 变异：绕过结构校验或误拒合法响应 | 红=coinm_public_parsers | 绿=coinm_public_parsers
// TDD-PROBE: parse::coinm::parse_coinm_open_interest_hist | 变异：放行缺失 pair/contractType/timestamp 或重复组合 | 红=coinm_open_interest_hist_requires_unique_local_pair_type_times_per_response | 绿=coinm_open_interest_hist_requires_unique_local_pair_type_times_per_response
// TDD-PROBE: parse::coinm::parse_coinm_book_ticker | 变异：绕过结构校验或误拒合法响应 | 红=coinm_public_parsers | 绿=coinm_public_parsers
// TDD-PROBE: parse::coinm::parse_coinm_book_snapshot | 变异：误拒 tuple 深度或放行未知字段 | 红=coinm_public_parsers | 绿=coinm_public_parsers
// TDD-PROBE: parse::coinm::parse_coinm_ticker_price | 变异：绕过结构校验或误拒合法响应 | 红=coinm_public_parsers | 绿=coinm_public_parsers
// TDD-PROBE: parse::coinm::parse_coinm_ticker24hr | 变异：绕过结构校验或误拒合法响应 | 红=coinm_public_parsers | 绿=coinm_public_parsers
// TDD-PROBE: parse::coinm::parse_coinm_taker_buy_sell_vol | 变异：绕过结构校验或误拒合法响应 | 红=coinm_public_parsers | 绿=coinm_public_parsers
// TDD-PROBE: parse::coinm::parse_coinm_global_long_short_account_ratio | 变异：绕过结构校验或误拒合法响应 | 红=coinm_public_parsers | 绿=coinm_public_parsers
// TDD-PROBE: parse::coinm::parse_coinm_top_long_short_account_ratio | 变异：绕过结构校验或误拒合法响应 | 红=coinm_public_parsers | 绿=coinm_public_parsers
// TDD-PROBE: parse::coinm::parse_coinm_top_long_short_position_ratio | 变异：绕过结构校验或误拒合法响应 | 红=coinm_public_parsers | 绿=coinm_public_parsers
// TDD-PROBE: parse::coinm::parse_coinm_basis | 变异：绕过结构校验或误拒合法响应 | 红=coinm_public_parsers | 绿=coinm_public_parsers
// TDD-PROBE: parse::coinm::parse_coinm_constituents | 变异：绕过结构校验或误拒合法响应 | 红=coinm_public_parsers | 绿=coinm_public_parsers
// TDD-PROBE: parse::options::parse_options_exchange_info | 变异：绕过结构校验或误拒合法响应 | 红=options_public_parsers | 绿=options_public_parsers
// TDD-PROBE: parse::options::parse_options_exercise_history | 变异：绕过结构校验或误拒合法响应 | 红=options_public_parsers | 绿=options_public_parsers
// TDD-PROBE: parse::options::parse_options_index | 变异：绕过结构校验或误拒合法响应 | 红=options_public_parsers | 绿=options_public_parsers
// TDD-PROBE: parse::options::parse_options_kline | 变异：绕过结构校验或误拒合法响应 | 红=options_public_parsers | 绿=options_public_parsers
// TDD-PROBE: parse::options::parse_options_open_interest | 变异：绕过结构校验或误拒合法响应 | 红=options_public_parsers | 绿=options_public_parsers
// TDD-PROBE: parse::options::parse_options_mark | 变异：绕过结构校验或误拒合法响应 | 红=options_public_parsers | 绿=options_public_parsers
// TDD-PROBE: parse::options::parse_options_block_trade | 变异：放行缺失或同 symbol 内重复 id | 红=options_block_trade_uses_symbol_scoped_id | 绿=options_block_trade_uses_symbol_scoped_id
// TDD-PROBE: parse::options::parse_options_trade | 变异：放行缺失或同 symbol 内重复 tradeId | 红=options_trade_uses_symbol_scoped_trade_id | 绿=options_trade_uses_symbol_scoped_trade_id
// TDD-PROBE: parse::options::parse_options_ticker | 变异：绕过结构校验或误拒合法响应 | 红=options_public_parsers | 绿=options_public_parsers
// TDD-PROBE: parse::options::parse_options_book_snapshot | 变异：误拒 tuple 深度或放行未知字段 | 红=options_public_parsers | 绿=options_public_parsers
// TDD-PROBE: parse::parse_exchange_info | 变异：快照哈希恒空 | 红=whitelist_snapshot_preserves_provenance | 绿=whitelist_snapshot_preserves_provenance
// TDD-PROBE: parse::find_unknown_fields | 变异：漏报未登记键 | 红=unknown_field_helpers_are_atomic | 绿=unknown_field_helpers_are_atomic
// TDD-PROBE: parse::reject_unknown_fields | 变异：未知字段放行 | 红=unknown_field_helpers_are_atomic | 绿=unknown_field_helpers_are_atomic
// TDD-PROBE: registered_evidence | 变异：接受自带哈希 | 红=authorization_requires_valid_consistent_evidence | 绿=authorization_requires_valid_consistent_evidence
// TDD-PROBE: current_authorization | 变异：空证据放行 | 红=authorization_requires_valid_consistent_evidence | 绿=authorization_requires_valid_consistent_evidence
// TDD-PROBE: Decimal::new | 变异：跳过十进制词法校验 | 红=decimal_preserves_lexical_value | 绿=decimal_preserves_lexical_value
// TDD-PROBE: value::validate_decimal | 变异：跳过十进制词法校验 | 红=public_decimal_validator_checks_complete_lexemes | 绿=public_decimal_validator_checks_complete_lexemes
// TDD-PROBE: Quantity::new | 变异：负数量取绝对值 | 红=quantity_preserves_sign_and_unit | 绿=quantity_preserves_sign_and_unit
// TDD-PROBE: Date::new | 变异：取消闰年校验 | 红=calendar_and_interval_validate_boundaries | 绿=calendar_and_interval_validate_boundaries
// TDD-PROBE: Interval::new | 变异：接受空周期 | 红=calendar_and_interval_validate_boundaries | 绿=calendar_and_interval_validate_boundaries
// TDD-PROBE: EndpointId::new | 变异：丢弃端点版本 | 红=identities_keep_route_and_market_separate | 绿=identities_keep_route_and_market_separate
// TDD-PROBE: DataSeriesId::new | 变异：用路由族覆盖市场 | 红=identities_keep_route_and_market_separate | 绿=identities_keep_route_and_market_separate
// TDD-PROBE: value::validate_sha256_hex | 变异：跳过长度或字符校验 | 红=hash_validation_rejects_malformed_values | 绿=hash_validation_rejects_malformed_values

use binancex::parse::{self, coinm::*, options::*, spot::*, usdm::*, WhitelistObservation};
use binancex::value::Instrument;
use binancex::{
    current_authorization, registered_evidence, BinanceErrorKind, DataSeriesId, Date, Decimal,
    EndpointId, Interval, Quantity, QuantityUnit, Sign,
};
use serde_json::{json, Value};

// 根据冻结树手工选取最小非空字段；没有推断尚未冻结的字段必填集合。
// 合成 JSON 包装中的元数据仅供测试识别，不传入严格 wire 解析器。
fn payload(raw: &str) -> String {
    let fixture = json!({
        "_synthetic": true,
        "_note": "合成契约探针，不是源观测数据",
        "payload": serde_json::from_str::<Value>(raw).unwrap()
    });
    assert_eq!(fixture["_synthetic"], true);
    assert!(!fixture["_note"].as_str().unwrap().is_empty());
    fixture["payload"].to_string()
}

// 在有效批次末尾加入非法行，验证解析器不会返回已经解析成功的前缀。
fn invalid_tail(raw: &str) -> (String, BinanceErrorKind) {
    let mut value: Value = serde_json::from_str(raw).unwrap();
    let target = match &mut value {
        Value::Array(rows) => {
            rows.push(rows[0].clone());
            rows.last_mut().unwrap()
        }
        _ => &mut value,
    };
    let expected = match target {
        Value::Object(object) => {
            object.insert("unregisteredField".into(), json!(true));
            BinanceErrorKind::UnknownField
        }
        Value::Array(tuple) => {
            tuple.pop();
            BinanceErrorKind::SchemaMismatch
        }
        _ => unreachable!("夹具根只能是对象或对象/元组数组"),
    };
    (value.to_string(), expected)
}

macro_rules! probe {
    ($parser:path, $raw:literal) => {{
        let valid = payload($raw);
        assert!(
            $parser(&valid).is_ok(),
            "合法合成响应被拒绝：{}",
            stringify!($parser)
        );
        assert_eq!(
            $parser(&valid[..valid.len() - 1]).unwrap_err().kind(),
            BinanceErrorKind::Invalid,
            "非法 JSON 分类：{}",
            stringify!($parser)
        );
        let (invalid, expected) = invalid_tail(&valid);
        assert_eq!(
            $parser(&invalid).unwrap_err().kind(),
            expected,
            "批次原子拒绝：{}",
            stringify!($parser)
        );
        assert!(
            $parser("null").is_err(),
            "空根被错误接受：{}",
            stringify!($parser)
        );
    }};
}

#[test]
fn spot_public_parsers() {
    probe!(parse_spot_exchange_info, r#"{"timezone":"1"}"#);
    probe!(
        parse_spot_execution_rules,
        r#"{"symbolRules":[{"symbol":"1"}]}"#
    );
    probe!(parse_spot_agg_trade, r#"[{"a":1}]"#);
    probe!(
        parse_spot_avg_price,
        r#"{"mins":1,"price":"1.00","closeTime":1}"#
    );
    probe!(parse_spot_trade, r#"[{"id":1}]"#);
    probe!(parse_spot_block_trade, r#"[{"id":1}]"#);
    probe!(
        parse_spot_kline,
        r#"[[1,"1","1","1","1","1",1,"1",1,"1","1","1"]]"#
    );
    probe!(
        parse_spot_ui_kline,
        r#"[[1,"1","1","1","1","1",1,"1",1,"1","1","1"]]"#
    );
    probe!(parse_spot_ticker, r#"{"symbol":"1"}"#);
    probe!(parse_spot_ticker24hr, r#"{"symbol":"1"}"#);
    probe!(parse_spot_ticker_price, r#"{"symbol":"1","price":"1.00"}"#);
    probe!(parse_spot_book_ticker, r#"{"symbol":"1"}"#);
    probe!(
        parse_spot_book_snapshot,
        r#"{"lastUpdateId":1,"bids":[["1","2"]],"asks":[]}"#
    );
    probe!(parse_spot_trading_day, r#"{"symbol":"1"}"#);
    probe!(parse_spot_reference_price, r#"{"symbol":"1"}"#);
    probe!(
        parse_spot_reference_price_calculation,
        r#"{"symbol":"1","calculationType":"EXTERNAL","externalCalculationId":1}"#
    );
}

#[test]
fn usdm_public_parsers() {
    probe!(parse_usdm_exchange_info, r#"{"exchangeFilters":["1"]}"#);
    probe!(parse_usdm_index_info, r#"[{"symbol":"1"}]"#);
    probe!(parse_usdm_agg_trade, r#"[{"a":1}]"#);
    probe!(parse_usdm_trade, r#"[{"id":1}]"#);
    probe!(
        parse_usdm_continuous_kline,
        r#"[[1,"1","1","1","1","1",1,"1",1,"1","1","1"]]"#
    );
    probe!(
        parse_usdm_kline,
        r#"[[1,"1","1","1","1","1",1,"1",1,"1","1","1"]]"#
    );
    probe!(
        parse_usdm_index_price_kline,
        r#"[[1,"1","1","1","1","1",1,"1",1,"1","1","1"]]"#
    );
    probe!(
        parse_usdm_mark_price_kline,
        r#"[[1,"1","1","1","1","1",1,"1",1,"1","1","1"]]"#
    );
    probe!(parse_usdm_premium_index, r#"{"symbol":"1"}"#);
    probe!(
        parse_usdm_premium_index_kline,
        r#"[[1,"1","1","1","1","1",1,"1",1,"1","1","1"]]"#
    );
    probe!(
        parse_usdm_funding_rate,
        r#"[{"symbol":"BTCUSDT","fundingTime":1,"rateType":"Regular"}]"#
    );
    probe!(parse_usdm_funding_info, r#"[{"symbol":"BLZUSDT"}]"#);
    probe!(
        parse_usdm_open_interest,
        r#"{"openInterest":"1","symbol":"BTCUSDT","time":1}"#
    );
    probe!(
        parse_usdm_open_interest_hist,
        r#"[{"symbol":"BTCUSDT","timestamp":1}]"#
    );
    probe!(parse_usdm_book_snapshot, r#"{"lastUpdateId":1}"#);
    probe!(parse_usdm_book_ticker, r#"{"symbol":"1"}"#);
    probe!(parse_usdm_ticker_price, r#"{"symbol":"1"}"#);
    probe!(parse_usdm_ticker24hr, r#"{"symbol":"1"}"#);
    probe!(
        parse_usdm_taker_long_short_ratio,
        r#"[{"buySellRatio":"1"}]"#
    );
    probe!(
        parse_usdm_global_long_short_account_ratio,
        r#"[{"symbol":"1"}]"#
    );
    probe!(
        parse_usdm_top_long_short_account_ratio,
        r#"[{"symbol":"1"}]"#
    );
    probe!(
        parse_usdm_top_long_short_position_ratio,
        r#"[{"symbol":"1"}]"#
    );
    probe!(parse_usdm_asset_index, r#"{"symbol":"1"}"#);
    probe!(parse_usdm_constituents, r#"{"symbol":"1"}"#);
    probe!(parse_usdm_insurance_balance, r#"{"symbols":["1"]}"#);
    probe!(parse_usdm_basis, r#"[{"indexPrice":"1"}]"#);
    probe!(parse_usdm_delivery_price, r#"[{"deliveryTime":1}]"#);
    probe!(parse_usdm_trading_schedule, r#"{"updateTime":1}"#);
    probe!(parse_usdm_convert_exchange_info, r#"[{"fromAsset":"1"}]"#);
    probe!(parse_usdm_adl_risk, r#"{"symbol":"1"}"#);
}

#[test]
fn coinm_public_parsers() {
    probe!(parse_coinm_exchange_info, r#"{"exchangeFilters":["1"]}"#);
    probe!(parse_coinm_agg_trade, r#"[{"a":1}]"#);
    probe!(parse_coinm_trade, r#"[{"id":1}]"#);
    probe!(
        parse_coinm_continuous_kline,
        r#"[[1,"1","1","1","1","1",1,"1",1,"1","1","1"]]"#
    );
    probe!(
        parse_coinm_kline,
        r#"[[1,"1","1","1","1","1",1,"1",1,"1","1","1"]]"#
    );
    probe!(
        parse_coinm_index_price_kline,
        r#"[[1,"1","1","1","1","1",1,"1",1,"1","1","1"]]"#
    );
    probe!(
        parse_coinm_mark_price_kline,
        r#"[[1,"1","1","1","1","1",1,"1",1,"1","1","1"]]"#
    );
    probe!(parse_coinm_premium_index, r#"[{"symbol":"1"}]"#);
    probe!(
        parse_coinm_premium_index_kline,
        r#"[[1,"1","1","1","1","1",1,"1",1,"1","1","1"]]"#
    );
    probe!(
        parse_coinm_funding_rate,
        r#"[{"symbol":"BTCUSD_PERP","fundingTime":1}]"#
    );
    probe!(parse_coinm_funding_info, r#"[{"symbol":"BTCUSD_PERP"}]"#);
    probe!(parse_coinm_open_interest, r#"{"symbol":"1"}"#);
    probe!(
        parse_coinm_open_interest_hist,
        r#"[{"pair":"BTCUSD","contractType":"PERPETUAL","timestamp":1}]"#
    );
    probe!(parse_coinm_book_ticker, r#"[{"lastUpdateId":1}]"#);
    probe!(
        parse_coinm_book_snapshot,
        r#"{"lastUpdateId":1,"bids":[["1","2"]],"asks":[]}"#
    );
    probe!(parse_coinm_ticker_price, r#"[{"symbol":"1"}]"#);
    probe!(parse_coinm_ticker24hr, r#"[{"symbol":"1"}]"#);
    probe!(parse_coinm_taker_buy_sell_vol, r#"[{"pair":"1"}]"#);
    probe!(
        parse_coinm_global_long_short_account_ratio,
        r#"[{"pair":"1"}]"#
    );
    probe!(
        parse_coinm_top_long_short_account_ratio,
        r#"[{"pair":"1"}]"#
    );
    probe!(
        parse_coinm_top_long_short_position_ratio,
        r#"[{"pair":"1"}]"#
    );
    probe!(parse_coinm_basis, r#"[{"indexPrice":"1"}]"#);
    probe!(parse_coinm_constituents, r#"{"symbol":"1"}"#);
}

#[test]
fn options_public_parsers() {
    probe!(parse_options_exchange_info, r#"{"timezone":"1"}"#);
    probe!(parse_options_exercise_history, r#"[{"symbol":"1"}]"#);
    probe!(parse_options_index, r#"{"time":1}"#);
    probe!(
        parse_options_kline,
        r#"[[1,"1","1","1","1","1",1,"1",1,"1","1","1"]]"#
    );
    probe!(parse_options_open_interest, r#"[{"symbol":"1"}]"#);
    probe!(parse_options_mark, r#"[{"symbol":"1"}]"#);
    probe!(parse_options_block_trade, r#"[{"symbol":"SYNTH","id":1}]"#);
    probe!(parse_options_trade, r#"[{"symbol":"SYNTH","tradeId":1}]"#);
    probe!(parse_options_ticker, r#"[{"symbol":"1"}]"#);
    probe!(
        parse_options_book_snapshot,
        r#"{"lastUpdateId":1,"bids":[["1","2"]],"asks":[]}"#
    );
}

#[test]
fn map_only_and_empty_objects_fail_closed() {
    assert_eq!(
        parse_spot_avg_price("[1]").unwrap_err().kind(),
        BinanceErrorKind::SchemaMismatch
    );
    assert_eq!(
        parse_spot_exchange_info(r#"{"symbols":[["BTCUSDT"]]}"#)
            .unwrap_err()
            .kind(),
        BinanceErrorKind::SchemaMismatch
    );
    assert_eq!(
        parse_usdm_exchange_info(r#"{"assets":[["USDT"]]}"#)
            .unwrap_err()
            .kind(),
        BinanceErrorKind::SchemaMismatch
    );
    assert_eq!(
        parse_coinm_exchange_info(r#"{"symbols":[["BTCUSD"]]}"#)
            .unwrap_err()
            .kind(),
        BinanceErrorKind::SchemaMismatch
    );
    assert_eq!(
        parse_options_exchange_info(r#"{"optionAssets":[["USDT"]]}"#)
            .unwrap_err()
            .kind(),
        BinanceErrorKind::SchemaMismatch
    );
    assert_eq!(
        parse_spot_avg_price("{}").unwrap_err().kind(),
        BinanceErrorKind::Missing
    );
    assert_eq!(
        parse_spot_trade("[{}]").unwrap_err().kind(),
        BinanceErrorKind::SchemaMismatch
    );
    assert_eq!(
        parse_options_exchange_info(r#"{"optionAssets":[{}]}"#)
            .unwrap_err()
            .kind(),
        BinanceErrorKind::SchemaMismatch
    );
}

#[test]
fn forms_preserve_object_and_array_unknown_field_errors() {
    macro_rules! form {
        ($($parser:path),+ $(,)?) => {$(
            assert!($parser(r#"{"symbol":"SYNTH"}"#).is_ok());
            assert!($parser(r#"[{"symbol":"SYNTH"}]"#).is_ok());
            assert_eq!($parser(r#"{"symbol":"SYNTH","newField":1}"#).unwrap_err().kind(), BinanceErrorKind::UnknownField);
            assert_eq!($parser(r#"[{"symbol":"SYNTH"},{"symbol":"SYNTH","newField":1}]"#).unwrap_err().kind(), BinanceErrorKind::UnknownField);
        )+};
    }
    form!(
        parse_spot_ticker,
        parse_spot_ticker24hr,
        parse_spot_book_ticker,
        parse_spot_trading_day,
        parse_usdm_premium_index,
        parse_usdm_book_ticker,
        parse_usdm_ticker_price,
        parse_usdm_ticker24hr,
        parse_usdm_asset_index,
        parse_usdm_adl_risk,
    );
    assert!(parse_spot_ticker_price(r#"{"symbol":"SYNTH","price":"1.00"}"#).is_ok());
    assert!(parse_spot_ticker_price(r#"[{"symbol":"SYNTH","price":"1.00"}]"#).is_ok());
    assert_eq!(
        parse_spot_ticker_price(r#"{"symbol":"SYNTH","price":"1.00","newField":1}"#)
            .unwrap_err()
            .kind(),
        BinanceErrorKind::UnknownField
    );
    assert_eq!(
        parse_spot_ticker_price(
            r#"[{"symbol":"SYNTH","price":"1.00"},{"symbol":"SYNTH","price":"1.00","newField":1}]"#
        )
        .unwrap_err()
        .kind(),
        BinanceErrorKind::UnknownField
    );
    assert!(parse_usdm_insurance_balance(r#"{"symbols":["SYNTH"]}"#).is_ok());
    assert!(parse_usdm_insurance_balance(r#"[{"symbols":["SYNTH"]}]"#).is_ok());
    assert_eq!(
        parse_usdm_insurance_balance(r#"[{"symbols":["SYNTH"],"assets":[{"newField":1}]}]"#)
            .unwrap_err()
            .kind(),
        BinanceErrorKind::UnknownField
    );
    assert_eq!(
        parse_spot_reference_price(r#"{"symbol":"SYNTH","newField":1}"#)
            .unwrap_err()
            .kind(),
        BinanceErrorKind::UnknownField
    );
    assert_eq!(
        parse_spot_reference_price_calculation(r#"{"symbol":"SYNTH","newField":1}"#)
            .unwrap_err()
            .kind(),
        BinanceErrorKind::UnknownField
    );
}

#[test]
fn twelve_kline_entries_reject_duplicate_identity_and_wrong_lengths() {
    // 合同证据只覆盖这十二类；不对 SpotUiKline 自行推断唯一身份。
    type KlineParser = fn(&str) -> binancex::BinanceResult<Vec<binancex::value::KlineRow>>;
    let parsers: [KlineParser; 12] = [
        |input| parse_spot_kline(input).map(|rows| rows.0),
        |input| parse_usdm_continuous_kline(input).map(|rows| rows.0),
        |input| parse_usdm_kline(input).map(|rows| rows.0),
        |input| parse_usdm_index_price_kline(input).map(|rows| rows.0),
        |input| parse_usdm_mark_price_kline(input).map(|rows| rows.0),
        |input| parse_usdm_premium_index_kline(input).map(|rows| rows.0),
        |input| parse_coinm_continuous_kline(input).map(|rows| rows.0),
        |input| parse_coinm_kline(input).map(|rows| rows.0),
        |input| parse_coinm_index_price_kline(input).map(|rows| rows.0),
        |input| parse_coinm_mark_price_kline(input).map(|rows| rows.0),
        |input| parse_coinm_premium_index_kline(input).map(|rows| rows.0),
        |input| parse_options_kline(input).map(|rows| rows.0),
    ];
    let first = json!([1, "1", "1", "1", "1", "1", 2, "1", 1, "1", "1", "0"]);
    let changed_same_time = json!([1, "9", "9", "9", "9", "9", 3, "9", 9, "9", "9", "0"]);
    let different_time = json!([2, "1", "1", "1", "1", "1", 3, "1", 1, "1", "1", "0"]);
    for parser in parsers {
        assert_eq!(
            parser(&json!([first, changed_same_time]).to_string())
                .unwrap_err()
                .kind(),
            BinanceErrorKind::IdentityConflict
        );
        assert_eq!(
            parser(&json!([first, different_time]).to_string())
                .unwrap()
                .len(),
            2
        );
        let mut short = first.as_array().unwrap().clone();
        short.pop();
        assert_eq!(
            parser(&json!([short]).to_string()).unwrap_err().kind(),
            BinanceErrorKind::SchemaMismatch
        );
        let mut long = first.as_array().unwrap().clone();
        long.push(json!("extra"));
        assert!(parser(&json!([long]).to_string()).is_err());
    }
}

#[test]
fn trading_schedule_requires_evidence_for_new_market_keys() {
    let mut markets = serde_json::Map::new();
    for key in [
        "EQUITY",
        "COMMODITY",
        "KR_EQUITY",
        "HK_EQUITY",
        "CN_EQUITY",
        "FX",
    ] {
        markets.insert(
            key.into(),
            json!({"sessions":[{"startTime":1,"endTime":2,"type":"SYNTH"}]}),
        );
    }
    let parsed =
        parse_usdm_trading_schedule(&json!({"marketSchedules":markets}).to_string()).unwrap();
    assert_eq!(parsed.market_schedules.unwrap().len(), 6);
    markets.insert("UNVERIFIED_MARKET".into(), json!({"sessions":[]}));
    assert_eq!(
        parse_usdm_trading_schedule(&json!({"marketSchedules":markets}).to_string())
            .unwrap_err()
            .kind(),
        BinanceErrorKind::UnknownField
    );
}

#[test]
fn decimal_preserves_lexical_value() {
    for raw in [
        "0",
        "-0",
        "-0.000e-100",
        "123456789012345678901234567890.012300",
        "1e9999",
        "+.5",
        "1.",
    ] {
        assert_eq!(Decimal::new(raw).unwrap().as_str(), raw);
    }
    for raw in [
        "", "-", ".", "1e", "1e+", "1e2e3", "1.2.3", "NaN", "inf", " 1", "1 ", "١",
    ] {
        assert_eq!(
            Decimal::new(raw).unwrap_err().kind(),
            BinanceErrorKind::Invalid,
            "{raw}"
        );
    }
    assert_eq!(Decimal::new("-0.000e-100").unwrap().sign(), Sign::Zero);
    assert_eq!(Decimal::new("-0.001").unwrap().sign(), Sign::Negative);
}

#[test]
fn public_decimal_validator_checks_complete_lexemes() {
    assert_eq!(
        binancex::value::validate_decimal("+").unwrap_err().kind(),
        BinanceErrorKind::Invalid
    );
    assert!(binancex::value::validate_decimal("-1.25e+3").is_ok());
}

#[test]
fn quantity_preserves_sign_and_unit() {
    let negative = Quantity::new(Decimal::new("-3.25").unwrap(), QuantityUnit::Contracts);
    assert_eq!(negative.value().as_str(), "-3.25");
    assert_eq!(negative.unit(), QuantityUnit::Contracts);
    assert_eq!(negative.sign(), Sign::Negative);
    let zero = Quantity::new(Decimal::new("-0").unwrap(), QuantityUnit::QuoteAsset);
    assert_eq!(zero.value().as_str(), "-0");
    assert_eq!(zero.sign(), Sign::Zero);
    assert!(Decimal::new("not-a-number").is_err());
}

#[test]
fn numeric_and_syntax_errors_keep_distinct_categories() {
    let precise = parse_spot_trade(r#"[{"id":9007199254740993}]"#).unwrap();
    assert_eq!(precise[0].id, Some(9_007_199_254_740_993));
    for raw in [
        r#"[{"id":9223372036854775808}]"#,
        r#"[{"id":-9223372036854775809}]"#,
        r#"[{"id":1.5}]"#,
    ] {
        assert_eq!(
            parse_spot_trade(raw).unwrap_err().kind(),
            BinanceErrorKind::LossyNumeric
        );
    }
    assert_eq!(
        parse_spot_trade(r#"[{"id":"1"}]"#).unwrap_err().kind(),
        BinanceErrorKind::SchemaMismatch
    );
    assert_eq!(
        parse_spot_trade(r#"[{"id":1}"#).unwrap_err().kind(),
        BinanceErrorKind::Invalid
    );
    let delivery = parse_usdm_delivery_price(
        r#"[{"deliveryTime":1,"deliveryPrice":123456789012345678901234567890.123456789}]"#,
    )
    .unwrap();
    assert_eq!(
        delivery[0].delivery_price.as_ref().unwrap().to_string(),
        "123456789012345678901234567890.123456789"
    );
}

#[test]
fn reference_price_missing_is_not_zero() {
    let absent = parse_spot_reference_price(r#"{"symbol":"SYNTH"}"#).unwrap();
    assert_eq!(absent.reference_price, None);
    let null = parse_spot_reference_price(r#"{"symbol":"SYNTH","referencePrice":null}"#).unwrap();
    assert_eq!(null.reference_price, None);
    let zero = parse_spot_reference_price(r#"{"symbol":"SYNTH","referencePrice":"-0"}"#).unwrap();
    assert_eq!(zero.reference_price.unwrap().as_str(), "-0");
    assert_eq!(
        parse_spot_reference_price(r#"{"referencePrice":0}"#)
            .unwrap_err()
            .kind(),
        BinanceErrorKind::SchemaMismatch
    );
}

#[test]
fn average_price_requires_selected_key_fields() {
    let complete = parse_spot_avg_price(r#"{"price":"1.20","closeTime":2}"#).unwrap();
    assert_eq!(complete.price.as_deref(), Some("1.20"));
    assert_eq!(complete.close_time, Some(2));
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
}

#[test]
fn spot_ticker_price_requires_symbol_and_price_for_each_item() {
    assert_eq!(
        parse_spot_ticker_price(r#"{}"#).unwrap_err().kind(),
        BinanceErrorKind::SchemaMismatch
    );
    for raw in [
        r#"{"symbol":"BTCUSDT"}"#,
        r#"{"price":"1.00"}"#,
        r#"{"symbol":null,"price":"1.00"}"#,
        r#"{"symbol":"BTCUSDT","price":null}"#,
        r#"{"symbol":"","price":"1.00"}"#,
        r#"{"symbol":"BTCUSDT","price":"   "}"#,
        r#"[{"symbol":"BTCUSDT","price":"1.00"},{"symbol":"ETHUSDT"}]"#,
    ] {
        assert_eq!(
            parse_spot_ticker_price(raw).unwrap_err().kind(),
            BinanceErrorKind::Missing,
            "价格响应关键字段错误分类不符：{raw}"
        );
    }
    assert!(parse_spot_ticker_price(
        r#"[{"symbol":"BTCUSDT","price":"1.00"},{"symbol":"ETHUSDT","price":"2.00"}]"#
    )
    .is_ok());
    assert!(parse_spot_ticker_price("[]").is_ok());
    assert_eq!(
        parse_spot_ticker_price(
            r#"[{"symbol":"BTCUSDT","price":"1.00"},{"symbol":"BTCUSDT","price":"2.00"}]"#
        )
        .unwrap_err()
        .kind(),
        BinanceErrorKind::IdentityConflict
    );
    assert_eq!(
        parse_spot_ticker_price(r#"{"symbol":"BTCUSDT","price":"not-a-price"}"#)
            .unwrap_err()
            .kind(),
        BinanceErrorKind::Invalid
    );
    assert_eq!(
        parse_spot_ticker_price(
            r#"[{"symbol":"","price":"1.00"},{"symbol":"ETHUSDT","price":"2.00","newField":1}]"#
        )
        .unwrap_err()
        .kind(),
        BinanceErrorKind::UnknownField
    );
    assert_eq!(
        parse_spot_ticker_price(r#"[null]"#).unwrap_err().kind(),
        BinanceErrorKind::SchemaMismatch
    );
}

#[test]
fn number_internal_tag_cannot_forge_delivery_price() {
    let forged =
        r#"[{"deliveryTime":1,"deliveryPrice":{"$serde_json::private::Number":"123.45"}}]"#;
    assert_eq!(
        parse_usdm_delivery_price(forged).unwrap_err().kind(),
        BinanceErrorKind::SchemaMismatch
    );
}

#[test]
fn whitelist_snapshot_preserves_provenance() {
    let observation = WhitelistObservation {
        family: "spot",
        raw: r#"{"symbols":[]}"#,
        source_endpoint_version: "/api/v3",
        observed_at_ms: 123,
    };
    let snapshot = parse::parse_exchange_info(observation.clone()).unwrap();
    assert_eq!(snapshot.family, "spot");
    assert_eq!(snapshot.source_endpoint_version, "/api/v3");
    assert_eq!(snapshot.observed_at_ms, 123);
    assert_eq!(snapshot.content_sha256.len(), 64);
    assert_eq!(
        snapshot,
        parse::parse_exchange_info(observation.clone()).unwrap()
    );
    let changed = parse::parse_exchange_info(WhitelistObservation {
        raw: "{\"symbols\": []}",
        ..observation.clone()
    })
    .unwrap();
    assert_ne!(snapshot.content_sha256, changed.content_sha256);
    assert!(parse::parse_exchange_info(WhitelistObservation {
        family: "",
        ..observation.clone()
    })
    .is_err());
    assert_eq!(
        parse::parse_exchange_info(WhitelistObservation {
            raw: "{",
            ..observation
        })
        .unwrap_err()
        .kind(),
        BinanceErrorKind::Invalid
    );
}

#[test]
fn unknown_field_helpers_are_atomic() {
    let object = json!({"known":1,"new":2});
    let unknown = parse::find_unknown_fields(object.as_object().unwrap(), &["known"]);
    assert_eq!(unknown, vec!["new"]);
    assert_eq!(
        parse::reject_unknown_fields(&unknown).unwrap_err().kind(),
        BinanceErrorKind::UnknownField
    );
    assert!(parse::reject_unknown_fields(&[]).is_ok());
    assert!(parse::find_unknown_fields(object.as_object().unwrap(), &["known", "new"]).is_empty());
}

#[test]
fn authorization_requires_valid_consistent_evidence() {
    let today = Date::new(2026, 9, 23).unwrap();
    assert!(!current_authorization(today, &[]).is_authorized());
    let evidence = |scope: &str, until: &str| {
        registered_evidence(
            &json!({
                "evidence_id":"SYNTH", "scope":scope, "signed_by":"synthetic-owner",
                "valid_from":"2026-01-01", "valid_until":until
            })
            .to_string(),
        )
        .unwrap()
    };
    let valid = evidence("offline:spot", "2026-12-31");
    assert_eq!(valid.scope(), "offline:spot");
    assert_eq!(valid.content_sha256().len(), 64);
    assert!(current_authorization(today, std::slice::from_ref(&valid)).is_authorized());
    assert!(
        !current_authorization(today, &[evidence("offline:spot", "2026-01-31")]).is_authorized()
    );
    let conflict = evidence("offline:usdm", "2026-12-31");
    assert!(!current_authorization(today, &[valid.clone(), conflict.clone()]).is_authorized());
    assert!(!current_authorization(today, &[conflict, valid]).is_authorized());
    assert_eq!(
        registered_evidence(r#"{"content_sha256":"forged"}"#)
            .unwrap_err()
            .kind(),
        BinanceErrorKind::UnknownField
    );
    assert_eq!(
        registered_evidence("{}").unwrap_err().kind(),
        BinanceErrorKind::Missing
    );
}

#[test]
fn calendar_and_interval_validate_boundaries() {
    assert!(Date::new(2000, 2, 29).is_ok());
    assert!(Date::new(1900, 2, 29).is_err());
    assert!(Date::new(2026, 2, 29).is_err());
    assert!(Date::new(2026, 0, 1).is_err());
    assert_eq!(Interval::new("1s").unwrap().as_str(), "1s");
    assert!(Interval::new("").is_err());
}

#[test]
fn identities_keep_route_and_market_separate() {
    let endpoint = EndpointId::new("binance", "usdm", "GET", "/fapi/v1/klines", "/fapi/v1");
    assert_eq!(endpoint.family(), "usdm");
    assert_eq!(endpoint.method(), "GET");
    assert_eq!(endpoint.path(), "/fapi/v1/klines");
    assert_eq!(endpoint.version(), "/fapi/v1");
    let alternate_route = EndpointId::new("binance", "coinm", "GET", "/dapi/v1/klines", "/dapi/v1");
    assert_ne!(endpoint, alternate_route);
    let instrument = Instrument::Symbol("SYNTH".into());
    // 合成等价路由样例，只验证来源字段不参与业务身份。
    let series = DataSeriesId::new("synthetic-market", instrument.clone(), "1m", "trade-kline");
    let equivalent_route_series =
        DataSeriesId::new("synthetic-market", instrument.clone(), "1m", "trade-kline");
    assert_eq!(series, equivalent_route_series);
    assert_eq!(series.market(), "synthetic-market");
    assert_eq!(
        series.entity(),
        &binancex::DataSeriesEntity::Instrument(instrument)
    );
}

#[test]
fn data_series_identity_includes_semantic_dimension() {
    let instrument = Instrument::Symbol("SYNTH".into());
    let klines = DataSeriesId::new("spot", instrument.clone(), "1m", "trade-kline");
    let ui_klines = DataSeriesId::new("spot", instrument, "1m", "ui-kline");
    assert_ne!(klines, ui_klines);
    assert_eq!(klines.variant(), "1m");
    assert_eq!(klines.semantic_dimension(), "trade-kline");
}

#[test]
fn hash_validation_rejects_malformed_values() {
    assert!(binancex::value::validate_sha256_hex(&"a".repeat(64)).is_ok());
    for raw in ["a".repeat(63), "a".repeat(65), "g".repeat(64)] {
        assert!(binancex::value::validate_sha256_hex(&raw).is_err());
    }
}
