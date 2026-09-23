//! 离线解析（无网络参数）。
//!
//! 入口形态为「字符串/字节 → 值对象集合」；未知字段原子失败。

use crate::error::{BinanceError, BinanceErrorKind, BinanceResult};

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

/// 白名单快照解析结果（身份四要素）。
///
/// `content_sha256` 由库对 `raw` 的 UTF-8 字节计算（SHA-256，hex 小写）。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WhitelistSnapshot {
    /// 产品族
    pub family: String,
    /// 来源端点版本（调用方提供）
    pub source_endpoint_version: String,
    /// 内容哈希（库计算）
    pub content_sha256: String,
    /// 观测时刻（毫秒）
    pub observed_at_ms: i64,
}

impl WhitelistSnapshot {
    /// 快照身份（family + 版本 + 哈希 + 时刻）。
    #[must_use]
    pub fn snapshot_id(&self) -> String {
        format!(
            "{}/{}@{}#{}",
            self.family, self.source_endpoint_version, self.observed_at_ms, self.content_sha256
        )
    }
}

/// 解析 exchangeInfo 为白名单快照。
///
/// 身份四要素中 `family` / `source_endpoint_version` / `observed_at_ms` 取自观测参数，
/// `content_sha256` 由库对 `raw` 的 UTF-8 字节计算。
///
/// # Errors
///
/// - [`BinanceErrorKind::Invalid`]：`family` 为空或 `raw` 非合法 JSON
pub fn parse_exchange_info(obs: WhitelistObservation<'_>) -> BinanceResult<WhitelistSnapshot> {
    if obs.family.is_empty() {
        return Err(BinanceError::new(BinanceErrorKind::Invalid, "family 为空"));
    }
    // raw 须为合法 JSON（不深校验结构——结构合同归 response-structures.json）
    let _: serde_json::Value = serde_json::from_str(obs.raw).map_err(|e| {
        BinanceError::new(
            BinanceErrorKind::Invalid,
            format!("exchangeInfo JSON 解析失败：{e}"),
        )
    })?;
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
}
