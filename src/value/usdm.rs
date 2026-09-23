//! USDM 族冻结响应值类型。
//!
//! 逐 endpoint_id 联接 type-map 与 response-structures；保留原始字段名、数组与定长元组。
//! 所有字段当前均可缺；对象拒绝未知字段，wire string 数值不转换。
//! 开放点沿用冻结合同，不代表生产接入许可；动态 map 的键不锁定为观测样本。

pub use analytics::{
    UsdmAdlRisk, UsdmAdlRiskItem, UsdmAssetIndex, UsdmAssetIndexItem, UsdmBasis, UsdmBasisItem,
    UsdmConstituents, UsdmConstituentsConstituentsItem, UsdmDeliveryPrice, UsdmDeliveryPriceItem,
    UsdmGlobalLongShortAccountRatio, UsdmGlobalLongShortAccountRatioItem, UsdmInsuranceBalance,
    UsdmInsuranceBalanceItem, UsdmInsuranceBalanceItemAssetsItem, UsdmTakerLongShortRatio,
    UsdmTakerLongShortRatioItem, UsdmTopLongShortAccountRatio, UsdmTopLongShortAccountRatioItem,
    UsdmTopLongShortPositionRatio, UsdmTopLongShortPositionRatioItem,
};

mod analytics;

#[doc(hidden)]
/// USDM 嵌套响应对象 UsdmExchangeInfoRateLimitsItem。
#[derive(Debug, Clone, PartialEq, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UsdmExchangeInfoRateLimitsItem {
    /// 原始响应字段 interval；未提供时为 None。
    #[serde(rename = "interval")]
    pub interval: Option<String>,
    /// 原始响应字段 intervalNum；未提供时为 None。
    #[serde(rename = "intervalNum")]
    pub interval_num: Option<i64>,
    /// 原始响应字段 limit；未提供时为 None。
    #[serde(rename = "limit")]
    pub limit: Option<i64>,
    /// 原始响应字段 rateLimitType；未提供时为 None。
    #[serde(rename = "rateLimitType")]
    pub rate_limit_type: Option<String>,
}

#[doc(hidden)]
/// USDM 嵌套响应对象 UsdmExchangeInfoAssetsItem。
#[derive(Debug, Clone, PartialEq, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UsdmExchangeInfoAssetsItem {
    /// 原始响应字段 asset；未提供时为 None。
    #[serde(rename = "asset")]
    pub asset: Option<String>,
    /// 原始响应字段 marginAvailable；未提供时为 None。
    #[serde(rename = "marginAvailable")]
    pub margin_available: Option<bool>,
    /// 原始响应字段 autoAssetExchange；未提供时为 None。
    #[serde(rename = "autoAssetExchange")]
    pub auto_asset_exchange: Option<String>,
}

#[doc(hidden)]
/// USDM 嵌套响应对象 UsdmExchangeInfoSymbolsItemFiltersItem。
#[derive(Debug, Clone, PartialEq, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UsdmExchangeInfoSymbolsItemFiltersItem {
    /// 原始响应字段 filterType；未提供时为 None。
    #[serde(rename = "filterType")]
    pub filter_type: Option<String>,
    /// 原始响应字段 maxPrice；未提供时为 None。
    #[serde(rename = "maxPrice")]
    pub max_price: Option<String>,
    /// 原始响应字段 minPrice；未提供时为 None。
    #[serde(rename = "minPrice")]
    pub min_price: Option<String>,
    /// 原始响应字段 tickSize；未提供时为 None。
    #[serde(rename = "tickSize")]
    pub tick_size: Option<String>,
    /// 原始响应字段 maxQty；未提供时为 None。
    #[serde(rename = "maxQty")]
    pub max_qty: Option<String>,
    /// 原始响应字段 minQty；未提供时为 None。
    #[serde(rename = "minQty")]
    pub min_qty: Option<String>,
    /// 原始响应字段 stepSize；未提供时为 None。
    #[serde(rename = "stepSize")]
    pub step_size: Option<String>,
    /// 原始响应字段 limit；未提供时为 None。
    #[serde(rename = "limit")]
    pub limit: Option<i64>,
    /// 原始响应字段 notional；未提供时为 None。
    #[serde(rename = "notional")]
    pub notional: Option<String>,
    /// 原始响应字段 multiplierUp；未提供时为 None。
    #[serde(rename = "multiplierUp")]
    pub multiplier_up: Option<String>,
    /// 原始响应字段 multiplierDown；未提供时为 None。
    #[serde(rename = "multiplierDown")]
    pub multiplier_down: Option<String>,
    /// 原始响应字段 multiplierDecimal；未提供时为 None。
    #[serde(rename = "multiplierDecimal")]
    pub multiplier_decimal: Option<String>,
}

#[doc(hidden)]
/// USDM 嵌套响应对象 UsdmExchangeInfoSymbolsItem。
#[derive(Debug, Clone, PartialEq, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UsdmExchangeInfoSymbolsItem {
    /// 原始响应字段 symbol；未提供时为 None。
    #[serde(rename = "symbol")]
    pub symbol: Option<String>,
    /// 原始响应字段 pair；未提供时为 None。
    #[serde(rename = "pair")]
    pub pair: Option<String>,
    /// 原始响应字段 contractType；未提供时为 None。
    #[serde(rename = "contractType")]
    pub contract_type: Option<String>,
    /// 原始响应字段 deliveryDate；未提供时为 None。
    #[serde(rename = "deliveryDate")]
    pub delivery_date: Option<i64>,
    /// 原始响应字段 onboardDate；未提供时为 None。
    #[serde(rename = "onboardDate")]
    pub onboard_date: Option<i64>,
    /// 原始响应字段 status；未提供时为 None。
    #[serde(rename = "status")]
    pub status: Option<String>,
    /// 原始响应字段 maintMarginPercent；未提供时为 None。
    #[serde(rename = "maintMarginPercent")]
    pub maint_margin_percent: Option<String>,
    /// 原始响应字段 requiredMarginPercent；未提供时为 None。
    #[serde(rename = "requiredMarginPercent")]
    pub required_margin_percent: Option<String>,
    /// 原始响应字段 baseAsset；未提供时为 None。
    #[serde(rename = "baseAsset")]
    pub base_asset: Option<String>,
    /// 原始响应字段 quoteAsset；未提供时为 None。
    #[serde(rename = "quoteAsset")]
    pub quote_asset: Option<String>,
    /// 原始响应字段 marginAsset；未提供时为 None。
    #[serde(rename = "marginAsset")]
    pub margin_asset: Option<String>,
    /// 原始响应字段 pricePrecision；未提供时为 None。
    #[serde(rename = "pricePrecision")]
    pub price_precision: Option<i64>,
    /// 原始响应字段 quantityPrecision；未提供时为 None。
    #[serde(rename = "quantityPrecision")]
    pub quantity_precision: Option<i64>,
    /// 原始响应字段 baseAssetPrecision；未提供时为 None。
    #[serde(rename = "baseAssetPrecision")]
    pub base_asset_precision: Option<i64>,
    /// 原始响应字段 quotePrecision；未提供时为 None。
    #[serde(rename = "quotePrecision")]
    pub quote_precision: Option<i64>,
    /// 原始响应字段 underlyingType；未提供时为 None。
    #[serde(rename = "underlyingType")]
    pub underlying_type: Option<String>,
    /// 原始响应字段 underlyingSubType；未提供时为 None。
    #[serde(rename = "underlyingSubType")]
    pub underlying_sub_type: Option<Vec<String>>,
    /// 原始响应字段 settlePlan；未提供时为 None。
    #[serde(rename = "settlePlan")]
    pub settle_plan: Option<i64>,
    /// 原始响应字段 triggerProtect；未提供时为 None。
    #[serde(rename = "triggerProtect")]
    pub trigger_protect: Option<String>,
    /// 原始响应字段 filters；未提供时为 None。
    #[serde(rename = "filters")]
    pub filters: Option<Vec<UsdmExchangeInfoSymbolsItemFiltersItem>>,
    /// 原始响应字段 orderTypes；未提供时为 None。
    #[serde(rename = "orderTypes")]
    pub order_types: Option<Vec<String>>,
    /// 原始响应字段 timeInForce；未提供时为 None。
    #[serde(rename = "timeInForce")]
    pub time_in_force: Option<Vec<String>>,
    /// 原始响应字段 liquidationFee；未提供时为 None。
    #[serde(rename = "liquidationFee")]
    pub liquidation_fee: Option<String>,
    /// 原始响应字段 marketTakeBound；未提供时为 None。
    #[serde(rename = "marketTakeBound")]
    pub market_take_bound: Option<String>,
}

/// USDM 冻结响应类型 UsdmExchangeInfo。
/// 开放点：symbols[].filters 为 union 形态（判别字段 filterType + 观测字段并集）；各 filterType 的字段集归属待逐类型核实后拆分为 variants。
/// 开放点：timezone 归属根对象（依据 Example Responses JSON）；字段表扁平文本不保留缩进，层级以示例 JSON 为准。
#[derive(Debug, Clone, PartialEq, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UsdmExchangeInfo {
    /// 原始响应字段 exchangeFilters；未提供时为 None。
    #[serde(rename = "exchangeFilters")]
    pub exchange_filters: Option<Vec<String>>,
    /// 原始响应字段 rateLimits；未提供时为 None。
    #[serde(rename = "rateLimits")]
    pub rate_limits: Option<Vec<UsdmExchangeInfoRateLimitsItem>>,
    /// 原始响应字段 serverTime；未提供时为 None。
    #[serde(rename = "serverTime")]
    pub server_time: Option<i64>,
    /// 原始响应字段 assets；未提供时为 None。
    #[serde(rename = "assets")]
    pub assets: Option<Vec<UsdmExchangeInfoAssetsItem>>,
    /// 原始响应字段 symbols；未提供时为 None。
    #[serde(rename = "symbols")]
    pub symbols: Option<Vec<UsdmExchangeInfoSymbolsItem>>,
    /// 原始响应字段 timezone；未提供时为 None。
    #[serde(rename = "timezone")]
    pub timezone: Option<String>,
}

#[doc(hidden)]
/// USDM 嵌套响应对象 UsdmIndexInfoItemBaseAssetListItem。
#[derive(Debug, Clone, PartialEq, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UsdmIndexInfoItemBaseAssetListItem {
    /// 原始响应字段 baseAsset；未提供时为 None。
    #[serde(rename = "baseAsset")]
    pub base_asset: Option<String>,
    /// 原始响应字段 quoteAsset；未提供时为 None。
    #[serde(rename = "quoteAsset")]
    pub quote_asset: Option<String>,
    /// 原始响应字段 weightInQuantity；未提供时为 None。
    #[serde(rename = "weightInQuantity")]
    pub weight_in_quantity: Option<String>,
    /// 原始响应字段 weightInPercentage；未提供时为 None。
    #[serde(rename = "weightInPercentage")]
    pub weight_in_percentage: Option<String>,
}

#[doc(hidden)]
/// USDM 嵌套响应对象 UsdmIndexInfoItem。
#[derive(Debug, Clone, PartialEq, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UsdmIndexInfoItem {
    /// 原始响应字段 symbol；未提供时为 None。
    #[serde(rename = "symbol")]
    pub symbol: Option<String>,
    /// 原始响应字段 time；未提供时为 None。
    #[serde(rename = "time")]
    pub time: Option<i64>,
    /// 原始响应字段 component；未提供时为 None。
    #[serde(rename = "component")]
    pub component: Option<String>,
    /// 原始响应字段 baseAssetList；未提供时为 None。
    #[serde(rename = "baseAssetList")]
    pub base_asset_list: Option<Vec<UsdmIndexInfoItemBaseAssetListItem>>,
}

/// USDM 冻结集合响应 UsdmIndexInfo。
pub type UsdmIndexInfo = Vec<UsdmIndexInfoItem>;

#[doc(hidden)]
/// USDM 嵌套响应对象 UsdmAggTradeItem。
#[derive(Debug, Clone, PartialEq, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UsdmAggTradeItem {
    /// 原始响应字段 a；未提供时为 None。
    #[serde(rename = "a")]
    pub a: Option<i64>,
    /// 原始响应字段 p；未提供时为 None。
    #[serde(rename = "p")]
    pub p: Option<String>,
    /// 原始响应字段 q；未提供时为 None。
    #[serde(rename = "q")]
    pub q: Option<String>,
    /// 原始响应字段 nq；未提供时为 None。
    #[serde(rename = "nq")]
    pub nq: Option<String>,
    /// 原始响应字段 f；未提供时为 None。
    #[serde(rename = "f")]
    pub f: Option<i64>,
    /// 原始响应字段 l；未提供时为 None。
    #[serde(rename = "l")]
    pub l: Option<i64>,
    /// 原始响应字段 T；未提供时为 None。
    #[serde(rename = "T")]
    pub t: Option<i64>,
    /// 原始响应字段 m；未提供时为 None。
    #[serde(rename = "m")]
    pub m: Option<bool>,
}

/// USDM 冻结集合响应 UsdmAggTrade。
pub type UsdmAggTrade = Vec<UsdmAggTradeItem>;

#[doc(hidden)]
/// USDM 嵌套响应对象 UsdmTradeItem。
#[derive(Debug, Clone, PartialEq, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UsdmTradeItem {
    /// 原始响应字段 id；未提供时为 None。
    #[serde(rename = "id")]
    pub id: Option<i64>,
    /// 原始响应字段 price；未提供时为 None。
    #[serde(rename = "price")]
    pub price: Option<String>,
    /// 原始响应字段 qty；未提供时为 None。
    #[serde(rename = "qty")]
    pub qty: Option<String>,
    /// 原始响应字段 quoteQty；未提供时为 None。
    #[serde(rename = "quoteQty")]
    pub quote_qty: Option<String>,
    /// 原始响应字段 time；未提供时为 None。
    #[serde(rename = "time")]
    pub time: Option<i64>,
    /// 原始响应字段 isBuyerMaker；未提供时为 None。
    #[serde(rename = "isBuyerMaker")]
    pub is_buyer_maker: Option<bool>,
    /// 原始响应字段 isRPITrade；未提供时为 None。
    #[serde(rename = "isRPITrade")]
    pub is_rpi_trade: Option<bool>,
}

/// USDM 冻结集合响应 UsdmTrade。
pub type UsdmTrade = Vec<UsdmTradeItem>;

/// USDM 冻结集合响应 UsdmContinuousKline。
pub type UsdmContinuousKline = Vec<(
    i64,
    String,
    String,
    String,
    String,
    String,
    i64,
    String,
    i64,
    String,
    String,
    String,
)>;

/// USDM 冻结集合响应 UsdmKline。
pub type UsdmKline = Vec<(
    i64,
    String,
    String,
    String,
    String,
    String,
    i64,
    String,
    i64,
    String,
    String,
    String,
)>;

/// USDM 冻结集合响应 UsdmIndexPriceKline。
pub type UsdmIndexPriceKline = Vec<(
    i64,
    String,
    String,
    String,
    String,
    String,
    i64,
    String,
    i64,
    String,
    String,
    String,
)>;

/// USDM 冻结集合响应 UsdmMarkPriceKline。
pub type UsdmMarkPriceKline = Vec<(
    i64,
    String,
    String,
    String,
    String,
    String,
    i64,
    String,
    i64,
    String,
    String,
    String,
)>;

#[doc(hidden)]
/// USDM 嵌套响应对象 UsdmPremiumIndexItem。
#[derive(Debug, Clone, PartialEq, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UsdmPremiumIndexItem {
    /// 原始响应字段 symbol；未提供时为 None。
    #[serde(rename = "symbol")]
    pub symbol: Option<String>,
    /// 原始响应字段 markPrice；未提供时为 None。
    #[serde(rename = "markPrice")]
    pub mark_price: Option<String>,
    /// 原始响应字段 indexPrice；未提供时为 None。
    #[serde(rename = "indexPrice")]
    pub index_price: Option<String>,
    /// 原始响应字段 estimatedSettlePrice；未提供时为 None。
    #[serde(rename = "estimatedSettlePrice")]
    pub estimated_settle_price: Option<String>,
    /// 原始响应字段 lastFundingRate；未提供时为 None。
    #[serde(rename = "lastFundingRate")]
    pub last_funding_rate: Option<String>,
    /// 原始响应字段 interestRate；未提供时为 None。
    #[serde(rename = "interestRate")]
    pub interest_rate: Option<String>,
    /// 原始响应字段 nextFundingTime；未提供时为 None。
    #[serde(rename = "nextFundingTime")]
    pub next_funding_time: Option<i64>,
    /// 原始响应字段 time；未提供时为 None。
    #[serde(rename = "time")]
    pub time: Option<i64>,
}

/// USDM 对象或数组响应 UsdmPremiumIndex。
/// 开放点：Variant 2（type=array）元素字段表未在渲染源单独列出（全文件无 Properties for Variant 2 段）；array 形 item 按 Variant 1 object 形状推断。
#[derive(Debug, Clone, PartialEq, serde::Deserialize)]
#[serde(untagged)]
pub enum UsdmPremiumIndex {
    /// 单对象响应。
    Object(UsdmPremiumIndexItem),
    /// 对象数组响应。
    Array(Vec<UsdmPremiumIndexItem>),
}

/// USDM 冻结集合响应 UsdmPremiumIndexKline。
pub type UsdmPremiumIndexKline = Vec<(
    i64,
    String,
    String,
    String,
    String,
    String,
    i64,
    String,
    i64,
    String,
    String,
    String,
)>;

#[doc(hidden)]
/// USDM 嵌套响应对象 UsdmFundingRateItem。
#[derive(Debug, Clone, PartialEq, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UsdmFundingRateItem {
    /// 原始响应字段 symbol；未提供时为 None。
    #[serde(rename = "symbol")]
    pub symbol: Option<String>,
    /// 原始响应字段 fundingRate；未提供时为 None。
    #[serde(rename = "fundingRate")]
    pub funding_rate: Option<String>,
    /// 原始响应字段 fundingTime；未提供时为 None。
    #[serde(rename = "fundingTime")]
    pub funding_time: Option<i64>,
    /// 原始响应字段 markPrice；未提供时为 None。
    #[serde(rename = "markPrice")]
    pub mark_price: Option<String>,
    /// 原始响应字段 rateType；未提供时为 None。
    #[serde(rename = "rateType")]
    pub rate_type: Option<String>,
}

/// USDM 冻结集合响应 UsdmFundingRate。
pub type UsdmFundingRate = Vec<UsdmFundingRateItem>;

#[doc(hidden)]
/// USDM 嵌套响应对象 UsdmFundingInfoItem。
#[derive(Debug, Clone, PartialEq, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UsdmFundingInfoItem {
    /// 原始响应字段 symbol；未提供时为 None。
    #[serde(rename = "symbol")]
    pub symbol: Option<String>,
    /// 原始响应字段 adjustedFundingRateCap；未提供时为 None。
    #[serde(rename = "adjustedFundingRateCap")]
    pub adjusted_funding_rate_cap: Option<String>,
    /// 原始响应字段 adjustedFundingRateFloor；未提供时为 None。
    #[serde(rename = "adjustedFundingRateFloor")]
    pub adjusted_funding_rate_floor: Option<String>,
    /// 原始响应字段 fundingIntervalHours；未提供时为 None。
    #[serde(rename = "fundingIntervalHours")]
    pub funding_interval_hours: Option<i64>,
    /// 原始响应字段 disclaimer；未提供时为 None。
    #[serde(rename = "disclaimer")]
    pub disclaimer: Option<bool>,
}

/// USDM 冻结集合响应 UsdmFundingInfo。
pub type UsdmFundingInfo = Vec<UsdmFundingInfoItem>;

/// USDM 冻结响应类型 UsdmOpenInterest。
#[derive(Debug, Clone, PartialEq, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UsdmOpenInterest {
    /// 原始响应字段 openInterest；未提供时为 None。
    #[serde(rename = "openInterest")]
    pub open_interest: Option<String>,
    /// 原始响应字段 symbol；未提供时为 None。
    #[serde(rename = "symbol")]
    pub symbol: Option<String>,
    /// 原始响应字段 time；未提供时为 None。
    #[serde(rename = "time")]
    pub time: Option<i64>,
}

#[doc(hidden)]
/// USDM 嵌套响应对象 UsdmOpenInterestHistItem。
#[derive(Debug, Clone, PartialEq, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UsdmOpenInterestHistItem {
    /// 原始响应字段 symbol；未提供时为 None。
    #[serde(rename = "symbol")]
    pub symbol: Option<String>,
    /// 原始响应字段 sumOpenInterest；未提供时为 None。
    #[serde(rename = "sumOpenInterest")]
    pub sum_open_interest: Option<String>,
    /// 原始响应字段 sumOpenInterestValue；未提供时为 None。
    #[serde(rename = "sumOpenInterestValue")]
    pub sum_open_interest_value: Option<String>,
    /// 原始响应字段 CMCCirculatingSupply；未提供时为 None。
    #[serde(rename = "CMCCirculatingSupply")]
    pub cmc_circulating_supply: Option<String>,
    /// 原始响应字段 timestamp；未提供时为 None。
    #[serde(rename = "timestamp")]
    pub timestamp: Option<i64>,
}

/// USDM 冻结集合响应 UsdmOpenInterestHist。
pub type UsdmOpenInterestHist = Vec<UsdmOpenInterestHistItem>;

/// USDM 冻结响应类型 UsdmBookSnapshot。
#[derive(Debug, Clone, PartialEq, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UsdmBookSnapshot {
    /// 原始响应字段 lastUpdateId；未提供时为 None。
    #[serde(rename = "lastUpdateId")]
    pub last_update_id: Option<i64>,
    /// 原始响应字段 E；未提供时为 None。
    #[serde(rename = "E")]
    pub e: Option<i64>,
    /// 原始响应字段 T；未提供时为 None。
    #[serde(rename = "T")]
    pub t: Option<i64>,
    /// 原始响应字段 bids；未提供时为 None。
    #[serde(rename = "bids")]
    pub bids: Option<Vec<(String, String)>>,
    /// 原始响应字段 asks；未提供时为 None。
    #[serde(rename = "asks")]
    pub asks: Option<Vec<(String, String)>>,
}

#[doc(hidden)]
/// USDM 嵌套响应对象 UsdmBookTickerItem。
#[derive(Debug, Clone, PartialEq, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UsdmBookTickerItem {
    /// 原始响应字段 symbol；未提供时为 None。
    #[serde(rename = "symbol")]
    pub symbol: Option<String>,
    /// 原始响应字段 bidPrice；未提供时为 None。
    #[serde(rename = "bidPrice")]
    pub bid_price: Option<String>,
    /// 原始响应字段 bidQty；未提供时为 None。
    #[serde(rename = "bidQty")]
    pub bid_qty: Option<String>,
    /// 原始响应字段 askPrice；未提供时为 None。
    #[serde(rename = "askPrice")]
    pub ask_price: Option<String>,
    /// 原始响应字段 askQty；未提供时为 None。
    #[serde(rename = "askQty")]
    pub ask_qty: Option<String>,
    /// 原始响应字段 time；未提供时为 None。
    #[serde(rename = "time")]
    pub time: Option<i64>,
}

/// USDM 对象或数组响应 UsdmBookTicker。
/// 开放点：Variant 2（type=array）元素字段表未在渲染源单独列出（全文件无 Properties for Variant 2 段）；array 形 item 按 Variant 1 object 形状推断。
#[derive(Debug, Clone, PartialEq, serde::Deserialize)]
#[serde(untagged)]
pub enum UsdmBookTicker {
    /// 单对象响应。
    Object(UsdmBookTickerItem),
    /// 对象数组响应。
    Array(Vec<UsdmBookTickerItem>),
}

#[doc(hidden)]
/// USDM 嵌套响应对象 UsdmTickerPriceItem。
#[derive(Debug, Clone, PartialEq, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UsdmTickerPriceItem {
    /// 原始响应字段 symbol；未提供时为 None。
    #[serde(rename = "symbol")]
    pub symbol: Option<String>,
    /// 原始响应字段 price；未提供时为 None。
    #[serde(rename = "price")]
    pub price: Option<String>,
    /// 原始响应字段 time；未提供时为 None。
    #[serde(rename = "time")]
    pub time: Option<i64>,
}

/// USDM 对象或数组响应 UsdmTickerPrice。
/// 开放点：Variant 2（type=array）元素字段表未在渲染源单独列出（全文件无 Properties for Variant 2 段）；array 形 item 按 Variant 1 object 形状推断。
#[derive(Debug, Clone, PartialEq, serde::Deserialize)]
#[serde(untagged)]
pub enum UsdmTickerPrice {
    /// 单对象响应。
    Object(UsdmTickerPriceItem),
    /// 对象数组响应。
    Array(Vec<UsdmTickerPriceItem>),
}

#[doc(hidden)]
/// USDM 嵌套响应对象 UsdmTicker24hrItem。
#[derive(Debug, Clone, PartialEq, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UsdmTicker24hrItem {
    /// 原始响应字段 symbol；未提供时为 None。
    #[serde(rename = "symbol")]
    pub symbol: Option<String>,
    /// 原始响应字段 priceChange；未提供时为 None。
    #[serde(rename = "priceChange")]
    pub price_change: Option<String>,
    /// 原始响应字段 priceChangePercent；未提供时为 None。
    #[serde(rename = "priceChangePercent")]
    pub price_change_percent: Option<String>,
    /// 原始响应字段 weightedAvgPrice；未提供时为 None。
    #[serde(rename = "weightedAvgPrice")]
    pub weighted_avg_price: Option<String>,
    /// 原始响应字段 lastPrice；未提供时为 None。
    #[serde(rename = "lastPrice")]
    pub last_price: Option<String>,
    /// 原始响应字段 lastQty；未提供时为 None。
    #[serde(rename = "lastQty")]
    pub last_qty: Option<String>,
    /// 原始响应字段 openPrice；未提供时为 None。
    #[serde(rename = "openPrice")]
    pub open_price: Option<String>,
    /// 原始响应字段 highPrice；未提供时为 None。
    #[serde(rename = "highPrice")]
    pub high_price: Option<String>,
    /// 原始响应字段 lowPrice；未提供时为 None。
    #[serde(rename = "lowPrice")]
    pub low_price: Option<String>,
    /// 原始响应字段 volume；未提供时为 None。
    #[serde(rename = "volume")]
    pub volume: Option<String>,
    /// 原始响应字段 quoteVolume；未提供时为 None。
    #[serde(rename = "quoteVolume")]
    pub quote_volume: Option<String>,
    /// 原始响应字段 openTime；未提供时为 None。
    #[serde(rename = "openTime")]
    pub open_time: Option<i64>,
    /// 原始响应字段 closeTime；未提供时为 None。
    #[serde(rename = "closeTime")]
    pub close_time: Option<i64>,
    /// 原始响应字段 firstId；未提供时为 None。
    #[serde(rename = "firstId")]
    pub first_id: Option<i64>,
    /// 原始响应字段 lastId；未提供时为 None。
    #[serde(rename = "lastId")]
    pub last_id: Option<i64>,
    /// 原始响应字段 count；未提供时为 None。
    #[serde(rename = "count")]
    pub count: Option<i64>,
}

/// USDM 对象或数组响应 UsdmTicker24hr。
/// 开放点：Variant 2（type=array）元素字段表未在渲染源单独列出（全文件无 Properties for Variant 2 段）；array 形 item 按 Variant 1 object 形状推断。
#[derive(Debug, Clone, PartialEq, serde::Deserialize)]
#[serde(untagged)]
pub enum UsdmTicker24hr {
    /// 单对象响应。
    Object(Box<UsdmTicker24hrItem>),
    /// 对象数组响应。
    Array(Vec<UsdmTicker24hrItem>),
}

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
