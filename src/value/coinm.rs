//! COINM 族冻结响应类型。
//!
//! 字段与根形状逐项对应响应结构合同，未标记必需的字段保留为 `Option`。
//! 所有对象拒绝未知字段；字符串数值保持源字符串，K 线保持十二项元组。
//!
//! 合同待核实项：交易所过滤器的字段表仍为混排集合，各 `filterType` 的字段归属
//! 尚未冻结，因此保留合同列出的可选字段，不推断枚举分支。
//! `timezone` 归根对象；`exchangeFilters` 与 `underlyingSubType` 按字段表
//! 使用字符串数组，不采纳示例中的异常嵌套空数组。

#![forbid(unsafe_code)]

/// 冻结响应中的嵌套对象。
#[doc(hidden)]
#[derive(Debug, Clone, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CoinmExchangeInfoRateLimitsItem {
    /// 源字段 `interval`。
    #[serde(rename = "interval")]
    pub interval: Option<String>,
    /// 源字段 `intervalNum`。
    #[serde(rename = "intervalNum")]
    pub interval_num: Option<i64>,
    /// 源字段 `limit`。
    #[serde(rename = "limit")]
    pub limit: Option<i64>,
    /// 源字段 `rateLimitType`。
    #[serde(rename = "rateLimitType")]
    pub rate_limit_type: Option<String>,
}

/// 冻结响应中的嵌套对象。
#[doc(hidden)]
#[derive(Debug, Clone, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CoinmExchangeInfoSymbolsItemFiltersItem {
    /// 源字段 `filterType`。
    #[serde(rename = "filterType")]
    pub filter_type: Option<String>,
    /// 源字段 `maxPrice`。
    #[serde(rename = "maxPrice")]
    pub max_price: Option<String>,
    /// 源字段 `minPrice`。
    #[serde(rename = "minPrice")]
    pub min_price: Option<String>,
    /// 源字段 `tickSize`。
    #[serde(rename = "tickSize")]
    pub tick_size: Option<String>,
    /// 源字段 `maxQty`。
    #[serde(rename = "maxQty")]
    pub max_qty: Option<String>,
    /// 源字段 `minQty`。
    #[serde(rename = "minQty")]
    pub min_qty: Option<String>,
    /// 源字段 `stepSize`。
    #[serde(rename = "stepSize")]
    pub step_size: Option<String>,
    /// 源字段 `limit`。
    #[serde(rename = "limit")]
    pub limit: Option<i64>,
    /// 源字段 `multiplierUp`。
    #[serde(rename = "multiplierUp")]
    pub multiplier_up: Option<String>,
    /// 源字段 `multiplierDown`。
    #[serde(rename = "multiplierDown")]
    pub multiplier_down: Option<String>,
    /// 源字段 `multiplierDecimal`。
    #[serde(rename = "multiplierDecimal")]
    pub multiplier_decimal: Option<String>,
}

/// 冻结响应中的嵌套对象。
#[doc(hidden)]
#[derive(Debug, Clone, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CoinmExchangeInfoSymbolsItem {
    /// 源字段 `filters`。
    #[serde(rename = "filters")]
    pub filters: Option<Vec<CoinmExchangeInfoSymbolsItemFiltersItem>>,
    /// 源字段 `orderTypes`。
    #[serde(rename = "orderTypes")]
    pub order_types: Option<Vec<String>>,
    /// 源字段 `timeInForce`。
    #[serde(rename = "timeInForce")]
    pub time_in_force: Option<Vec<String>>,
    /// 源字段 `liquidationFee`。
    #[serde(rename = "liquidationFee")]
    pub liquidation_fee: Option<String>,
    /// 源字段 `marketTakeBound`。
    #[serde(rename = "marketTakeBound")]
    pub market_take_bound: Option<String>,
    /// 源字段 `symbol`。
    #[serde(rename = "symbol")]
    pub symbol: Option<String>,
    /// 源字段 `pair`。
    #[serde(rename = "pair")]
    pub pair: Option<String>,
    /// 源字段 `contractType`。
    #[serde(rename = "contractType")]
    pub contract_type: Option<String>,
    /// 源字段 `deliveryDate`。
    #[serde(rename = "deliveryDate")]
    pub delivery_date: Option<i64>,
    /// 源字段 `onboardDate`。
    #[serde(rename = "onboardDate")]
    pub onboard_date: Option<i64>,
    /// 源字段 `contractStatus`。
    #[serde(rename = "contractStatus")]
    pub contract_status: Option<String>,
    /// 源字段 `contractSize`。
    #[serde(rename = "contractSize")]
    pub contract_size: Option<i64>,
    /// 源字段 `quoteAsset`。
    #[serde(rename = "quoteAsset")]
    pub quote_asset: Option<String>,
    /// 源字段 `baseAsset`。
    #[serde(rename = "baseAsset")]
    pub base_asset: Option<String>,
    /// 源字段 `marginAsset`。
    #[serde(rename = "marginAsset")]
    pub margin_asset: Option<String>,
    /// 源字段 `pricePrecision`。
    #[serde(rename = "pricePrecision")]
    pub price_precision: Option<i64>,
    /// 源字段 `quantityPrecision`。
    #[serde(rename = "quantityPrecision")]
    pub quantity_precision: Option<i64>,
    /// 源字段 `baseAssetPrecision`。
    #[serde(rename = "baseAssetPrecision")]
    pub base_asset_precision: Option<i64>,
    /// 源字段 `quotePrecision`。
    #[serde(rename = "quotePrecision")]
    pub quote_precision: Option<i64>,
    /// 源字段 `equalQtyPrecision`。
    #[serde(rename = "equalQtyPrecision")]
    pub equal_qty_precision: Option<i64>,
    /// 源字段 `triggerProtect`。
    #[serde(rename = "triggerProtect")]
    pub trigger_protect: Option<String>,
    /// 源字段 `maintMarginPercent`。
    #[serde(rename = "maintMarginPercent")]
    pub maint_margin_percent: Option<String>,
    /// 源字段 `requiredMarginPercent`。
    #[serde(rename = "requiredMarginPercent")]
    pub required_margin_percent: Option<String>,
    /// 源字段 `underlyingType`。
    #[serde(rename = "underlyingType")]
    pub underlying_type: Option<String>,
    /// 源字段 `underlyingSubType`。
    #[serde(rename = "underlyingSubType")]
    pub underlying_sub_type: Option<Vec<String>>,
}

/// 冻结响应对象。
#[derive(Debug, Clone, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CoinmExchangeInfo {
    /// 源字段 `exchangeFilters`。
    #[serde(rename = "exchangeFilters")]
    pub exchange_filters: Option<Vec<String>>,
    /// 源字段 `rateLimits`。
    #[serde(rename = "rateLimits")]
    pub rate_limits: Option<Vec<CoinmExchangeInfoRateLimitsItem>>,
    /// 源字段 `serverTime`。
    #[serde(rename = "serverTime")]
    pub server_time: Option<i64>,
    /// 源字段 `symbols`。
    #[serde(rename = "symbols")]
    pub symbols: Option<Vec<CoinmExchangeInfoSymbolsItem>>,
    /// 源字段 `timezone`。
    #[serde(rename = "timezone")]
    pub timezone: Option<String>,
}

/// 冻结响应中的嵌套对象。
#[doc(hidden)]
#[derive(Debug, Clone, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CoinmAggTradeItem {
    /// 源字段 `a`。
    #[serde(rename = "a")]
    pub a: Option<i64>,
    /// 源字段 `p`。
    #[serde(rename = "p")]
    pub p: Option<String>,
    /// 源字段 `q`。
    #[serde(rename = "q")]
    pub q: Option<String>,
    /// 源字段 `f`。
    #[serde(rename = "f")]
    pub f: Option<i64>,
    /// 源字段 `l`。
    #[serde(rename = "l")]
    pub l: Option<i64>,
    /// 源字段 `T`。
    #[serde(rename = "T")]
    pub t: Option<i64>,
    /// 源字段 `m`。
    #[serde(rename = "m")]
    pub m: Option<bool>,
}

/// 冻结合同中的完整响应。
pub type CoinmAggTrade = Vec<CoinmAggTradeItem>;

/// 冻结响应中的嵌套对象。
#[doc(hidden)]
#[derive(Debug, Clone, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CoinmTradeItem {
    /// 源字段 `id`。
    #[serde(rename = "id")]
    pub id: Option<i64>,
    /// 源字段 `price`。
    #[serde(rename = "price")]
    pub price: Option<String>,
    /// 源字段 `qty`。
    #[serde(rename = "qty")]
    pub qty: Option<String>,
    /// 源字段 `baseQty`。
    #[serde(rename = "baseQty")]
    pub base_qty: Option<String>,
    /// 源字段 `time`。
    #[serde(rename = "time")]
    pub time: Option<i64>,
    /// 源字段 `isBuyerMaker`。
    #[serde(rename = "isBuyerMaker")]
    pub is_buyer_maker: Option<bool>,
}

/// 冻结合同中的完整响应。
pub type CoinmTrade = Vec<CoinmTradeItem>;

crate::value::typed_kline!(CoinmContinuousKline, "COIN-M 连续合约 K 线响应。");

crate::value::typed_kline!(CoinmKline, "COIN-M 原生 K 线响应。");

crate::value::typed_kline!(CoinmIndexPriceKline, "COIN-M 指数价格 K 线响应。");

crate::value::typed_kline!(CoinmMarkPriceKline, "COIN-M 标记价格 K 线响应。");

/// 冻结响应中的嵌套对象。
#[doc(hidden)]
#[derive(Debug, Clone, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CoinmPremiumIndexItem {
    /// 源字段 `symbol`。
    #[serde(rename = "symbol")]
    pub symbol: Option<String>,
    /// 源字段 `pair`。
    #[serde(rename = "pair")]
    pub pair: Option<String>,
    /// 源字段 `markPrice`。
    #[serde(rename = "markPrice")]
    pub mark_price: Option<String>,
    /// 源字段 `indexPrice`。
    #[serde(rename = "indexPrice")]
    pub index_price: Option<String>,
    /// 源字段 `estimatedSettlePrice`。
    #[serde(rename = "estimatedSettlePrice")]
    pub estimated_settle_price: Option<String>,
    /// 源字段 `lastFundingRate`。
    #[serde(rename = "lastFundingRate")]
    pub last_funding_rate: Option<String>,
    /// 源字段 `interestRate`。
    #[serde(rename = "interestRate")]
    pub interest_rate: Option<String>,
    /// 源字段 `nextFundingTime`。
    #[serde(rename = "nextFundingTime")]
    pub next_funding_time: Option<i64>,
    /// 源字段 `time`。
    #[serde(rename = "time")]
    pub time: Option<i64>,
}

/// 冻结合同中的完整响应。
pub type CoinmPremiumIndex = Vec<CoinmPremiumIndexItem>;

crate::value::typed_kline!(CoinmPremiumIndexKline, "COIN-M 溢价指数 K 线响应。");

/// 冻结响应中的嵌套对象。
#[doc(hidden)]
#[derive(Debug, Clone, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CoinmFundingRateItem {
    /// 源字段 `symbol`。
    #[serde(rename = "symbol")]
    pub symbol: Option<String>,
    /// 源字段 `fundingTime`。
    #[serde(rename = "fundingTime")]
    pub funding_time: Option<i64>,
    /// 源字段 `fundingRate`。
    #[serde(rename = "fundingRate")]
    pub funding_rate: Option<String>,
}

/// 冻结合同中的完整响应。
pub type CoinmFundingRate = Vec<CoinmFundingRateItem>;

/// 冻结响应中的嵌套对象。
#[doc(hidden)]
#[derive(Debug, Clone, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CoinmFundingInfoItem {
    /// 源字段 `symbol`。
    #[serde(rename = "symbol")]
    pub symbol: Option<String>,
    /// 源字段 `adjustedFundingRateCap`。
    #[serde(rename = "adjustedFundingRateCap")]
    pub adjusted_funding_rate_cap: Option<String>,
    /// 源字段 `adjustedFundingRateFloor`。
    #[serde(rename = "adjustedFundingRateFloor")]
    pub adjusted_funding_rate_floor: Option<String>,
    /// 源字段 `fundingIntervalHours`。
    #[serde(rename = "fundingIntervalHours")]
    pub funding_interval_hours: Option<i64>,
    /// 源字段 `disclaimer`。
    #[serde(rename = "disclaimer")]
    pub disclaimer: Option<bool>,
}

/// 冻结合同中的完整响应。
pub type CoinmFundingInfo = Vec<CoinmFundingInfoItem>;

/// 冻结响应对象。
#[derive(Debug, Clone, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CoinmOpenInterest {
    /// 源字段 `symbol`。
    #[serde(rename = "symbol")]
    pub symbol: Option<String>,
    /// 源字段 `pair`。
    #[serde(rename = "pair")]
    pub pair: Option<String>,
    /// 源字段 `openInterest`。
    #[serde(rename = "openInterest")]
    pub open_interest: Option<String>,
    /// 源字段 `contractType`。
    #[serde(rename = "contractType")]
    pub contract_type: Option<String>,
    /// 源字段 `time`。
    #[serde(rename = "time")]
    pub time: Option<i64>,
}

/// 冻结响应中的嵌套对象。
#[doc(hidden)]
#[derive(Debug, Clone, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CoinmOpenInterestHistItem {
    /// 源字段 `pair`。
    #[serde(rename = "pair")]
    pub pair: Option<String>,
    /// 源字段 `contractType`。
    #[serde(rename = "contractType")]
    pub contract_type: Option<String>,
    /// 源字段 `sumOpenInterest`。
    #[serde(rename = "sumOpenInterest")]
    pub sum_open_interest: Option<String>,
    /// 源字段 `sumOpenInterestValue`。
    #[serde(rename = "sumOpenInterestValue")]
    pub sum_open_interest_value: Option<String>,
    /// 源字段 `timestamp`。
    #[serde(rename = "timestamp")]
    pub timestamp: Option<i64>,
}

/// 冻结合同中的完整响应。
pub type CoinmOpenInterestHist = Vec<CoinmOpenInterestHistItem>;

/// 冻结响应中的嵌套对象。
#[doc(hidden)]
#[derive(Debug, Clone, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CoinmBookTickerItem {
    /// 源字段 `lastUpdateId`。
    #[serde(rename = "lastUpdateId")]
    pub last_update_id: Option<i64>,
    /// 源字段 `symbol`。
    #[serde(rename = "symbol")]
    pub symbol: Option<String>,
    /// 源字段 `pair`。
    #[serde(rename = "pair")]
    pub pair: Option<String>,
    /// 源字段 `bidPrice`。
    #[serde(rename = "bidPrice")]
    pub bid_price: Option<String>,
    /// 源字段 `bidQty`。
    #[serde(rename = "bidQty")]
    pub bid_qty: Option<String>,
    /// 源字段 `askPrice`。
    #[serde(rename = "askPrice")]
    pub ask_price: Option<String>,
    /// 源字段 `askQty`。
    #[serde(rename = "askQty")]
    pub ask_qty: Option<String>,
    /// 源字段 `time`。
    #[serde(rename = "time")]
    pub time: Option<i64>,
}

/// 冻结合同中的完整响应。
pub type CoinmBookTicker = Vec<CoinmBookTickerItem>;

/// COINM 冻结深度快照响应类型 CoinmBookSnapshot。
#[derive(Debug, Clone, PartialEq, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CoinmBookSnapshot {
    /// 原始响应字段 lastUpdateId；未提供时为 None。
    #[serde(rename = "lastUpdateId")]
    pub last_update_id: Option<i64>,
    /// 原始响应字段 symbol；未提供时为 None。
    #[serde(rename = "symbol")]
    pub symbol: Option<String>,
    /// 原始响应字段 pair；未提供时为 None。
    #[serde(rename = "pair")]
    pub pair: Option<String>,
    /// 原始响应字段 E；未提供时为 None。
    #[serde(rename = "E")]
    pub e: Option<i64>,
    /// 原始响应字段 T；未提供时为 None。
    #[serde(rename = "T")]
    pub t: Option<i64>,
    /// 原始响应字段 bids；每档为价格与数量二元组。
    #[serde(rename = "bids")]
    pub bids: Option<Vec<(String, String)>>,
    /// 原始响应字段 asks；每档为价格与数量二元组。
    #[serde(rename = "asks")]
    pub asks: Option<Vec<(String, String)>>,
}

/// 冻结响应中的嵌套对象。
#[doc(hidden)]
#[derive(Debug, Clone, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CoinmTickerPriceItem {
    /// 源字段 `symbol`。
    #[serde(rename = "symbol")]
    pub symbol: Option<String>,
    /// 源字段 `ps`。
    #[serde(rename = "ps")]
    pub ps: Option<String>,
    /// 源字段 `price`。
    #[serde(rename = "price")]
    pub price: Option<String>,
    /// 源字段 `time`。
    #[serde(rename = "time")]
    pub time: Option<i64>,
}

/// 冻结合同中的完整响应。
pub type CoinmTickerPrice = Vec<CoinmTickerPriceItem>;

/// 冻结响应中的嵌套对象。
#[doc(hidden)]
#[derive(Debug, Clone, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CoinmTicker24hrItem {
    /// 源字段 `symbol`。
    #[serde(rename = "symbol")]
    pub symbol: Option<String>,
    /// 源字段 `pair`。
    #[serde(rename = "pair")]
    pub pair: Option<String>,
    /// 源字段 `priceChange`。
    #[serde(rename = "priceChange")]
    pub price_change: Option<String>,
    /// 源字段 `priceChangePercent`。
    #[serde(rename = "priceChangePercent")]
    pub price_change_percent: Option<String>,
    /// 源字段 `weightedAvgPrice`。
    #[serde(rename = "weightedAvgPrice")]
    pub weighted_avg_price: Option<String>,
    /// 源字段 `lastPrice`。
    #[serde(rename = "lastPrice")]
    pub last_price: Option<String>,
    /// 源字段 `lastQty`。
    #[serde(rename = "lastQty")]
    pub last_qty: Option<String>,
    /// 源字段 `openPrice`。
    #[serde(rename = "openPrice")]
    pub open_price: Option<String>,
    /// 源字段 `highPrice`。
    #[serde(rename = "highPrice")]
    pub high_price: Option<String>,
    /// 源字段 `lowPrice`。
    #[serde(rename = "lowPrice")]
    pub low_price: Option<String>,
    /// 源字段 `volume`。
    #[serde(rename = "volume")]
    pub volume: Option<String>,
    /// 源字段 `baseVolume`。
    #[serde(rename = "baseVolume")]
    pub base_volume: Option<String>,
    /// 源字段 `openTime`。
    #[serde(rename = "openTime")]
    pub open_time: Option<i64>,
    /// 源字段 `closeTime`。
    #[serde(rename = "closeTime")]
    pub close_time: Option<i64>,
    /// 源字段 `firstId`。
    #[serde(rename = "firstId")]
    pub first_id: Option<i64>,
    /// 源字段 `lastId`。
    #[serde(rename = "lastId")]
    pub last_id: Option<i64>,
    /// 源字段 `count`。
    #[serde(rename = "count")]
    pub count: Option<i64>,
}

/// 冻结合同中的完整响应。
pub type CoinmTicker24hr = Vec<CoinmTicker24hrItem>;

/// 冻结响应中的嵌套对象。
#[doc(hidden)]
#[derive(Debug, Clone, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CoinmTakerBuySellVolItem {
    /// 源字段 `pair`。
    #[serde(rename = "pair")]
    pub pair: Option<String>,
    /// 源字段 `contractType`。
    #[serde(rename = "contractType")]
    pub contract_type: Option<String>,
    /// 源字段 `takerBuyVol`。
    #[serde(rename = "takerBuyVol")]
    pub taker_buy_vol: Option<String>,
    /// 源字段 `takerSellVol`。
    #[serde(rename = "takerSellVol")]
    pub taker_sell_vol: Option<String>,
    /// 源字段 `takerBuyVolValue`。
    #[serde(rename = "takerBuyVolValue")]
    pub taker_buy_vol_value: Option<String>,
    /// 源字段 `takerSellVolValue`。
    #[serde(rename = "takerSellVolValue")]
    pub taker_sell_vol_value: Option<String>,
    /// 源字段 `timestamp`。
    #[serde(rename = "timestamp")]
    pub timestamp: Option<i64>,
}

/// 冻结合同中的完整响应。
pub type CoinmTakerBuySellVol = Vec<CoinmTakerBuySellVolItem>;

/// 冻结响应中的嵌套对象。
#[doc(hidden)]
#[derive(Debug, Clone, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CoinmGlobalLongShortAccountRatioItem {
    /// 源字段 `pair`。
    #[serde(rename = "pair")]
    pub pair: Option<String>,
    /// 源字段 `longShortRatio`。
    #[serde(rename = "longShortRatio")]
    pub long_short_ratio: Option<String>,
    /// 源字段 `longAccount`。
    #[serde(rename = "longAccount")]
    pub long_account: Option<String>,
    /// 源字段 `shortAccount`。
    #[serde(rename = "shortAccount")]
    pub short_account: Option<String>,
    /// 源字段 `timestamp`。
    #[serde(rename = "timestamp")]
    pub timestamp: Option<i64>,
}

/// 冻结合同中的完整响应。
pub type CoinmGlobalLongShortAccountRatio = Vec<CoinmGlobalLongShortAccountRatioItem>;

/// 冻结响应中的嵌套对象。
#[doc(hidden)]
#[derive(Debug, Clone, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CoinmTopLongShortAccountRatioItem {
    /// 源字段 `pair`。
    #[serde(rename = "pair")]
    pub pair: Option<String>,
    /// 源字段 `longShortRatio`。
    #[serde(rename = "longShortRatio")]
    pub long_short_ratio: Option<String>,
    /// 源字段 `longAccount`。
    #[serde(rename = "longAccount")]
    pub long_account: Option<String>,
    /// 源字段 `shortAccount`。
    #[serde(rename = "shortAccount")]
    pub short_account: Option<String>,
    /// 源字段 `timestamp`。
    #[serde(rename = "timestamp")]
    pub timestamp: Option<i64>,
}

/// 冻结合同中的完整响应。
pub type CoinmTopLongShortAccountRatio = Vec<CoinmTopLongShortAccountRatioItem>;

/// 冻结响应中的嵌套对象。
#[doc(hidden)]
#[derive(Debug, Clone, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CoinmTopLongShortPositionRatioItem {
    /// 源字段 `pair`。
    #[serde(rename = "pair")]
    pub pair: Option<String>,
    /// 源字段 `longShortRatio`。
    #[serde(rename = "longShortRatio")]
    pub long_short_ratio: Option<String>,
    /// 源字段 `longPosition`。
    #[serde(rename = "longPosition")]
    pub long_position: Option<String>,
    /// 源字段 `shortPosition`。
    #[serde(rename = "shortPosition")]
    pub short_position: Option<String>,
    /// 源字段 `timestamp`。
    #[serde(rename = "timestamp")]
    pub timestamp: Option<i64>,
}

/// 冻结合同中的完整响应。
pub type CoinmTopLongShortPositionRatio = Vec<CoinmTopLongShortPositionRatioItem>;

/// 冻结响应中的嵌套对象。
#[doc(hidden)]
#[derive(Debug, Clone, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CoinmBasisItem {
    /// 源字段 `indexPrice`。
    #[serde(rename = "indexPrice")]
    pub index_price: Option<String>,
    /// 源字段 `contractType`。
    #[serde(rename = "contractType")]
    pub contract_type: Option<String>,
    /// 源字段 `basisRate`。
    #[serde(rename = "basisRate")]
    pub basis_rate: Option<String>,
    /// 源字段 `futuresPrice`。
    #[serde(rename = "futuresPrice")]
    pub futures_price: Option<String>,
    /// 源字段 `annualizedBasisRate`。
    #[serde(rename = "annualizedBasisRate")]
    pub annualized_basis_rate: Option<String>,
    /// 源字段 `basis`。
    #[serde(rename = "basis")]
    pub basis: Option<String>,
    /// 源字段 `pair`。
    #[serde(rename = "pair")]
    pub pair: Option<String>,
    /// 源字段 `timestamp`。
    #[serde(rename = "timestamp")]
    pub timestamp: Option<i64>,
}

/// 冻结合同中的完整响应。
pub type CoinmBasis = Vec<CoinmBasisItem>;

/// 冻结响应中的嵌套对象。
#[doc(hidden)]
#[derive(Debug, Clone, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CoinmConstituentsConstituentsItem {
    /// 源字段 `exchange`。
    #[serde(rename = "exchange")]
    pub exchange: Option<String>,
    /// 源字段 `symbol`。
    #[serde(rename = "symbol")]
    pub symbol: Option<String>,
    /// 源字段 `price`。
    #[serde(rename = "price")]
    pub price: Option<String>,
    /// 源字段 `weight`。
    #[serde(rename = "weight")]
    pub weight: Option<String>,
}

/// 冻结响应对象。
#[derive(Debug, Clone, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CoinmConstituents {
    /// 源字段 `symbol`。
    #[serde(rename = "symbol")]
    pub symbol: Option<String>,
    /// 源字段 `time`。
    #[serde(rename = "time")]
    pub time: Option<i64>,
    /// 源字段 `constituents`。
    #[serde(rename = "constituents")]
    pub constituents: Option<Vec<CoinmConstituentsConstituentsItem>>,
}
