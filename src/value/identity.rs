//! 身份：`EndpointId` / `DataSeriesId` / `Instrument` / `Subject`。

use std::fmt;

/// 端点身份（来源关系 / provenance）。
///
/// `EndpointId = provider + catalog_api_family + method + path + version`。
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EndpointId {
    provider: String,
    catalog_api_family: String,
    method: String,
    path: String,
    version: String,
}

impl EndpointId {
    /// 构造。
    #[must_use]
    pub fn new(
        provider: impl Into<String>,
        catalog_api_family: impl Into<String>,
        method: impl Into<String>,
        path: impl Into<String>,
        version: impl Into<String>,
    ) -> Self {
        Self {
            provider: provider.into(),
            catalog_api_family: catalog_api_family.into(),
            method: method.into(),
            path: path.into(),
            version: version.into(),
        }
    }

    /// 目录归属族。
    #[must_use]
    pub fn family(&self) -> &str {
        &self.catalog_api_family
    }

    /// 请求方法。
    #[must_use]
    pub fn method(&self) -> &str {
        &self.method
    }

    /// 路径。
    #[must_use]
    pub fn path(&self) -> &str {
        &self.path
    }

    /// 版本（由 path 前缀隐含，如 `/api/v3`、`/fapi/v1`）。
    #[must_use]
    pub fn version(&self) -> &str {
        &self.version
    }
}

impl fmt::Display for EndpointId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}/{} {} {}",
            self.provider, self.catalog_api_family, self.method, self.path
        )
    }
}

/// 数据系列身份（业务身份，不含 `EndpointId`）。
///
/// `DataSeriesId = actual_market_family + Instrument/Subject + 原生变体 + 语义维度`。
/// 同一业务事实经两条已证明等价路由时，业务身份唯一，来源关系保留两份。
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DataSeriesId {
    actual_market_family: String,
    entity: DataSeriesEntity,
    variant: String,
    semantic_dimension: String,
}

impl DataSeriesId {
    /// 构造。
    #[must_use]
    pub fn new(
        actual_market_family: impl Into<String>,
        entity: impl Into<DataSeriesEntity>,
        variant: impl Into<String>,
        semantic_dimension: impl Into<String>,
    ) -> Self {
        Self {
            actual_market_family: actual_market_family.into(),
            entity: entity.into(),
            variant: variant.into(),
            semantic_dimension: semantic_dimension.into(),
        }
    }

    /// 实际市场（由标的身份判定，不由 URL 前缀推导）。
    #[must_use]
    pub fn market(&self) -> &str {
        &self.actual_market_family
    }

    /// 标的。
    #[must_use]
    pub fn entity(&self) -> &DataSeriesEntity {
        &self.entity
    }

    /// 原生离散变体。
    #[must_use]
    pub fn variant(&self) -> &str {
        &self.variant
    }

    /// 序列语义维度；例如区分 Spot klines 与 uiKlines。
    #[must_use]
    pub fn semantic_dimension(&self) -> &str {
        &self.semantic_dimension
    }
}

/// 序列业务身份主体。
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DataSeriesEntity {
    /// 交易标的。
    Instrument(Instrument),
    /// 合约对。
    Pair(String),
    /// 期权或指数所用 underlying。
    Underlying(String),
    /// 到期日。
    Expiry(String),
    /// 无 symbol 的公开全局主体。
    Subject(Subject),
}

impl From<Instrument> for DataSeriesEntity {
    fn from(value: Instrument) -> Self {
        Self::Instrument(value)
    }
}

impl From<Subject> for DataSeriesEntity {
    fn from(value: Subject) -> Self {
        Self::Subject(value)
    }
}

/// 标的。
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Instrument {
    /// Spot/UM/CM 的 symbol（含合约类型、到期日、结算资产）
    Symbol(String),
    /// Options 的 underlyingAsset + expiry + strike + side
    Option {
        /// 标的资产
        underlying_asset: String,
        /// 到期日
        expiry: String,
        /// 行权价
        strike: String,
        /// 方向（C/P）
        side: String,
    },
}

/// 公开全局主体（无 symbol 的公开对象）。
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Subject {
    /// 标的宇宙（instrument_universe）
    InstrumentUniverse,
    /// 指数成分
    Constituents,
    /// 资产指数
    AssetIndex,
    /// 保险基金
    InsuranceFund,
    /// 交易时间表
    TradingSchedule,
    /// 其他（自定义标识）
    Other(String),
}

impl fmt::Display for Subject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InstrumentUniverse => f.write_str("InstrumentUniverse"),
            Self::Constituents => f.write_str("Constituents"),
            Self::AssetIndex => f.write_str("AssetIndex"),
            Self::InsuranceFund => f.write_str("InsuranceFund"),
            Self::TradingSchedule => f.write_str("TradingSchedule"),
            Self::Other(s) => write!(f, "Subject({s})"),
        }
    }
}
