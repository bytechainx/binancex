//! USDM 市场资料响应值类型，统一从 `value::usdm` 导出。

#[doc(hidden)]
/// USDM 嵌套响应对象 UsdmAssetIndexItem。
#[derive(Debug, Clone, PartialEq, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UsdmAssetIndexItem {
    /// 原始响应字段 symbol；未提供时为 None。
    #[serde(rename = "symbol")]
    pub symbol: Option<String>,
    /// 原始响应字段 time；未提供时为 None。
    #[serde(rename = "time")]
    pub time: Option<i64>,
    /// 原始响应字段 index；未提供时为 None。
    #[serde(rename = "index")]
    pub index: Option<String>,
    /// 原始响应字段 bidBuffer；未提供时为 None。
    #[serde(rename = "bidBuffer")]
    pub bid_buffer: Option<String>,
    /// 原始响应字段 askBuffer；未提供时为 None。
    #[serde(rename = "askBuffer")]
    pub ask_buffer: Option<String>,
    /// 原始响应字段 bidRate；未提供时为 None。
    #[serde(rename = "bidRate")]
    pub bid_rate: Option<String>,
    /// 原始响应字段 askRate；未提供时为 None。
    #[serde(rename = "askRate")]
    pub ask_rate: Option<String>,
    /// 原始响应字段 autoExchangeBidBuffer；未提供时为 None。
    #[serde(rename = "autoExchangeBidBuffer")]
    pub auto_exchange_bid_buffer: Option<String>,
    /// 原始响应字段 autoExchangeAskBuffer；未提供时为 None。
    #[serde(rename = "autoExchangeAskBuffer")]
    pub auto_exchange_ask_buffer: Option<String>,
    /// 原始响应字段 autoExchangeBidRate；未提供时为 None。
    #[serde(rename = "autoExchangeBidRate")]
    pub auto_exchange_bid_rate: Option<String>,
    /// 原始响应字段 autoExchangeAskRate；未提供时为 None。
    #[serde(rename = "autoExchangeAskRate")]
    pub auto_exchange_ask_rate: Option<String>,
}

/// USDM 对象或数组响应 UsdmAssetIndex。
/// Variant 2（type=array）元素按冻结响应结构合同映射为 `UsdmAssetIndexItem`。
#[derive(Debug, Clone, PartialEq, serde::Deserialize)]
#[serde(untagged)]
pub enum UsdmAssetIndex {
    /// 单对象响应。
    Object(Box<UsdmAssetIndexItem>),
    /// 对象数组响应。
    Array(Vec<UsdmAssetIndexItem>),
}

#[doc(hidden)]
/// USDM 嵌套响应对象 UsdmConstituentsConstituentsItem。
#[derive(Debug, Clone, PartialEq, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UsdmConstituentsConstituentsItem {
    /// 原始响应字段 exchange；未提供时为 None。
    #[serde(rename = "exchange")]
    pub exchange: Option<String>,
    /// 原始响应字段 symbol；未提供时为 None。
    #[serde(rename = "symbol")]
    pub symbol: Option<String>,
    /// 原始响应字段 price；未提供时为 None。
    #[serde(rename = "price")]
    pub price: Option<String>,
    /// 原始响应字段 weight；未提供时为 None。
    #[serde(rename = "weight")]
    pub weight: Option<String>,
}

/// USDM 冻结响应类型 UsdmConstituents。
#[derive(Debug, Clone, PartialEq, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UsdmConstituents {
    /// 原始响应字段 symbol；未提供时为 None。
    #[serde(rename = "symbol")]
    pub symbol: Option<String>,
    /// 原始响应字段 time；未提供时为 None。
    #[serde(rename = "time")]
    pub time: Option<i64>,
    /// 原始响应字段 constituents；未提供时为 None。
    #[serde(rename = "constituents")]
    pub constituents: Option<Vec<UsdmConstituentsConstituentsItem>>,
}

#[doc(hidden)]
/// USDM 嵌套响应对象 UsdmInsuranceBalanceItemAssetsItem。
#[derive(Debug, Clone, PartialEq, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UsdmInsuranceBalanceItemAssetsItem {
    /// 原始响应字段 asset；未提供时为 None。
    #[serde(rename = "asset")]
    pub asset: Option<String>,
    /// 原始响应字段 marginBalance；未提供时为 None。
    #[serde(rename = "marginBalance")]
    pub margin_balance: Option<String>,
    /// 原始响应字段 updateTime；未提供时为 None。
    #[serde(rename = "updateTime")]
    pub update_time: Option<i64>,
}

#[doc(hidden)]
/// USDM 嵌套响应对象 UsdmInsuranceBalanceItem。
#[derive(Debug, Clone, PartialEq, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UsdmInsuranceBalanceItem {
    /// 原始响应字段 symbols；未提供时为 None。
    #[serde(rename = "symbols")]
    pub symbols: Option<Vec<String>>,
    /// 原始响应字段 assets；未提供时为 None。
    #[serde(rename = "assets")]
    pub assets: Option<Vec<UsdmInsuranceBalanceItemAssetsItem>>,
}

/// USDM 对象或数组响应 UsdmInsuranceBalance。
/// 开放点：symbols 字段表类型为 string[]，与保险余额语义的资产数组关系待核实。
/// 开放点：数组形态触发条件（symbol 省略）未在源页该端点显式说明，按可选 symbol 参数语义与其他同款端点惯例推断。
#[derive(Debug, Clone, PartialEq, serde::Deserialize)]
#[serde(untagged)]
pub enum UsdmInsuranceBalance {
    /// 单对象响应。
    Object(UsdmInsuranceBalanceItem),
    /// 对象数组响应。
    Array(Vec<UsdmInsuranceBalanceItem>),
}

#[doc(hidden)]
/// USDM 嵌套响应对象 UsdmBasisItem。
#[derive(Debug, Clone, PartialEq, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UsdmBasisItem {
    /// 原始响应字段 indexPrice；未提供时为 None。
    #[serde(rename = "indexPrice")]
    pub index_price: Option<String>,
    /// 原始响应字段 contractType；未提供时为 None。
    #[serde(rename = "contractType")]
    pub contract_type: Option<String>,
    /// 原始响应字段 basisRate；未提供时为 None。
    #[serde(rename = "basisRate")]
    pub basis_rate: Option<String>,
    /// 原始响应字段 futuresPrice；未提供时为 None。
    #[serde(rename = "futuresPrice")]
    pub futures_price: Option<String>,
    /// 原始响应字段 annualizedBasisRate；未提供时为 None。
    #[serde(rename = "annualizedBasisRate")]
    pub annualized_basis_rate: Option<String>,
    /// 原始响应字段 basis；未提供时为 None。
    #[serde(rename = "basis")]
    pub basis: Option<String>,
    /// 原始响应字段 pair；未提供时为 None。
    #[serde(rename = "pair")]
    pub pair: Option<String>,
    /// 原始响应字段 timestamp；未提供时为 None。
    #[serde(rename = "timestamp")]
    pub timestamp: Option<i64>,
}

/// USDM 冻结集合响应 UsdmBasis。
pub type UsdmBasis = Vec<UsdmBasisItem>;

#[doc(hidden)]
/// USDM 嵌套响应对象 UsdmDeliveryPriceItem。
#[derive(Debug, Clone, PartialEq, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UsdmDeliveryPriceItem {
    /// 原始响应字段 deliveryTime；未提供时为 None。
    #[serde(rename = "deliveryTime")]
    pub delivery_time: Option<i64>,
    /// 原始响应字段 deliveryPrice；未提供时为 None。
    #[serde(rename = "deliveryPrice")]
    pub delivery_price: Option<serde_json::Number>,
}

/// USDM 冻结集合响应 UsdmDeliveryPrice。
pub type UsdmDeliveryPrice = Vec<UsdmDeliveryPriceItem>;

#[doc(hidden)]
/// USDM 嵌套响应对象 UsdmTradingScheduleMarketSchedulesValueSessionsItem。
#[derive(Debug, Clone, PartialEq, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UsdmTradingScheduleMarketSchedulesValueSessionsItem {
    /// 原始响应字段 startTime；未提供时为 None。
    #[serde(rename = "startTime")]
    pub start_time: Option<i64>,
    /// 原始响应字段 endTime；未提供时为 None。
    #[serde(rename = "endTime")]
    pub end_time: Option<i64>,
    /// 原始响应字段 type；未提供时为 None。
    #[serde(rename = "type")]
    pub r#type: Option<String>,
}

#[doc(hidden)]
/// USDM 嵌套响应对象 UsdmTradingScheduleMarketSchedulesValue。
#[derive(Debug, Clone, PartialEq, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UsdmTradingScheduleMarketSchedulesValue {
    /// 原始响应字段 sessions；未提供时为 None。
    #[serde(rename = "sessions")]
    pub sessions: Option<Vec<UsdmTradingScheduleMarketSchedulesValueSessionsItem>>,
}

/// USDM 冻结响应类型 UsdmTradingSchedule。
/// 开放点：marketSchedules 键集为渲染页观测枚举（六市场），官方未声明键集封闭；遇新键须先扩证据再放行。
#[derive(Debug, Clone, PartialEq, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UsdmTradingSchedule {
    /// 原始响应字段 updateTime；未提供时为 None。
    #[serde(rename = "updateTime")]
    pub update_time: Option<i64>,
    /// 原始响应字段 marketSchedules；未提供时为 None。
    #[serde(rename = "marketSchedules")]
    pub market_schedules:
        Option<std::collections::BTreeMap<String, UsdmTradingScheduleMarketSchedulesValue>>,
}

#[doc(hidden)]
/// USDM 嵌套响应对象 UsdmConvertExchangeInfoItem。
#[derive(Debug, Clone, PartialEq, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UsdmConvertExchangeInfoItem {
    /// 原始响应字段 fromAsset；未提供时为 None。
    #[serde(rename = "fromAsset")]
    pub from_asset: Option<String>,
    /// 原始响应字段 toAsset；未提供时为 None。
    #[serde(rename = "toAsset")]
    pub to_asset: Option<String>,
    /// 原始响应字段 fromAssetMinAmount；未提供时为 None。
    #[serde(rename = "fromAssetMinAmount")]
    pub from_asset_min_amount: Option<String>,
    /// 原始响应字段 fromAssetMaxAmount；未提供时为 None。
    #[serde(rename = "fromAssetMaxAmount")]
    pub from_asset_max_amount: Option<String>,
    /// 原始响应字段 toAssetMinAmount；未提供时为 None。
    #[serde(rename = "toAssetMinAmount")]
    pub to_asset_min_amount: Option<String>,
    /// 原始响应字段 toAssetMaxAmount；未提供时为 None。
    #[serde(rename = "toAssetMaxAmount")]
    pub to_asset_max_amount: Option<String>,
    /// 原始响应字段 fromIsBase；未提供时为 None。
    #[serde(rename = "fromIsBase")]
    pub from_is_base: Option<bool>,
}

/// USDM 冻结集合响应 UsdmConvertExchangeInfo。
/// 开放点：fromIsBase 仅见于活体捕获（渲染页未记载）——文档滞后差异已登记；如官方文档后续补载可撤此点。
pub type UsdmConvertExchangeInfo = Vec<UsdmConvertExchangeInfoItem>;

#[doc(hidden)]
/// USDM 嵌套响应对象 UsdmAdlRiskItem。
#[derive(Debug, Clone, PartialEq, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UsdmAdlRiskItem {
    /// 原始响应字段 symbol；未提供时为 None。
    #[serde(rename = "symbol")]
    pub symbol: Option<String>,
    /// 原始响应字段 adlRisk；未提供时为 None。
    #[serde(rename = "adlRisk")]
    pub adl_risk: Option<String>,
    /// 原始响应字段 updateTime；未提供时为 None。
    #[serde(rename = "updateTime")]
    pub update_time: Option<i64>,
}

/// USDM 对象或数组响应 UsdmAdlRisk。
/// 开放点：Variant 2（type=array）元素字段表未在渲染源单独列出（全文件无 Properties for Variant 2 段）；array 形 item 按 Variant 1 object 形状推断。
/// 开放点：数组形态触发条件（symbol 省略）未在源页该端点显式说明，按可选 symbol 参数语义与其他同款端点惯例推断。
#[derive(Debug, Clone, PartialEq, serde::Deserialize)]
#[serde(untagged)]
pub enum UsdmAdlRisk {
    /// 单对象响应。
    Object(UsdmAdlRiskItem),
    /// 对象数组响应。
    Array(Vec<UsdmAdlRiskItem>),
}
