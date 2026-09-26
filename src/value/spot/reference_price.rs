//! Spot 参考价格响应值类型。

use crate::value::numeric::Decimal;

/// 对应冻结端点：BN-SPOT-REST-018。
/// 冻结的 SpotReferencePrice 响应对象。
#[derive(Debug, Clone, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SpotReferencePrice {
    /// 响应字段 `symbol`。
    #[serde(rename = "symbol")]
    pub symbol: Option<String>,
    /// 响应字段 `referencePrice`。
    #[serde(rename = "referencePrice")]
    pub reference_price: Option<Decimal>,
    /// 响应字段 `timestamp`。
    #[serde(rename = "timestamp")]
    pub timestamp: Option<i64>,
}

/// 对应冻结端点：BN-SPOT-REST-019。
///
/// `parse_spot_reference_price_calculation` 校验判别值及对应的条件字段。
/// 冻结的 SpotReferencePriceCalculation 响应对象。
#[derive(Debug, Clone, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SpotReferencePriceCalculation {
    /// 响应字段 `symbol`。
    #[serde(rename = "symbol")]
    pub symbol: Option<String>,
    /// 响应字段 `calculationType`。
    #[serde(rename = "calculationType")]
    pub calculation_type: Option<String>,
    /// 响应字段 `bucketCount`。
    #[serde(rename = "bucketCount")]
    pub bucket_count: Option<i64>,
    /// 响应字段 `bucketWidthMs`。
    #[serde(rename = "bucketWidthMs")]
    pub bucket_width_ms: Option<i64>,
    /// 响应字段 `externalCalculationId`。
    #[serde(rename = "externalCalculationId")]
    pub external_calculation_id: Option<i64>,
}
