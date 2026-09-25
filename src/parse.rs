//! 离线解析（无网络参数）。
//!
//! 入口形态为「字符串/字节 → 值对象集合」；未知字段原子失败。

use crate::error::{BinanceError, BinanceErrorKind, BinanceResult};
pub use crate::value::WhitelistSnapshot;
use serde::de::{DeserializeSeed, MapAccess, SeqAccess, Visitor};
use std::fmt;

pub mod coinm;
pub mod options;
pub mod spot;
pub mod usdm;

/// 反序列化冻结响应结构；已登记字段的对象必须为非空映射，未知字段原子失败。
pub(crate) fn deserialize_strict<T: serde::de::DeserializeOwned>(input: &str) -> BinanceResult<T> {
    let mut deserializer = serde_json::Deserializer::from_str(input);
    let value = T::deserialize(StrictDeserializer(&mut deserializer)).map_err(classify_error)?;
    deserializer.end().map_err(classify_error)?;
    Ok(value)
}

/// 本地关键字段策略：确认对象成员既存在又非 null。
///
/// 调用方先用具体类型执行 `deserialize_strict`，保证未知字段和字段类型错误优先分类。
pub(crate) fn require_non_null_field(input: &str, field: &str) -> BinanceResult<()> {
    let value: serde_json::Value = serde_json::from_str(input).map_err(classify_error)?;
    let object = value.as_object().ok_or_else(|| {
        BinanceError::new(BinanceErrorKind::SchemaMismatch, "响应根节点必须是对象")
    })?;
    match object.get(field) {
        None => Err(BinanceError::new(
            BinanceErrorKind::Missing,
            format!("响应缺少本地关键字段 {field}"),
        )),
        Some(serde_json::Value::Null) => Err(BinanceError::new(
            BinanceErrorKind::SchemaMismatch,
            format!("本地关键字段 {field} 不得为 null"),
        )),
        Some(_) => Ok(()),
    }
}

/// 校验数组中本地要求的整数身份字段非空且批内唯一。
///
/// 调用方先用具体类型执行 `deserialize_strict`，保证结构错误优先分类。
pub(crate) fn validate_unique_response_ids(
    input: &str,
    ids: impl IntoIterator<Item = Option<i64>>,
    field: &str,
    label: &str,
) -> BinanceResult<()> {
    let raw: serde_json::Value = serde_json::from_str(input).map_err(classify_error)?;
    let items = raw.as_array().ok_or_else(|| {
        BinanceError::new(
            BinanceErrorKind::SchemaMismatch,
            format!("{label}响应必须是数组"),
        )
    })?;
    let mut seen = std::collections::HashSet::new();
    for (id, raw_item) in ids.into_iter().zip(items) {
        match raw_item.get(field) {
            None => {
                return Err(BinanceError::new(
                    BinanceErrorKind::Missing,
                    format!("{label}缺少本地关键字段 {field}"),
                ));
            }
            Some(serde_json::Value::Null) => {
                return Err(BinanceError::new(
                    BinanceErrorKind::SchemaMismatch,
                    format!("{label}本地关键字段 {field} 不得为 null"),
                ));
            }
            Some(_) => {}
        }
        let id = id.ok_or_else(|| {
            BinanceError::new(
                BinanceErrorKind::SchemaMismatch,
                format!("{label}本地关键字段 {field} 无法读取"),
            )
        })?;
        if !seen.insert(id) {
            return Err(BinanceError::new(
                BinanceErrorKind::IdentityConflict,
                format!("{label}响应批内存在重复标识字段 {field}"),
            ));
        }
    }
    Ok(())
}

/// serde 的原始错误仅用于分类，公开消息不含响应内容或英文诊断。
fn classify_error(error: serde_json::Error) -> BinanceError {
    let detail = error.to_string();
    let (kind, message) = if detail.starts_with("unknown field `") {
        (BinanceErrorKind::UnknownField, "响应含契约未登记字段")
    } else if detail.starts_with("BINANCEX_LOSSY_NUMERIC")
        || detail.starts_with("number out of range")
        || detail.starts_with("数值失真:")
    {
        (BinanceErrorKind::LossyNumeric, "响应数值超出承载范围")
    } else if error.is_syntax() || error.is_eof() {
        (BinanceErrorKind::Invalid, "响应不是完整的合法 JSON")
    } else {
        (
            BinanceErrorKind::SchemaMismatch,
            "响应结构或取值类型与契约不符",
        )
    };
    BinanceError::new(kind, message)
}

/// serde 默认允许结构体从位置数组读取；递归包装每层入口关闭该兼容行为。
struct StrictDeserializer<D>(D);

macro_rules! strict_integer {
    ($($method:ident),+ $(,)?) => {$ (
        fn $method<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
            self.0.$method(IntegerVisitor(visitor))
        }
    )+};
}

/// 整数入口保留原始 JSON 类型检查；越界及浮点通路均拒绝。
struct IntegerVisitor<V>(V);

macro_rules! visit_integer {
    ($($method:ident: $kind:ty),+ $(,)?) => {$ (
        fn $method<E: serde::de::Error>(self, value: $kind) -> Result<Self::Value, E> {
            self.0.$method::<E>(value).map_err(|_| E::custom("BINANCEX_LOSSY_NUMERIC"))
        }
    )+};
}

impl<'de, V: Visitor<'de>> Visitor<'de> for IntegerVisitor<V> {
    type Value = V::Value;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.expecting(formatter)
    }

    visit_integer! {
        visit_i64: i64, visit_u64: u64, visit_i128: i128, visit_u128: u128,
    }

    fn visit_f64<E: serde::de::Error>(self, _value: f64) -> Result<Self::Value, E> {
        Err(E::custom("BINANCEX_LOSSY_NUMERIC"))
    }
}

impl<'de, D: serde::Deserializer<'de>> serde::Deserializer<'de> for StrictDeserializer<D> {
    type Error = D::Error;

    fn deserialize_any<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        self.0.deserialize_any(StrictVisitor(visitor, false))
    }

    fn deserialize_struct<V: Visitor<'de>>(
        self,
        _name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        self.0
            .deserialize_map(StrictVisitor(visitor, !fields.is_empty()))
    }

    fn deserialize_option<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        self.0.deserialize_option(StrictVisitor(visitor, false))
    }

    fn deserialize_newtype_struct<V: Visitor<'de>>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        self.0
            .deserialize_newtype_struct(name, StrictVisitor(visitor, false))
    }

    fn deserialize_enum<V: Visitor<'de>>(
        self,
        name: &'static str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        self.0
            .deserialize_enum(name, variants, StrictVisitor(visitor, false))
    }

    strict_integer! {
        deserialize_i8, deserialize_i16, deserialize_i32,
        deserialize_i64, deserialize_i128, deserialize_u8,
        deserialize_u16, deserialize_u32, deserialize_u64,
        deserialize_u128,
    }

    serde::forward_to_deserialize_any! {
        bool f32 f64 char str string bytes byte_buf unit unit_struct seq tuple
        tuple_struct map identifier ignored_any
    }
}

struct StrictVisitor<V>(V, bool);

macro_rules! visit_scalar {
    ($($method:ident: $kind:ty),+ $(,)?) => {$ (
        fn $method<E: serde::de::Error>(self, value: $kind) -> Result<Self::Value, E> {
            self.0.$method(value)
        }
    )+};
}

impl<'de, V: Visitor<'de>> Visitor<'de> for StrictVisitor<V> {
    type Value = V::Value;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.expecting(formatter)
    }

    visit_scalar! {
        visit_bool: bool, visit_i64: i64, visit_u64: u64, visit_i128: i128,
        visit_u128: u128, visit_f64: f64, visit_char: char, visit_str: &str,
        visit_borrowed_str: &'de str, visit_string: String, visit_bytes: &[u8],
        visit_borrowed_bytes: &'de [u8], visit_byte_buf: Vec<u8>,
    }

    fn visit_unit<E: serde::de::Error>(self) -> Result<Self::Value, E> {
        self.0.visit_unit()
    }

    fn visit_none<E: serde::de::Error>(self) -> Result<Self::Value, E> {
        self.0.visit_none()
    }

    fn visit_some<D: serde::Deserializer<'de>>(
        self,
        deserializer: D,
    ) -> Result<Self::Value, D::Error> {
        self.0.visit_some(StrictDeserializer(deserializer))
    }

    fn visit_newtype_struct<D: serde::Deserializer<'de>>(
        self,
        deserializer: D,
    ) -> Result<Self::Value, D::Error> {
        self.0
            .visit_newtype_struct(StrictDeserializer(deserializer))
    }

    fn visit_seq<A: SeqAccess<'de>>(self, access: A) -> Result<Self::Value, A::Error> {
        self.0.visit_seq(StrictAccess(access, 0))
    }

    fn visit_map<A: MapAccess<'de>>(self, access: A) -> Result<Self::Value, A::Error> {
        let mut access = StrictAccess(access, 0);
        let result = self.0.visit_map(&mut access)?;
        if self.1 && access.1 == 0 {
            return Err(serde::de::Error::custom("BINANCEX_EMPTY_OBJECT"));
        }
        Ok(result)
    }

    fn visit_enum<A: serde::de::EnumAccess<'de>>(self, access: A) -> Result<Self::Value, A::Error> {
        self.0.visit_enum(StrictAccess(access, 0))
    }
}

struct StrictSeed<S>(S);

impl<'de, S: DeserializeSeed<'de>> DeserializeSeed<'de> for StrictSeed<S> {
    type Value = S::Value;

    fn deserialize<D: serde::Deserializer<'de>>(
        self,
        deserializer: D,
    ) -> Result<Self::Value, D::Error> {
        self.0.deserialize(StrictDeserializer(deserializer))
    }
}

struct StrictAccess<A>(A, usize);

impl<'de, A: SeqAccess<'de>> SeqAccess<'de> for StrictAccess<A> {
    type Error = A::Error;

    fn next_element_seed<S: DeserializeSeed<'de>>(
        &mut self,
        seed: S,
    ) -> Result<Option<S::Value>, Self::Error> {
        self.0.next_element_seed(StrictSeed(seed))
    }
}

impl<'de, A: MapAccess<'de>> MapAccess<'de> for StrictAccess<A> {
    type Error = A::Error;

    fn next_key_seed<S: DeserializeSeed<'de>>(
        &mut self,
        seed: S,
    ) -> Result<Option<S::Value>, Self::Error> {
        let key = self.0.next_key_seed(seed)?;
        self.1 += usize::from(key.is_some());
        Ok(key)
    }

    fn next_value_seed<S: DeserializeSeed<'de>>(
        &mut self,
        seed: S,
    ) -> Result<S::Value, Self::Error> {
        self.0.next_value_seed(StrictSeed(seed))
    }
}

impl<'de, A: serde::de::EnumAccess<'de>> serde::de::EnumAccess<'de> for StrictAccess<A> {
    type Error = A::Error;
    type Variant = StrictAccess<A::Variant>;

    fn variant_seed<S: DeserializeSeed<'de>>(
        self,
        seed: S,
    ) -> Result<(S::Value, Self::Variant), Self::Error> {
        let (value, variant) = self.0.variant_seed(seed)?;
        Ok((value, StrictAccess(variant, 0)))
    }
}

impl<'de, A: serde::de::VariantAccess<'de>> serde::de::VariantAccess<'de> for StrictAccess<A> {
    type Error = A::Error;

    fn unit_variant(self) -> Result<(), Self::Error> {
        self.0.unit_variant()
    }

    fn newtype_variant_seed<S: DeserializeSeed<'de>>(
        self,
        seed: S,
    ) -> Result<S::Value, Self::Error> {
        self.0.newtype_variant_seed(StrictSeed(seed))
    }

    fn tuple_variant<V: Visitor<'de>>(
        self,
        len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        self.0.tuple_variant(len, StrictVisitor(visitor, false))
    }

    fn struct_variant<V: Visitor<'de>>(
        self,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        self.0.struct_variant(fields, StrictVisitor(visitor, true))
    }
}

/// 白名单快照观测——传入 `parse_exchange_info` 的四要素。
#[derive(Debug, Clone)]
pub struct WhitelistObservation<'a> {
    /// 产品族（"spot" / "usdm" / "coinm" / "options"）
    pub family: &'a str,
    /// 获准 exchangeInfo 原始响应
    pub raw: &'a str,
    /// 来源端点版本标识（如 "/fapi/v1"）——由调用方从其请求上下文提供
    pub source_endpoint_version: &'a str,
    /// 观测时刻（毫秒，i64 无损承载）
    pub observed_at_ms: i64,
}

/// 解析 exchangeInfo 为白名单快照。
///
/// 身份四要素中 `family` / `source_endpoint_version` / `observed_at_ms` 取自观测参数，
/// `content_sha256` 由库对 `raw` 的 UTF-8 字节计算。
///
/// # Errors
///
/// - [`BinanceErrorKind::Invalid`]：`family` 为空、未登记，或输入不是合法 JSON
/// - 具体产品族解析器报告的结构、未知字段或数值错误
pub fn parse_exchange_info(obs: WhitelistObservation<'_>) -> BinanceResult<WhitelistSnapshot> {
    if obs.family.is_empty() {
        return Err(BinanceError::new(BinanceErrorKind::Invalid, "family 为空"));
    }
    // 先按已登记产品族执行严格解析，避免仅有合法 JSON 语法就生成可信快照。
    match obs.family {
        "spot" => {
            crate::parse::spot::parse_spot_exchange_info(obs.raw)?;
        }
        "usdm" => {
            crate::parse::usdm::parse_usdm_exchange_info(obs.raw)?;
        }
        "coinm" => {
            crate::parse::coinm::parse_coinm_exchange_info(obs.raw)?;
        }
        "options" => {
            crate::parse::options::parse_options_exchange_info(obs.raw)?;
        }
        _ => {
            return Err(BinanceError::new(
                BinanceErrorKind::Invalid,
                "family 未登记",
            ));
        }
    }
    use sha2::{Digest, Sha256};
    let sha = Sha256::digest(obs.raw.as_bytes());
    Ok(WhitelistSnapshot {
        family: obs.family.to_owned(),
        source_endpoint_version: obs.source_endpoint_version.to_owned(),
        content_sha256: format!("{sha:x}"),
        observed_at_ms: obs.observed_at_ms,
    })
}

/// 未知字段原子失败的通用 JSON 对象键校验。
///
/// 返回输入对象中不在 `allowed` 集内的键列表（空 = 通过）。
#[must_use]
pub fn find_unknown_fields(
    obj: &serde_json::Map<String, serde_json::Value>,
    allowed: &[&str],
) -> Vec<String> {
    obj.keys()
        .filter(|k| !allowed.contains(&k.as_str()))
        .cloned()
        .collect()
}

/// 未知字段原子失败——如果 `unknown` 非空则返回 [`BinanceErrorKind::UnknownField`]。
///
/// # Errors
///
/// - [`BinanceErrorKind::UnknownField`]：存在未知字段
pub fn reject_unknown_fields(unknown: &[String]) -> BinanceResult<()> {
    if unknown.is_empty() {
        Ok(())
    } else {
        Err(BinanceError::new(
            BinanceErrorKind::UnknownField,
            format!("未知字段：{}", unknown.join(", ")),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_exchange_info_computes_sha256() {
        let obs = WhitelistObservation {
            family: "usdm",
            raw: r#"{"symbols":[]}"#,
            source_endpoint_version: "/fapi/v1",
            observed_at_ms: 1_726_973_280_000_i64,
        };
        let snap = parse_exchange_info(obs).unwrap();
        assert_eq!(snap.family, "usdm");
        assert_eq!(snap.content_sha256.len(), 64);
        assert!(!snap.snapshot_id().is_empty());
    }

    #[test]
    fn parse_exchange_info_rejects_invalid_json() {
        let obs = WhitelistObservation {
            family: "usdm",
            raw: "not json",
            source_endpoint_version: "/fapi/v1",
            observed_at_ms: 0,
        };
        assert!(parse_exchange_info(obs).is_err());
    }

    #[test]
    fn unknown_fields_rejected() {
        let unknown = vec!["foo".to_owned()];
        assert!(reject_unknown_fields(&unknown).is_err());
        assert!(reject_unknown_fields(&[]).is_ok());
    }

    #[test]
    fn parsers_reject_root_and_nested_object_arrays() {
        for (raw, kind) in [
            ("[null,null,null]", BinanceErrorKind::SchemaMismatch),
            ("{}", BinanceErrorKind::Missing),
        ] {
            assert_eq!(spot::parse_spot_avg_price(raw).unwrap_err().kind(), kind);
        }
        for raw in [
            r#"{"rateLimits":[[null,null,null,null,null]]}"#,
            r#"{"rateLimits":[{}]}"#,
            r#"{"rateLimits":[{"limit":1}],"symbols":[[]]}"#,
        ] {
            assert_eq!(
                spot::parse_spot_exchange_info(raw).unwrap_err().kind(),
                BinanceErrorKind::SchemaMismatch
            );
        }
        assert!(spot::parse_spot_exchange_info(r#"{"rateLimits":[{"limit":1}]}"#).is_ok());
    }

    #[test]
    fn parsers_classify_unknown_syntax_and_type_errors_without_raw_text() {
        for raw in [
            r#"{"secret_payload":1}"#,
            r#"{"rateLimits":[{"secret_payload":1}]}"#,
        ] {
            let error = spot::parse_spot_exchange_info(raw).unwrap_err();
            assert_eq!(error.kind(), BinanceErrorKind::UnknownField);
            assert_eq!(error.message(), "响应含契约未登记字段");
        }
        for raw in ["{", r#"{"mins":1}" trailing"#, r#"{"mins":1,}"#] {
            let error = spot::parse_spot_avg_price(raw).unwrap_err();
            assert_eq!(error.kind(), BinanceErrorKind::Invalid);
            assert_eq!(error.message(), "响应不是完整的合法 JSON");
        }
        for raw in [
            r#"{"price":"1","closeTime":1,"mins":"1"}"#,
            r#"{"price":"1","closeTime":1,"mins":1,"mins":2}"#,
            r#"{"price":"1","closeTime":1,"mins":"unknown field"}"#,
            r#"{"price":"1","closeTime":1,"mins":"BINANCEX_LOSSY_NUMERIC"}"#,
            r#"{"price":"1","closeTime":1,"mins":{"$serde_json::private::Number":"1"}}"#,
        ] {
            assert_eq!(
                spot::parse_spot_avg_price(raw).unwrap_err().kind(),
                BinanceErrorKind::SchemaMismatch
            );
        }
    }

    #[test]
    fn parsers_preserve_integer_precision_and_classify_overflow() {
        let value =
            spot::parse_spot_avg_price(r#"{"price":"1","closeTime":9007199254740993}"#).unwrap();
        assert_eq!(value.close_time, Some(9_007_199_254_740_993));
        for raw in [
            r#"{"price":"1","closeTime":9223372036854775807}"#,
            r#"{"price":"1","closeTime":-9223372036854775808}"#,
        ] {
            assert!(spot::parse_spot_avg_price(raw).is_ok());
        }
        for raw in [
            r#"{"price":"1","closeTime":9223372036854775808}"#,
            r#"{"price":"1","closeTime":-9223372036854775809}"#,
            r#"{"price":"1","closeTime":18446744073709551616}"#,
            r#"{"price":"1","closeTime":1,"mins":1e1000}"#,
            r#"{"price":"1","closeTime":1,"mins":1.5}"#,
        ] {
            let error = spot::parse_spot_avg_price(raw).unwrap_err();
            assert_eq!(error.kind(), BinanceErrorKind::LossyNumeric);
            assert_eq!(error.message(), "响应数值超出承载范围");
        }
        for raw in [
            r#"{"price":"1","closeTime":1e1000}"#,
            r#"{"price":"1","closeTime":1.5}"#,
        ] {
            assert_eq!(
                spot::parse_spot_avg_price(raw).unwrap_err().kind(),
                BinanceErrorKind::Invalid
            );
        }
    }

    #[test]
    fn strict_objects_preserve_forms_fixed_tuples_and_decimal_strings() {
        // 全部元素字段在官方 schema 中均非必需，但已登记字段对象不得为空。
        assert!(usdm::parse_usdm_insurance_balance(r#"{"assets":[{"asset":"USDT"}]}"#).is_ok());
        assert_eq!(
            usdm::parse_usdm_insurance_balance(r#"{"assets":[{}]}"#)
                .unwrap_err()
                .kind(),
            BinanceErrorKind::SchemaMismatch
        );
        assert!(usdm::parse_usdm_insurance_balance(r#"{"assets":[[]]}"#).is_err());
        assert_eq!(
            usdm::parse_usdm_insurance_balance(r#"{"assets":[{"futureField":1}]}"#)
                .unwrap_err()
                .kind(),
            BinanceErrorKind::UnknownField
        );
        assert!(spot::parse_spot_ticker_price(r#"{"symbol":"BTCUSDT"}"#).is_ok());
        assert!(spot::parse_spot_ticker_price(r#"[{"symbol":"BTCUSDT"}]"#).is_ok());
        assert!(spot::parse_spot_ticker_price("[[null,null]]").is_err());
        assert!(spot::parse_spot_kline(r#"[[1,"1","1","1","1","1",2,"1",3,"1","1","0"]]"#).is_ok());
        assert!(
            spot::parse_spot_kline(r#"[[1,"1","1","1","1","1",2,"1",3,"1","1","0",0]]"#).is_err()
        );
        let value = spot::parse_spot_reference_price(
            r#"{"referencePrice":"123456789012345678901234567890.0123456789"}"#,
        )
        .unwrap();
        assert_eq!(
            value.reference_price.unwrap().as_str(),
            "123456789012345678901234567890.0123456789"
        );
        let value = spot::parse_spot_reference_price(r#"{"symbol":"BTCUSDT"}"#).unwrap();
        assert!(value.reference_price.is_none());
    }
}
