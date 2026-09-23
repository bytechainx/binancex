//! 错误模型：`BinanceError` / `BinanceErrorKind` / `BinanceResult`。
//!
//! 形状以 `specs/binancex/contracts/source-library-contract.md` §3 为唯一权威。

use std::fmt;

/// 错误分类。`#[non_exhaustive]` 允许后续扩展。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum BinanceErrorKind {
    /// 输入形态或取值非法
    Invalid,
    /// 缺少必需项
    Missing,
    /// 重复身份 / 身份与 family 不符
    IdentityConflict,
    /// 未知字段原子失败
    UnknownField,
    /// 数值经 f64 中转；或超出所选承载类型的表示范围
    LossyNumeric,
    /// 单位/符号语义不可判定
    InvalidQuantity,
    /// referencePrice 等缺失被当 0
    MissingReference,
    /// 响应结构与契约不符
    SchemaMismatch,
    /// 授权判定未通过（fail-closed）
    AuthorizationDenied,
    /// 结构可解析但语义不被接受
    SemanticallyRejected,
    /// 尚未实现的规划能力
    NotApplicable,
    /// 不变量被破坏
    Invariant,
}

impl fmt::Display for BinanceErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Invalid => "输入形态或取值非法",
            Self::Missing => "缺少必需项",
            Self::IdentityConflict => "身份冲突",
            Self::UnknownField => "未知字段",
            Self::LossyNumeric => "数值失真",
            Self::InvalidQuantity => "数量/单位不可判定",
            Self::MissingReference => "参考值缺失",
            Self::SchemaMismatch => "结构不符",
            Self::AuthorizationDenied => "授权拒绝",
            Self::SemanticallyRejected => "语义拒绝",
            Self::NotApplicable => "尚未实现",
            Self::Invariant => "不变量破坏",
        };
        f.write_str(s)
    }
}

/// binancex 错误类型。源身份 `binance` 派生（非 crate 名派生）。
#[derive(Debug, Clone, thiserror::Error)]
#[non_exhaustive]
pub struct BinanceError {
    kind: BinanceErrorKind,
    message: String,
}

impl BinanceError {
    /// 构造错误。
    pub fn new(kind: BinanceErrorKind, message: impl Into<String>) -> Self {
        Self {
            kind,
            message: message.into(),
        }
    }

    /// 错误分类。
    #[must_use]
    pub fn kind(&self) -> BinanceErrorKind {
        self.kind
    }

    /// 本层无网络与 I/O，恒 `false`——不变量破坏亦不可重试（须修复而非重试）。
    #[must_use]
    pub fn is_retryable(&self) -> bool {
        false
    }

    /// 错误消息（不泄露凭据、不回显原始响应正文）。
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }
}

impl fmt::Display for BinanceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.kind, self.message)
    }
}

/// 结果别名。
pub type BinanceResult<T> = Result<T, BinanceError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error_display_contains_kind() {
        let e = BinanceError::new(BinanceErrorKind::UnknownField, "未知字段 foo");
        assert!(e.to_string().contains("未知字段"));
        assert_eq!(e.kind(), BinanceErrorKind::UnknownField);
        assert!(!e.is_retryable());
    }
}
