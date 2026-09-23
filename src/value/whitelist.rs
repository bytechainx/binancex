//! 白名单快照值对象。

/// 白名单快照解析结果，由族、来源端点版本、内容哈希与观测时刻共同标识。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WhitelistSnapshot {
    /// 产品族。
    pub family: String,
    /// 来源端点版本，由调用方提供。
    pub source_endpoint_version: String,
    /// 原始响应 UTF-8 字节的 SHA-256 小写十六进制摘要。
    pub content_sha256: String,
    /// 观测时刻，单位为毫秒。
    pub observed_at_ms: i64,
}

impl WhitelistSnapshot {
    /// 返回由四个身份要素组成的稳定快照标识。
    #[must_use]
    pub fn snapshot_id(&self) -> String {
        format!(
            "{}/{}@{}#{}",
            self.family, self.source_endpoint_version, self.observed_at_ms, self.content_sha256
        )
    }
}
