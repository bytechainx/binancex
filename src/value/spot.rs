//! Spot 族冻结响应的值对象。
//!
//! 类型按端点映射联接响应结构合同；可缺字段使用 `Option`，未知字段拒绝解析。
//! 合同中的待核实事项保留在对应类型文档中，不构成 live 或生产授权。

#![forbid(unsafe_code)]

mod reference_price;

pub use reference_price::{SpotReferencePrice, SpotReferencePriceCalculation};

/// 冻结响应中的嵌套对象。
#[doc(hidden)]
#[derive(Debug, Clone, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SpotExchangeInfoRateLimitsItem {
    /// 响应字段 `rateLimitType`。
    #[serde(rename = "rateLimitType")]
    pub rate_limit_type: Option<String>,
    /// 响应字段 `interval`。
    #[serde(rename = "interval")]
    pub interval: Option<String>,
    /// 响应字段 `intervalNum`。
    #[serde(rename = "intervalNum")]
    pub interval_num: Option<i64>,
    /// 响应字段 `limit`。
    #[serde(rename = "limit")]
    pub limit: Option<i64>,
    /// 响应字段 `count`。
    #[serde(rename = "count")]
    pub count: Option<i64>,
}

/// 冻结响应中的嵌套对象。
#[doc(hidden)]
#[derive(Debug, Clone, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SpotExchangeInfoExchangeFiltersItem {
    /// 响应字段 `filterType`。
    #[serde(rename = "filterType")]
    pub filter_type: Option<String>,
    /// 响应字段 `maxNumOrders`。
    #[serde(rename = "maxNumOrders")]
    pub max_num_orders: Option<i64>,
    /// 响应字段 `maxNumAlgoOrders`。
    #[serde(rename = "maxNumAlgoOrders")]
    pub max_num_algo_orders: Option<i64>,
    /// 响应字段 `maxNumIcebergOrders`。
    #[serde(rename = "maxNumIcebergOrders")]
    pub max_num_iceberg_orders: Option<i64>,
    /// 响应字段 `maxNumOrderLists`。
    #[serde(rename = "maxNumOrderLists")]
    pub max_num_order_lists: Option<i64>,
}

/// 冻结响应中的嵌套对象。
#[doc(hidden)]
#[derive(Debug, Clone, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SpotExchangeInfoSymbolsItemFiltersItem {
    /// 响应字段 `filterType`。
    #[serde(rename = "filterType")]
    pub filter_type: Option<String>,
    /// 响应字段 `priceExponent`。
    #[serde(rename = "priceExponent")]
    pub price_exponent: Option<i64>,
    /// 响应字段 `minPrice`。
    #[serde(rename = "minPrice")]
    pub min_price: Option<String>,
    /// 响应字段 `maxPrice`。
    #[serde(rename = "maxPrice")]
    pub max_price: Option<String>,
    /// 响应字段 `tickSize`。
    #[serde(rename = "tickSize")]
    pub tick_size: Option<String>,
    /// 响应字段 `multiplierUp`。
    #[serde(rename = "multiplierUp")]
    pub multiplier_up: Option<String>,
    /// 响应字段 `multiplierDown`。
    #[serde(rename = "multiplierDown")]
    pub multiplier_down: Option<String>,
    /// 响应字段 `avgPriceMins`。
    #[serde(rename = "avgPriceMins")]
    pub avg_price_mins: Option<i64>,
    /// 响应字段 `bidMultiplierUp`。
    #[serde(rename = "bidMultiplierUp")]
    pub bid_multiplier_up: Option<String>,
    /// 响应字段 `bidMultiplierDown`。
    #[serde(rename = "bidMultiplierDown")]
    pub bid_multiplier_down: Option<String>,
    /// 响应字段 `askMultiplierUp`。
    #[serde(rename = "askMultiplierUp")]
    pub ask_multiplier_up: Option<String>,
    /// 响应字段 `askMultiplierDown`。
    #[serde(rename = "askMultiplierDown")]
    pub ask_multiplier_down: Option<String>,
    /// 响应字段 `minQty`。
    #[serde(rename = "minQty")]
    pub min_qty: Option<String>,
    /// 响应字段 `maxQty`。
    #[serde(rename = "maxQty")]
    pub max_qty: Option<String>,
    /// 响应字段 `stepSize`。
    #[serde(rename = "stepSize")]
    pub step_size: Option<String>,
    /// 响应字段 `minNotional`。
    #[serde(rename = "minNotional")]
    pub min_notional: Option<String>,
    /// 响应字段 `maxNotional`。
    #[serde(rename = "maxNotional")]
    pub max_notional: Option<String>,
    /// 响应字段 `applyToMarket`。
    #[serde(rename = "applyToMarket")]
    pub apply_to_market: Option<bool>,
    /// 响应字段 `applyMinToMarket`。
    #[serde(rename = "applyMinToMarket")]
    pub apply_min_to_market: Option<bool>,
    /// 响应字段 `applyMaxToMarket`。
    #[serde(rename = "applyMaxToMarket")]
    pub apply_max_to_market: Option<bool>,
    /// 响应字段 `limit`。
    #[serde(rename = "limit")]
    pub limit: Option<i64>,
    /// 响应字段 `maxNumOrders`。
    #[serde(rename = "maxNumOrders")]
    pub max_num_orders: Option<i64>,
    /// 响应字段 `maxNumAlgoOrders`。
    #[serde(rename = "maxNumAlgoOrders")]
    pub max_num_algo_orders: Option<i64>,
    /// 响应字段 `maxNumIcebergOrders`。
    #[serde(rename = "maxNumIcebergOrders")]
    pub max_num_iceberg_orders: Option<i64>,
    /// 响应字段 `maxPosition`。
    #[serde(rename = "maxPosition")]
    pub max_position: Option<String>,
    /// 响应字段 `minTrailingAboveDelta`。
    #[serde(rename = "minTrailingAboveDelta")]
    pub min_trailing_above_delta: Option<i64>,
    /// 响应字段 `maxTrailingAboveDelta`。
    #[serde(rename = "maxTrailingAboveDelta")]
    pub max_trailing_above_delta: Option<i64>,
    /// 响应字段 `minTrailingBelowDelta`。
    #[serde(rename = "minTrailingBelowDelta")]
    pub min_trailing_below_delta: Option<i64>,
    /// 响应字段 `maxTrailingBelowDelta`。
    #[serde(rename = "maxTrailingBelowDelta")]
    pub max_trailing_below_delta: Option<i64>,
    /// 响应字段 `maxNumOrderAmends`。
    #[serde(rename = "maxNumOrderAmends")]
    pub max_num_order_amends: Option<i64>,
    /// 响应字段 `maxNumOrderLists`。
    #[serde(rename = "maxNumOrderLists")]
    pub max_num_order_lists: Option<i64>,
    /// 响应字段 `endTime`。
    #[serde(rename = "endTime")]
    pub end_time: Option<i64>,
}

/// 冻结响应中的嵌套对象。
#[doc(hidden)]
#[derive(Debug, Clone, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SpotExchangeInfoSymbolsItem {
    /// 响应字段 `symbol`。
    #[serde(rename = "symbol")]
    pub symbol: Option<String>,
    /// 响应字段 `status`。
    #[serde(rename = "status")]
    pub status: Option<String>,
    /// 响应字段 `baseAsset`。
    #[serde(rename = "baseAsset")]
    pub base_asset: Option<String>,
    /// 响应字段 `baseAssetPrecision`。
    #[serde(rename = "baseAssetPrecision")]
    pub base_asset_precision: Option<i64>,
    /// 响应字段 `quoteAsset`。
    #[serde(rename = "quoteAsset")]
    pub quote_asset: Option<String>,
    /// 响应字段 `quotePrecision`。
    #[serde(rename = "quotePrecision")]
    pub quote_precision: Option<i64>,
    /// 响应字段 `quoteAssetPrecision`。
    #[serde(rename = "quoteAssetPrecision")]
    pub quote_asset_precision: Option<i64>,
    /// 响应字段 `baseCommissionPrecision`。
    #[serde(rename = "baseCommissionPrecision")]
    pub base_commission_precision: Option<i64>,
    /// 响应字段 `quoteCommissionPrecision`。
    #[serde(rename = "quoteCommissionPrecision")]
    pub quote_commission_precision: Option<i64>,
    /// 响应字段 `orderTypes`。
    #[serde(rename = "orderTypes")]
    pub order_types: Option<Vec<String>>,
    /// 响应字段 `icebergAllowed`。
    #[serde(rename = "icebergAllowed")]
    pub iceberg_allowed: Option<bool>,
    /// 响应字段 `ocoAllowed`。
    #[serde(rename = "ocoAllowed")]
    pub oco_allowed: Option<bool>,
    /// 响应字段 `otoAllowed`。
    #[serde(rename = "otoAllowed")]
    pub oto_allowed: Option<bool>,
    /// 响应字段 `opoAllowed`。
    #[serde(rename = "opoAllowed")]
    pub opo_allowed: Option<bool>,
    /// 响应字段 `quoteOrderQtyMarketAllowed`。
    #[serde(rename = "quoteOrderQtyMarketAllowed")]
    pub quote_order_qty_market_allowed: Option<bool>,
    /// 响应字段 `allowTrailingStop`。
    #[serde(rename = "allowTrailingStop")]
    pub allow_trailing_stop: Option<bool>,
    /// 响应字段 `cancelReplaceAllowed`。
    #[serde(rename = "cancelReplaceAllowed")]
    pub cancel_replace_allowed: Option<bool>,
    /// 响应字段 `amendAllowed`。
    #[serde(rename = "amendAllowed")]
    pub amend_allowed: Option<bool>,
    /// 响应字段 `pegInstructionsAllowed`。
    #[serde(rename = "pegInstructionsAllowed")]
    pub peg_instructions_allowed: Option<bool>,
    /// 响应字段 `isSpotTradingAllowed`。
    #[serde(rename = "isSpotTradingAllowed")]
    pub is_spot_trading_allowed: Option<bool>,
    /// 响应字段 `isMarginTradingAllowed`。
    #[serde(rename = "isMarginTradingAllowed")]
    pub is_margin_trading_allowed: Option<bool>,
    /// 响应字段 `filters`。
    #[serde(rename = "filters")]
    pub filters: Option<Vec<SpotExchangeInfoSymbolsItemFiltersItem>>,
    /// 响应字段 `permissions`。
    #[serde(rename = "permissions")]
    pub permissions: Option<Vec<String>>,
    /// 响应字段 `permissionSets`。
    #[serde(rename = "permissionSets")]
    pub permission_sets: Option<Vec<Vec<String>>>,
    /// 响应字段 `defaultSelfTradePreventionMode`。
    #[serde(rename = "defaultSelfTradePreventionMode")]
    pub default_self_trade_prevention_mode: Option<String>,
    /// 响应字段 `allowedSelfTradePreventionModes`。
    #[serde(rename = "allowedSelfTradePreventionModes")]
    pub allowed_self_trade_prevention_modes: Option<Vec<String>>,
}

/// 冻结响应中的嵌套对象。
#[doc(hidden)]
#[derive(Debug, Clone, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SpotExchangeInfoSorsItem {
    /// 响应字段 `baseAsset`。
    #[serde(rename = "baseAsset")]
    pub base_asset: Option<String>,
    /// 响应字段 `symbols`。
    #[serde(rename = "symbols")]
    pub symbols: Option<Vec<String>>,
}

/// 对应冻结端点：BN-SPOT-REST-001。
///
/// 待核实：T_PLUS_SELL 的 OpenAPI 分支列出 `endTime`，但未约束判别值且允许其他已登记字段；分支字段集仍开放。
///
/// 冻结的 SpotExchangeInfo 响应对象。
#[derive(Debug, Clone, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SpotExchangeInfo {
    /// 响应字段 `timezone`。
    #[serde(rename = "timezone")]
    pub timezone: Option<String>,
    /// 响应字段 `serverTime`。
    #[serde(rename = "serverTime")]
    pub server_time: Option<i64>,
    /// 响应字段 `rateLimits`。
    #[serde(rename = "rateLimits")]
    pub rate_limits: Option<Vec<SpotExchangeInfoRateLimitsItem>>,
    /// 响应字段 `exchangeFilters`。
    #[serde(rename = "exchangeFilters")]
    pub exchange_filters: Option<Vec<SpotExchangeInfoExchangeFiltersItem>>,
    /// 响应字段 `symbols`。
    #[serde(rename = "symbols")]
    pub symbols: Option<Vec<SpotExchangeInfoSymbolsItem>>,
    /// 响应字段 `sors`。
    #[serde(rename = "sors")]
    pub sors: Option<Vec<SpotExchangeInfoSorsItem>>,
}

/// 冻结响应中的嵌套对象。
#[doc(hidden)]
#[derive(Debug, Clone, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SpotExecutionRulesSymbolRulesItemRulesItem {
    /// 响应字段 `ruleType`。
    #[serde(rename = "ruleType")]
    pub rule_type: Option<String>,
    /// 响应字段 `bidLimitMultUp`。
    #[serde(rename = "bidLimitMultUp")]
    pub bid_limit_mult_up: Option<String>,
    /// 响应字段 `bidLimitMultDown`。
    #[serde(rename = "bidLimitMultDown")]
    pub bid_limit_mult_down: Option<String>,
    /// 响应字段 `askLimitMultUp`。
    #[serde(rename = "askLimitMultUp")]
    pub ask_limit_mult_up: Option<String>,
    /// 响应字段 `askLimitMultDown`。
    #[serde(rename = "askLimitMultDown")]
    pub ask_limit_mult_down: Option<String>,
}

/// 冻结响应中的嵌套对象。
#[doc(hidden)]
#[derive(Debug, Clone, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SpotExecutionRulesSymbolRulesItem {
    /// 响应字段 `symbol`。
    #[serde(rename = "symbol")]
    pub symbol: Option<String>,
    /// 响应字段 `rules`。
    #[serde(rename = "rules")]
    pub rules: Option<Vec<SpotExecutionRulesSymbolRulesItemRulesItem>>,
}

/// 对应冻结端点：BN-SPOT-REST-002。
/// 冻结的 SpotExecutionRules 响应对象。
#[derive(Debug, Clone, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SpotExecutionRules {
    /// 响应字段 `symbolRules`。
    #[serde(rename = "symbolRules")]
    pub symbol_rules: Option<Vec<SpotExecutionRulesSymbolRulesItem>>,
}

/// 冻结响应中的嵌套对象。
#[doc(hidden)]
#[derive(Debug, Clone, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SpotAggTradeItem {
    /// 响应字段 `a`。
    #[serde(rename = "a")]
    pub a: Option<i64>,
    /// 响应字段 `p`。
    #[serde(rename = "p")]
    pub p: Option<String>,
    /// 响应字段 `q`。
    #[serde(rename = "q")]
    pub q: Option<String>,
    /// 响应字段 `f`。
    #[serde(rename = "f")]
    pub f: Option<i64>,
    /// 响应字段 `l`。
    #[serde(rename = "l")]
    pub l: Option<i64>,
    /// 响应字段 `T`。
    #[serde(rename = "T")]
    pub t_upper: Option<i64>,
    /// 响应字段 `m`。
    #[serde(rename = "m")]
    pub m: Option<bool>,
    /// 响应字段 `M`。
    #[serde(rename = "M")]
    pub m_upper: Option<bool>,
}

/// 对应冻结端点：BN-SPOT-REST-005。
/// 冻结的 SpotAggTrade 响应集合。
pub type SpotAggTrade = Vec<SpotAggTradeItem>;

/// 对应冻结端点：BN-SPOT-REST-006。
/// 冻结的 SpotAvgPrice 响应对象。
#[derive(Debug, Clone, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SpotAvgPrice {
    /// 响应字段 `mins`。
    #[serde(rename = "mins")]
    pub mins: Option<i64>,
    /// 响应字段 `price`。
    #[serde(rename = "price")]
    pub price: Option<String>,
    /// 响应字段 `closeTime`。
    #[serde(rename = "closeTime")]
    pub close_time: Option<i64>,
}

/// 冻结响应中的嵌套对象。
#[doc(hidden)]
#[derive(Debug, Clone, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SpotTradeItem {
    /// 响应字段 `id`。
    #[serde(rename = "id")]
    pub id: Option<i64>,
    /// 响应字段 `price`。
    #[serde(rename = "price")]
    pub price: Option<String>,
    /// 响应字段 `qty`。
    #[serde(rename = "qty")]
    pub qty: Option<String>,
    /// 响应字段 `quoteQty`。
    #[serde(rename = "quoteQty")]
    pub quote_qty: Option<String>,
    /// 响应字段 `time`。
    #[serde(rename = "time")]
    pub time: Option<i64>,
    /// 响应字段 `isBuyerMaker`。
    #[serde(rename = "isBuyerMaker")]
    pub is_buyer_maker: Option<bool>,
    /// 响应字段 `isBestMatch`。
    #[serde(rename = "isBestMatch")]
    pub is_best_match: Option<bool>,
}

/// 对应冻结端点：BN-SPOT-REST-008、BN-SPOT-REST-009。
/// 冻结的 SpotTrade 响应集合。
pub type SpotTrade = Vec<SpotTradeItem>;

/// 冻结响应中的嵌套对象。
#[doc(hidden)]
#[derive(Debug, Clone, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SpotBlockTradeItem {
    /// 响应字段 `id`。
    #[serde(rename = "id")]
    pub id: Option<i64>,
    /// 响应字段 `price`。
    #[serde(rename = "price")]
    pub price: Option<String>,
    /// 响应字段 `qty`。
    #[serde(rename = "qty")]
    pub qty: Option<String>,
    /// 响应字段 `quoteQty`。
    #[serde(rename = "quoteQty")]
    pub quote_qty: Option<String>,
    /// 响应字段 `time`。
    #[serde(rename = "time")]
    pub time: Option<i64>,
    /// 响应字段 `isBuyerMaker`。
    #[serde(rename = "isBuyerMaker")]
    pub is_buyer_maker: Option<bool>,
}

/// 对应冻结端点：BN-SPOT-REST-010。
/// 冻结的 SpotBlockTrade 响应集合。
pub type SpotBlockTrade = Vec<SpotBlockTradeItem>;

crate::value::typed_kline!(SpotKline, "Spot K 线响应，来自 BN-SPOT-REST-011。");

crate::value::typed_kline!(SpotUiKline, "Spot UI K 线响应，来自 BN-SPOT-REST-017。");

/// 冻结响应中的嵌套对象。
#[doc(hidden)]
#[derive(Debug, Clone, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SpotTickerItem {
    /// 响应字段 `symbol`。
    #[serde(rename = "symbol")]
    pub symbol: Option<String>,
    /// 响应字段 `priceChange`。
    #[serde(rename = "priceChange")]
    pub price_change: Option<String>,
    /// 响应字段 `priceChangePercent`。
    #[serde(rename = "priceChangePercent")]
    pub price_change_percent: Option<String>,
    /// 响应字段 `weightedAvgPrice`。
    #[serde(rename = "weightedAvgPrice")]
    pub weighted_avg_price: Option<String>,
    /// 响应字段 `openPrice`。
    #[serde(rename = "openPrice")]
    pub open_price: Option<String>,
    /// 响应字段 `highPrice`。
    #[serde(rename = "highPrice")]
    pub high_price: Option<String>,
    /// 响应字段 `lowPrice`。
    #[serde(rename = "lowPrice")]
    pub low_price: Option<String>,
    /// 响应字段 `lastPrice`。
    #[serde(rename = "lastPrice")]
    pub last_price: Option<String>,
    /// 响应字段 `volume`。
    #[serde(rename = "volume")]
    pub volume: Option<String>,
    /// 响应字段 `quoteVolume`。
    #[serde(rename = "quoteVolume")]
    pub quote_volume: Option<String>,
    /// 响应字段 `openTime`。
    #[serde(rename = "openTime")]
    pub open_time: Option<i64>,
    /// 响应字段 `closeTime`。
    #[serde(rename = "closeTime")]
    pub close_time: Option<i64>,
    /// 响应字段 `firstId`。
    #[serde(rename = "firstId")]
    pub first_id: Option<i64>,
    /// 响应字段 `lastId`。
    #[serde(rename = "lastId")]
    pub last_id: Option<i64>,
    /// 响应字段 `count`。
    #[serde(rename = "count")]
    pub count: Option<i64>,
}

/// 对应冻结端点：BN-SPOT-REST-012。
///
/// 待核实：Variant 2（type=array）元素字段表未在渲染源单独列出（全文件无 Properties for Variant 2 段）；array 形 item 按 Variant 1 object 形状推断
///
/// 待核实：数组形态触发条件（symbols → 数组形）未在源页显式说明，页面仅要求 symbol/symbols 二选一提供；按参数语义与 Decision Table 双形推断
///
/// 待核实：Example Responses 中数值字段以 JSON number 出现（如 priceChange: -8）而字段表声明 string；类型以字段表为准，示例差异待官方澄清
/// 按响应根节点区分单对象与数组形态。
#[derive(Debug, Clone, serde::Deserialize)]
#[serde(untagged)]
pub enum SpotTicker {
    /// symbol 参数提供（单标的）。
    Object(Box<SpotTickerItem>),
    /// symbols 参数提供（多标的，至多 100）。
    Array(Vec<SpotTickerItem>),
}

/// 冻结响应中的嵌套对象。
#[doc(hidden)]
#[derive(Debug, Clone, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SpotTicker24hrItem {
    /// 响应字段 `symbol`。
    #[serde(rename = "symbol")]
    pub symbol: Option<String>,
    /// 响应字段 `priceChange`。
    #[serde(rename = "priceChange")]
    pub price_change: Option<String>,
    /// 响应字段 `priceChangePercent`。
    #[serde(rename = "priceChangePercent")]
    pub price_change_percent: Option<String>,
    /// 响应字段 `weightedAvgPrice`。
    #[serde(rename = "weightedAvgPrice")]
    pub weighted_avg_price: Option<String>,
    /// 响应字段 `prevClosePrice`。
    #[serde(rename = "prevClosePrice")]
    pub prev_close_price: Option<String>,
    /// 响应字段 `lastPrice`。
    #[serde(rename = "lastPrice")]
    pub last_price: Option<String>,
    /// 响应字段 `lastQty`。
    #[serde(rename = "lastQty")]
    pub last_qty: Option<String>,
    /// 响应字段 `bidPrice`。
    #[serde(rename = "bidPrice")]
    pub bid_price: Option<String>,
    /// 响应字段 `bidQty`。
    #[serde(rename = "bidQty")]
    pub bid_qty: Option<String>,
    /// 响应字段 `askPrice`。
    #[serde(rename = "askPrice")]
    pub ask_price: Option<String>,
    /// 响应字段 `askQty`。
    #[serde(rename = "askQty")]
    pub ask_qty: Option<String>,
    /// 响应字段 `openPrice`。
    #[serde(rename = "openPrice")]
    pub open_price: Option<String>,
    /// 响应字段 `highPrice`。
    #[serde(rename = "highPrice")]
    pub high_price: Option<String>,
    /// 响应字段 `lowPrice`。
    #[serde(rename = "lowPrice")]
    pub low_price: Option<String>,
    /// 响应字段 `volume`。
    #[serde(rename = "volume")]
    pub volume: Option<String>,
    /// 响应字段 `quoteVolume`。
    #[serde(rename = "quoteVolume")]
    pub quote_volume: Option<String>,
    /// 响应字段 `openTime`。
    #[serde(rename = "openTime")]
    pub open_time: Option<i64>,
    /// 响应字段 `closeTime`。
    #[serde(rename = "closeTime")]
    pub close_time: Option<i64>,
    /// 响应字段 `firstId`。
    #[serde(rename = "firstId")]
    pub first_id: Option<i64>,
    /// 响应字段 `lastId`。
    #[serde(rename = "lastId")]
    pub last_id: Option<i64>,
    /// 响应字段 `count`。
    #[serde(rename = "count")]
    pub count: Option<i64>,
}

/// 对应冻结端点：BN-SPOT-REST-013。
///
/// 待核实：Variant 2（type=array）元素字段表未在渲染源单独列出（全文件无 Properties for Variant 2 段）；array 形 item 按 Variant 1 object 形状推断
///
/// 待核实：数组形态触发条件未在源页显式说明（参数描述要求 symbol/symbols 二选一，但权重表另列有 symbol/symbols 省略情形）；按参数语义与 Decision Table 双形推断，省略时为全标的数组
///
/// 待核实：Example Responses 中数值字段以 JSON number 出现（如 priceChange: -94.999998）而字段表声明 string；类型以字段表为准，示例差异待官方澄清
/// 按响应根节点区分单对象与数组形态。
#[derive(Debug, Clone, serde::Deserialize)]
#[serde(untagged)]
pub enum SpotTicker24hr {
    /// symbol 参数提供（单标的）。
    Object(Box<SpotTicker24hrItem>),
    /// symbols 参数提供（多标的，至多 100）或 symbol/symbols 均省略（全标的）。
    Array(Vec<SpotTicker24hrItem>),
}

/// 冻结响应中的嵌套对象。
#[doc(hidden)]
#[derive(Debug, Clone, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SpotTickerPriceItem {
    /// 响应字段 `symbol`。
    #[serde(rename = "symbol")]
    pub symbol: Option<String>,
    /// 响应字段 `price`。
    #[serde(rename = "price")]
    pub price: Option<String>,
}

/// 对应冻结端点：BN-SPOT-REST-015。
///
/// 待核实：Variant 2（type=array）元素字段表未在渲染源单独列出（全文件无 Properties for Variant 2 段）；array 形 item 按 Variant 1 object 形状推断
///
/// 待核实：Example 数值以 JSON number 出现而字段表声明 string，类型以字段表为准，待官方澄清（与 012/013/016 同口径）
/// 按响应根节点区分单对象与数组形态。
#[derive(Debug, Clone, serde::Deserialize)]
#[serde(untagged)]
pub enum SpotTickerPrice {
    /// symbol 参数提供（单标的）。
    Object(Box<SpotTickerPriceItem>),
    /// symbols 参数提供（多标的）或两参数均未发送（全标的；页面明示 neither 参数发送时返回全标的数组）。
    Array(Vec<SpotTickerPriceItem>),
}

/// 冻结响应中的嵌套对象。
#[doc(hidden)]
#[derive(Debug, Clone, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SpotBookTickerItem {
    /// 响应字段 `symbol`。
    #[serde(rename = "symbol")]
    pub symbol: Option<String>,
    /// 响应字段 `bidPrice`。
    #[serde(rename = "bidPrice")]
    pub bid_price: Option<String>,
    /// 响应字段 `bidQty`。
    #[serde(rename = "bidQty")]
    pub bid_qty: Option<String>,
    /// 响应字段 `askPrice`。
    #[serde(rename = "askPrice")]
    pub ask_price: Option<String>,
    /// 响应字段 `askQty`。
    #[serde(rename = "askQty")]
    pub ask_qty: Option<String>,
}

/// Spot 冻结深度快照响应类型 SpotBookSnapshot。
#[derive(Debug, Clone, PartialEq, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SpotBookSnapshot {
    /// 原始响应字段 lastUpdateId；未提供时为 None。
    #[serde(rename = "lastUpdateId")]
    pub last_update_id: Option<i64>,
    /// 原始响应字段 bids；每档为价格与数量二元组。
    #[serde(rename = "bids")]
    pub bids: Option<Vec<(String, String)>>,
    /// 原始响应字段 asks；每档为价格与数量二元组。
    #[serde(rename = "asks")]
    pub asks: Option<Vec<(String, String)>>,
}

/// 对应冻结端点：BN-SPOT-REST-014。
///
/// 待核实：Variant 2（type=array）元素字段表未在渲染源单独列出（全文件无 Properties for Variant 2 段）；array 形 item 按 Variant 1 object 形状推断
///
/// 待核实：Example 数值以 JSON number 出现而字段表声明 string，类型以字段表为准，待官方澄清（与 012/013/016 同口径）
/// 按响应根节点区分单对象与数组形态。
#[derive(Debug, Clone, serde::Deserialize)]
#[serde(untagged)]
pub enum SpotBookTicker {
    /// symbol 参数提供（单标的）。
    Object(Box<SpotBookTickerItem>),
    /// symbols 参数提供（多标的）或两参数均未发送（全标的；页面明示 neither 参数发送时返回全标的数组）。
    Array(Vec<SpotBookTickerItem>),
}

/// 冻结响应中的嵌套对象。
#[doc(hidden)]
#[derive(Debug, Clone, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SpotTradingDayItem {
    /// 响应字段 `symbol`。
    #[serde(rename = "symbol")]
    pub symbol: Option<String>,
    /// 响应字段 `priceChange`。
    #[serde(rename = "priceChange")]
    pub price_change: Option<String>,
    /// 响应字段 `priceChangePercent`。
    #[serde(rename = "priceChangePercent")]
    pub price_change_percent: Option<String>,
    /// 响应字段 `weightedAvgPrice`。
    #[serde(rename = "weightedAvgPrice")]
    pub weighted_avg_price: Option<String>,
    /// 响应字段 `openPrice`。
    #[serde(rename = "openPrice")]
    pub open_price: Option<String>,
    /// 响应字段 `highPrice`。
    #[serde(rename = "highPrice")]
    pub high_price: Option<String>,
    /// 响应字段 `lowPrice`。
    #[serde(rename = "lowPrice")]
    pub low_price: Option<String>,
    /// 响应字段 `lastPrice`。
    #[serde(rename = "lastPrice")]
    pub last_price: Option<String>,
    /// 响应字段 `volume`。
    #[serde(rename = "volume")]
    pub volume: Option<String>,
    /// 响应字段 `quoteVolume`。
    #[serde(rename = "quoteVolume")]
    pub quote_volume: Option<String>,
    /// 响应字段 `openTime`。
    #[serde(rename = "openTime")]
    pub open_time: Option<i64>,
    /// 响应字段 `closeTime`。
    #[serde(rename = "closeTime")]
    pub close_time: Option<i64>,
    /// 响应字段 `firstId`。
    #[serde(rename = "firstId")]
    pub first_id: Option<i64>,
    /// 响应字段 `lastId`。
    #[serde(rename = "lastId")]
    pub last_id: Option<i64>,
    /// 响应字段 `count`。
    #[serde(rename = "count")]
    pub count: Option<i64>,
}

/// 对应冻结端点：BN-SPOT-REST-016。
///
/// 待核实：Variant 2（type=array）元素字段表未在渲染源单独列出（全文件无 Properties for Variant 2 段）；array 形 item 按 Variant 1 object 形状推断
///
/// 待核实：数组形态触发条件（symbols → 数组形）未在源页显式说明，页面仅要求 symbol/symbols 二选一提供；按参数语义与 Decision Table 双形推断
///
/// 待核实：Example Responses 中数值字段以 JSON number 出现（如 priceChange: -83.13）而字段表声明 string；类型以字段表为准，示例差异待官方澄清
/// 按响应根节点区分单对象与数组形态。
#[derive(Debug, Clone, serde::Deserialize)]
#[serde(untagged)]
pub enum SpotTradingDay {
    /// symbol 参数提供（单标的）。
    Object(Box<SpotTradingDayItem>),
    /// symbols 参数提供（多标的，至多 100）。
    Array(Vec<SpotTradingDayItem>),
}
