//! USDM 市场统计、指数与风险响应值类型。
//!
//! 保留冻结合同的字段、wire 形状与开放点，通过上级模块维持既有公开路径。

#[doc(hidden)]
/// USDM 嵌套响应对象 UsdmTakerLongShortRatioItem。
#[derive(Debug, Clone, PartialEq, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UsdmTakerLongShortRatioItem {
    /// 原始响应字段 buySellRatio；未提供时为 None。
    #[serde(rename = "buySellRatio")]
    pub buy_sell_ratio: Option<String>,
    /// 原始响应字段 buyVol；未提供时为 None。
    #[serde(rename = "buyVol")]
    pub buy_vol: Option<String>,
    /// 原始响应字段 sellVol；未提供时为 None。
    #[serde(rename = "sellVol")]
    pub sell_vol: Option<String>,
    /// 原始响应字段 timestamp；未提供时为 None。
    #[serde(rename = "timestamp")]
    pub timestamp: Option<i64>,
}

/// USDM 冻结集合响应 UsdmTakerLongShortRatio。
pub type UsdmTakerLongShortRatio = Vec<UsdmTakerLongShortRatioItem>;

#[doc(hidden)]
/// USDM 嵌套响应对象 UsdmGlobalLongShortAccountRatioItem。
#[derive(Debug, Clone, PartialEq, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UsdmGlobalLongShortAccountRatioItem {
    /// 原始响应字段 symbol；未提供时为 None。
    #[serde(rename = "symbol")]
    pub symbol: Option<String>,
    /// 原始响应字段 longShortRatio；未提供时为 None。
    #[serde(rename = "longShortRatio")]
    pub long_short_ratio: Option<String>,
    /// 原始响应字段 longAccount；未提供时为 None。
    #[serde(rename = "longAccount")]
    pub long_account: Option<String>,
    /// 原始响应字段 shortAccount；未提供时为 None。
    #[serde(rename = "shortAccount")]
    pub short_account: Option<String>,
    /// 原始响应字段 timestamp；未提供时为 None。
    #[serde(rename = "timestamp")]
    pub timestamp: Option<i64>,
}

/// USDM 冻结集合响应 UsdmGlobalLongShortAccountRatio。
pub type UsdmGlobalLongShortAccountRatio = Vec<UsdmGlobalLongShortAccountRatioItem>;

#[doc(hidden)]
/// USDM 嵌套响应对象 UsdmTopLongShortAccountRatioItem。
#[derive(Debug, Clone, PartialEq, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UsdmTopLongShortAccountRatioItem {
    /// 原始响应字段 symbol；未提供时为 None。
    #[serde(rename = "symbol")]
    pub symbol: Option<String>,
    /// 原始响应字段 longShortRatio；未提供时为 None。
    #[serde(rename = "longShortRatio")]
    pub long_short_ratio: Option<String>,
    /// 原始响应字段 longAccount；未提供时为 None。
    #[serde(rename = "longAccount")]
    pub long_account: Option<String>,
    /// 原始响应字段 shortAccount；未提供时为 None。
    #[serde(rename = "shortAccount")]
    pub short_account: Option<String>,
    /// 原始响应字段 timestamp；未提供时为 None。
    #[serde(rename = "timestamp")]
    pub timestamp: Option<i64>,
}

/// USDM 冻结集合响应 UsdmTopLongShortAccountRatio。
pub type UsdmTopLongShortAccountRatio = Vec<UsdmTopLongShortAccountRatioItem>;

#[doc(hidden)]
/// USDM 嵌套响应对象 UsdmTopLongShortPositionRatioItem。
#[derive(Debug, Clone, PartialEq, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UsdmTopLongShortPositionRatioItem {
    /// 原始响应字段 symbol；未提供时为 None。
    #[serde(rename = "symbol")]
    pub symbol: Option<String>,
    /// 原始响应字段 longShortRatio；未提供时为 None。
    #[serde(rename = "longShortRatio")]
    pub long_short_ratio: Option<String>,
    /// 原始响应字段 longAccount；未提供时为 None。
    #[serde(rename = "longAccount")]
    pub long_account: Option<String>,
    /// 原始响应字段 shortAccount；未提供时为 None。
    #[serde(rename = "shortAccount")]
    pub short_account: Option<String>,
    /// 原始响应字段 timestamp；未提供时为 None。
    #[serde(rename = "timestamp")]
    pub timestamp: Option<i64>,
}

/// USDM 冻结集合响应 UsdmTopLongShortPositionRatio。
pub type UsdmTopLongShortPositionRatio = Vec<UsdmTopLongShortPositionRatioItem>;

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
/// 开放点：Variant 2（type=array）元素字段表未在渲染源单独列出（全文件无 Properties for Variant 2 段）；array 形 item 按 Variant 1 object 形状推断。
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
pub struct UsdmInsuranceBalanceItemAssetsItem {}

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
/// 开放点：Variant 2（type=array）元素字段表未在渲染源单独列出（全文件无 Properties for Variant 2 段）；array 形 item 按 Variant 1 object 形状推断。
/// 开放点：Variant 1 的 assets 为 object[]，其元素字段在 Properties 段未展开（Example Responses 中 assets 为空数组，无实例证据）；item 暂记为空 object，元素结构待补证。
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
