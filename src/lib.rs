//! binancex —— Binance 公开 REST 数据的类型层与采集契约层。
//!
//! 本 crate 是纯类型层：值对象、离线解析、fail-closed 授权判定。
//! 不做联网采集、认证、缓存、存储、单位换算或派生指标。
//! `production_decision = NO-GO`：解析成功及库内授权判定均不构成 live 采集或生产接入许可。
//!
//! ## 模块
//!
//! - [`error`] —— 错误模型（`BinanceError` / `BinanceErrorKind` / `BinanceResult`）
//! - [`value`] —— 共享内核与四族值对象门面
//! - [`authz`] —— fail-closed 授权判定（无签核恒 `Denied`）
//! - [`parse`] —— 离线解析（无网络参数）
//! - [`value::spot`] / [`value::usdm`] / [`value::coinm`] / [`value::options`] —— 四族值对象
//!
//! ## 非目标
//!
//! 本 crate 不提供 HTTP 客户端、不读凭据、不落存储。
//! 采集契约归元仓库规格包（`specs/binancex/`），不随 crate 打包。
//! production_decision = "NO-GO"；授权状态为 `unknown`，无 Owner 签核时授权判定保持拒绝。

#![forbid(unsafe_code)]
#![deny(missing_docs)]
#![deny(unreachable_pub)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![deny(clippy::todo)]
#![deny(clippy::unimplemented)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used, clippy::panic))]

pub mod authz;
pub mod error;
pub mod parse;
pub mod value;

pub use authz::{
    current_authorization, registered_evidence, AuthorizationEvidence, BinanceAuthorization,
};
pub use error::{BinanceError, BinanceErrorKind, BinanceResult};
pub use value::{
    DataSeriesEntity, DataSeriesId, Date, Decimal, EndpointId, Instrument, Interval,
    PitEligibility, Quantity, QuantityUnit, Sign, Subject, TimePrecision, WhitelistSnapshot,
};
