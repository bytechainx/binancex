//! Options 族冻结响应值对象。
//!
//! 对象字段按合同均为可选，未知字段拒绝；字符串数值保留原文。
//! `exchangeInfo` 的过滤器字段归属仍待核实；`timezone` 位于根对象，
//! `minQty` / `maxQty` 在交易对与过滤器两个层级分别保留。

/// 历史行权响应。
pub type OptionsExerciseHistory = Vec<OptionsExerciseHistoryItem>;

/// 期权持仓响应；timestamp 按冻结合同保留字符串。
pub type OptionsOpenInterest = Vec<OptionsOpenInterestItem>;

/// 期权标记价格与希腊字母响应。
pub type OptionsMark = Vec<OptionsMarkItem>;

/// 期权大宗交易响应。
pub type OptionsBlockTrade = Vec<OptionsBlockTradeItem>;

/// 期权近期交易响应。
pub type OptionsTrade = Vec<OptionsTradeItem>;

/// 期权二十四小时行情响应。
pub type OptionsTicker = Vec<OptionsTickerItem>;

crate::value::typed_kline!(
    OptionsKline,
    "期权 K 线响应，每行严格保留冻结合同的十二个位置。"
);

/// 期权交易所信息。
#[derive(Debug, Clone, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OptionsExchangeInfo {
    /// 源字段 `timezone`。
    #[serde(rename = "timezone")]
    pub timezone: Option<String>,
    /// 源字段 `serverTime`。
    #[serde(rename = "serverTime")]
    pub server_time: Option<i64>,
    /// 源字段 `optionContracts`。
    #[serde(rename = "optionContracts")]
    pub option_contracts: Option<Vec<OptionsOptionContracts>>,
    /// 源字段 `optionAssets`。
    #[serde(rename = "optionAssets")]
    pub option_assets: Option<Vec<OptionsOptionAssets>>,
    /// 源字段 `optionSymbols`。
    #[serde(rename = "optionSymbols")]
    pub option_symbols: Option<Vec<OptionsOptionSymbols>>,
    /// 源字段 `rateLimits`。
    #[serde(rename = "rateLimits")]
    pub rate_limits: Option<Vec<OptionsRateLimits>>,
}

/// 期权合约标的资产信息。
#[doc(hidden)]
#[derive(Debug, Clone, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OptionsOptionContracts {
    /// 源字段 `baseAsset`。
    #[serde(rename = "baseAsset")]
    pub base_asset: Option<String>,
    /// 源字段 `quoteAsset`。
    #[serde(rename = "quoteAsset")]
    pub quote_asset: Option<String>,
    /// 源字段 `underlying`。
    #[serde(rename = "underlying")]
    pub underlying: Option<String>,
    /// 源字段 `settleAsset`。
    #[serde(rename = "settleAsset")]
    pub settle_asset: Option<String>,
}

/// 期权资产信息。
#[doc(hidden)]
#[derive(Debug, Clone, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OptionsOptionAssets {
    /// 源字段 `name`。
    #[serde(rename = "name")]
    pub name: Option<String>,
}

/// 期权交易对信息；数量边界与 filters 内的数量边界各自保留。
#[doc(hidden)]
#[derive(Debug, Clone, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OptionsOptionSymbols {
    /// 源字段 `expiryDate`。
    #[serde(rename = "expiryDate")]
    pub expiry_date: Option<i64>,
    /// 源字段 `filters`。
    #[serde(rename = "filters")]
    pub filters: Option<Vec<OptionsFilters>>,
    /// 源字段 `symbol`。
    #[serde(rename = "symbol")]
    pub symbol: Option<String>,
    /// 源字段 `side`。
    #[serde(rename = "side")]
    pub side: Option<String>,
    /// 源字段 `strikePrice`。
    #[serde(rename = "strikePrice")]
    pub strike_price: Option<String>,
    /// 源字段 `underlying`。
    #[serde(rename = "underlying")]
    pub underlying: Option<String>,
    /// 源字段 `unit`。
    #[serde(rename = "unit")]
    pub unit: Option<i64>,
    /// 源字段 `liquidationFeeRate`。
    #[serde(rename = "liquidationFeeRate")]
    pub liquidation_fee_rate: Option<String>,
    /// 源字段 `minQty`。
    #[serde(rename = "minQty")]
    pub min_qty: Option<String>,
    /// 源字段 `maxQty`。
    #[serde(rename = "maxQty")]
    pub max_qty: Option<String>,
    /// 源字段 `initialMargin`。
    #[serde(rename = "initialMargin")]
    pub initial_margin: Option<String>,
    /// 源字段 `maintenanceMargin`。
    #[serde(rename = "maintenanceMargin")]
    pub maintenance_margin: Option<String>,
    /// 源字段 `minInitialMargin`。
    #[serde(rename = "minInitialMargin")]
    pub min_initial_margin: Option<String>,
    /// 源字段 `minMaintenanceMargin`。
    #[serde(rename = "minMaintenanceMargin")]
    pub min_maintenance_margin: Option<String>,
    /// 源字段 `priceScale`。
    #[serde(rename = "priceScale")]
    pub price_scale: Option<i64>,
    /// 源字段 `quantityScale`。
    #[serde(rename = "quantityScale")]
    pub quantity_scale: Option<i64>,
    /// 源字段 `quoteAsset`。
    #[serde(rename = "quoteAsset")]
    pub quote_asset: Option<String>,
    /// 源字段 `contractType`。
    #[serde(rename = "contractType")]
    pub contract_type: Option<String>,
    /// 源字段 `underlyingType`。
    #[serde(rename = "underlyingType")]
    pub underlying_type: Option<String>,
    /// 源字段 `nakedSell`。
    #[serde(rename = "nakedSell")]
    pub naked_sell: Option<bool>,
    /// 源字段 `status`。
    #[serde(rename = "status")]
    pub status: Option<String>,
}

/// 过滤器观测字段并集；各 filterType 字段归属尚待证据核实，暂不拆分变体。
#[doc(hidden)]
#[derive(Debug, Clone, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OptionsFilters {
    /// 源字段 `filterType`。
    #[serde(rename = "filterType")]
    pub filter_type: Option<String>,
    /// 源字段 `minPrice`。
    #[serde(rename = "minPrice")]
    pub min_price: Option<String>,
    /// 源字段 `maxPrice`。
    #[serde(rename = "maxPrice")]
    pub max_price: Option<String>,
    /// 源字段 `tickSize`。
    #[serde(rename = "tickSize")]
    pub tick_size: Option<String>,
    /// 源字段 `minQty`。
    #[serde(rename = "minQty")]
    pub min_qty: Option<String>,
    /// 源字段 `maxQty`。
    #[serde(rename = "maxQty")]
    pub max_qty: Option<String>,
    /// 源字段 `stepSize`。
    #[serde(rename = "stepSize")]
    pub step_size: Option<String>,
}

/// 请求频率限制。
#[doc(hidden)]
#[derive(Debug, Clone, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OptionsRateLimits {
    /// 源字段 `rateLimitType`。
    #[serde(rename = "rateLimitType")]
    pub rate_limit_type: Option<String>,
    /// 源字段 `interval`。
    #[serde(rename = "interval")]
    pub interval: Option<String>,
    /// 源字段 `intervalNum`。
    #[serde(rename = "intervalNum")]
    pub interval_num: Option<i64>,
    /// 源字段 `limit`。
    #[serde(rename = "limit")]
    pub limit: Option<i64>,
}

/// 历史行权记录。
#[doc(hidden)]
#[derive(Debug, Clone, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OptionsExerciseHistoryItem {
    /// 源字段 `symbol`。
    #[serde(rename = "symbol")]
    pub symbol: Option<String>,
    /// 源字段 `strikePrice`。
    #[serde(rename = "strikePrice")]
    pub strike_price: Option<String>,
    /// 源字段 `realStrikePrice`。
    #[serde(rename = "realStrikePrice")]
    pub real_strike_price: Option<String>,
    /// 源字段 `expiryDate`。
    #[serde(rename = "expiryDate")]
    pub expiry_date: Option<i64>,
    /// 源字段 `strikeResult`。
    #[serde(rename = "strikeResult")]
    pub strike_result: Option<String>,
}

/// 期权标的指数价格。
#[derive(Debug, Clone, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OptionsIndex {
    /// 源字段 `time`。
    #[serde(rename = "time")]
    pub time: Option<i64>,
    /// 源字段 `indexPrice`。
    #[serde(rename = "indexPrice")]
    pub index_price: Option<String>,
}

/// 期权持仓记录；timestamp 按冻结合同保留字符串。
#[doc(hidden)]
#[derive(Debug, Clone, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OptionsOpenInterestItem {
    /// 源字段 `symbol`。
    #[serde(rename = "symbol")]
    pub symbol: Option<String>,
    /// 源字段 `sumOpenInterest`。
    #[serde(rename = "sumOpenInterest")]
    pub sum_open_interest: Option<String>,
    /// 源字段 `sumOpenInterestUsd`。
    #[serde(rename = "sumOpenInterestUsd")]
    pub sum_open_interest_usd: Option<String>,
    /// 源字段 `timestamp`。
    #[serde(rename = "timestamp")]
    pub timestamp: Option<String>,
}

/// 期权标记价格与希腊字母记录。
#[doc(hidden)]
#[derive(Debug, Clone, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OptionsMarkItem {
    /// 源字段 `symbol`。
    #[serde(rename = "symbol")]
    pub symbol: Option<String>,
    /// 源字段 `markPrice`。
    #[serde(rename = "markPrice")]
    pub mark_price: Option<String>,
    /// 源字段 `bidIV`。
    #[serde(rename = "bidIV")]
    pub bid_iv: Option<String>,
    /// 源字段 `askIV`。
    #[serde(rename = "askIV")]
    pub ask_iv: Option<String>,
    /// 源字段 `markIV`。
    #[serde(rename = "markIV")]
    pub mark_iv: Option<String>,
    /// 源字段 `delta`。
    #[serde(rename = "delta")]
    pub delta: Option<String>,
    /// 源字段 `theta`。
    #[serde(rename = "theta")]
    pub theta: Option<String>,
    /// 源字段 `gamma`。
    #[serde(rename = "gamma")]
    pub gamma: Option<String>,
    /// 源字段 `vega`。
    #[serde(rename = "vega")]
    pub vega: Option<String>,
    /// 源字段 `highPriceLimit`。
    #[serde(rename = "highPriceLimit")]
    pub high_price_limit: Option<String>,
    /// 源字段 `lowPriceLimit`。
    #[serde(rename = "lowPriceLimit")]
    pub low_price_limit: Option<String>,
    /// 源字段 `riskFreeInterest`。
    #[serde(rename = "riskFreeInterest")]
    pub risk_free_interest: Option<String>,
}

/// 期权大宗交易记录。
#[doc(hidden)]
#[derive(Debug, Clone, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OptionsBlockTradeItem {
    /// 源字段 `id`。
    #[serde(rename = "id")]
    pub id: Option<i64>,
    /// 源字段 `tradeId`。
    #[serde(rename = "tradeId")]
    pub trade_id: Option<i64>,
    /// 源字段 `symbol`。
    #[serde(rename = "symbol")]
    pub symbol: Option<String>,
    /// 源字段 `price`。
    #[serde(rename = "price")]
    pub price: Option<String>,
    /// 源字段 `qty`。
    #[serde(rename = "qty")]
    pub qty: Option<String>,
    /// 源字段 `quoteQty`。
    #[serde(rename = "quoteQty")]
    pub quote_qty: Option<String>,
    /// 源字段 `side`。
    #[serde(rename = "side")]
    pub side: Option<i64>,
    /// 源字段 `time`。
    #[serde(rename = "time")]
    pub time: Option<i64>,
}

/// 期权近期交易记录。
#[doc(hidden)]
#[derive(Debug, Clone, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OptionsTradeItem {
    /// 源字段 `id`。
    #[serde(rename = "id")]
    pub id: Option<i64>,
    /// 源字段 `tradeId`。
    #[serde(rename = "tradeId")]
    pub trade_id: Option<i64>,
    /// 源字段 `symbol`。
    #[serde(rename = "symbol")]
    pub symbol: Option<String>,
    /// 源字段 `price`。
    #[serde(rename = "price")]
    pub price: Option<String>,
    /// 源字段 `qty`。
    #[serde(rename = "qty")]
    pub qty: Option<String>,
    /// 源字段 `quoteQty`。
    #[serde(rename = "quoteQty")]
    pub quote_qty: Option<String>,
    /// 源字段 `side`。
    #[serde(rename = "side")]
    pub side: Option<i64>,
    /// 源字段 `time`。
    #[serde(rename = "time")]
    pub time: Option<i64>,
}

/// 期权二十四小时行情记录。
#[doc(hidden)]
#[derive(Debug, Clone, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OptionsTickerItem {
    /// 源字段 `symbol`。
    #[serde(rename = "symbol")]
    pub symbol: Option<String>,
    /// 源字段 `priceChange`。
    #[serde(rename = "priceChange")]
    pub price_change: Option<String>,
    /// 源字段 `priceChangePercent`。
    #[serde(rename = "priceChangePercent")]
    pub price_change_percent: Option<String>,
    /// 源字段 `lastPrice`。
    #[serde(rename = "lastPrice")]
    pub last_price: Option<String>,
    /// 源字段 `lastQty`。
    #[serde(rename = "lastQty")]
    pub last_qty: Option<String>,
    /// 源字段 `open`。
    #[serde(rename = "open")]
    pub open: Option<String>,
    /// 源字段 `high`。
    #[serde(rename = "high")]
    pub high: Option<String>,
    /// 源字段 `low`。
    #[serde(rename = "low")]
    pub low: Option<String>,
    /// 源字段 `volume`。
    #[serde(rename = "volume")]
    pub volume: Option<String>,
    /// 源字段 `amount`。
    #[serde(rename = "amount")]
    pub amount: Option<String>,
    /// 源字段 `bidPrice`。
    #[serde(rename = "bidPrice")]
    pub bid_price: Option<String>,
    /// 源字段 `askPrice`。
    #[serde(rename = "askPrice")]
    pub ask_price: Option<String>,
    /// 源字段 `openTime`。
    #[serde(rename = "openTime")]
    pub open_time: Option<i64>,
    /// 源字段 `closeTime`。
    #[serde(rename = "closeTime")]
    pub close_time: Option<i64>,
    /// 源字段 `firstTradeId`。
    #[serde(rename = "firstTradeId")]
    pub first_trade_id: Option<i64>,
    /// 源字段 `tradeCount`。
    #[serde(rename = "tradeCount")]
    pub trade_count: Option<i64>,
    /// 源字段 `strikePrice`。
    #[serde(rename = "strikePrice")]
    pub strike_price: Option<String>,
    /// 源字段 `exercisePrice`。
    #[serde(rename = "exercisePrice")]
    pub exercise_price: Option<String>,
}

/// Options 冻结深度快照响应类型 OptionsBookSnapshot。
#[derive(Debug, Clone, PartialEq, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OptionsBookSnapshot {
    /// 原始响应字段 bids；每档为价格与数量二元组。
    #[serde(rename = "bids")]
    pub bids: Option<Vec<(String, String)>>,
    /// 原始响应字段 asks；每档为价格与数量二元组。
    #[serde(rename = "asks")]
    pub asks: Option<Vec<(String, String)>>,
    /// 原始响应字段 T；未提供时为 None。
    #[serde(rename = "T")]
    pub t: Option<i64>,
    /// 原始响应字段 lastUpdateId；未提供时为 None。
    #[serde(rename = "lastUpdateId")]
    pub last_update_id: Option<i64>,
}
