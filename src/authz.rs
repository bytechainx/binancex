//! fail-closed 授权判定。
//!
//! 形状以 `specs/binancex/contracts/source-library-contract.md` §4 为唯一权威。

use crate::error::{BinanceError, BinanceErrorKind, BinanceResult};
use crate::value::{validate_sha256_hex, Date};
use sha2::{Digest, Sha256};
use std::fmt;

/// 授权结论（只读，不改变授权状态）。
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum BinanceAuthorization {
    /// 授权通过
    Authorized {
        /// 覆盖范围
        scope: String,
    },
    /// 拒绝（携带可读原因）
    Denied {
        /// 拒绝原因
        reason: String,
    },
}

impl BinanceAuthorization {
    /// 便捷判定：`Authorized` → `true`，`Denied` → `false`。
    #[must_use]
    pub fn is_authorized(&self) -> bool {
        matches!(self, Self::Authorized { .. })
    }
}

impl fmt::Display for BinanceAuthorization {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Authorized { scope } => write!(f, "Authorized({scope})"),
            Self::Denied { reason } => write!(f, "Denied({reason})"),
        }
    }
}

/// 授权证据值对象：对 Owner 签核工件的离线解析结果。
///
/// 库只解析与判定，不产生、不签署、不验证身份——
/// 签署者身份权威归元仓库 Owner 台账。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthorizationEvidence {
    evidence_id: String,
    scope: String,
    signed_by: String,
    valid_from: Date,
    valid_until: Date,
    content_sha256: String,
}

impl AuthorizationEvidence {
    /// 证据标识。
    #[must_use]
    pub fn evidence_id(&self) -> &str {
        &self.evidence_id
    }

    /// 覆盖范围声明。
    #[must_use]
    pub fn scope(&self) -> &str {
        &self.scope
    }

    /// 签署者标识（身份验证不归本库）。
    #[must_use]
    pub fn signed_by(&self) -> &str {
        &self.signed_by
    }

    /// 生效日（含）。
    #[must_use]
    pub fn valid_from(&self) -> &Date {
        &self.valid_from
    }

    /// 失效日（含）。
    #[must_use]
    pub fn valid_until(&self) -> &Date {
        &self.valid_until
    }

    /// 证据工件原始 UTF-8 字节的 SHA-256（hex 小写，库计算）。
    #[must_use]
    pub fn content_sha256(&self) -> &str {
        &self.content_sha256
    }

    fn is_structurally_valid(&self) -> bool {
        !self.evidence_id.is_empty()
            && !self.scope.is_empty()
            && !self.signed_by.is_empty()
            && self.valid_from <= self.valid_until
            && validate_sha256_hex(&self.content_sha256).is_ok()
    }
}

/// 解析证据工件（离线 JSON）。
///
/// 输入允许键 = `{evidence_id, scope, signed_by, valid_from, valid_until}` 五项；
/// `content_sha256` 由库计算输出，输入含该键 → [`BinanceErrorKind::UnknownField`] 拒绝。
/// `valid_from` / `valid_until` 为 `"YYYY-MM-DD"` 字符串。
///
/// # Errors
///
/// - [`BinanceErrorKind::UnknownField`]：输入含 `content_sha256` 或其他未定义键
/// - [`BinanceErrorKind::Missing`]：缺少必需键
/// - [`BinanceErrorKind::Invalid`]：空串或日期格式非法
pub fn registered_evidence(raw: &str) -> BinanceResult<AuthorizationEvidence> {
    let v: serde_json::Value = serde_json::from_str(raw).map_err(|e| {
        BinanceError::new(
            BinanceErrorKind::Invalid,
            format!("证据工件 JSON 解析失败：{e}"),
        )
    })?;

    let obj = v
        .as_object()
        .ok_or_else(|| BinanceError::new(BinanceErrorKind::Invalid, "证据工件须为 JSON 对象"))?;

    // 未知字段原子失败（含 content_sha256——防调用方自带哈希冒充登记）
    for key in obj.keys() {
        if ![
            "evidence_id",
            "scope",
            "signed_by",
            "valid_from",
            "valid_until",
        ]
        .contains(&key.as_str())
        {
            return Err(BinanceError::new(
                BinanceErrorKind::UnknownField,
                format!("证据工件含未定义键 `{key}`（content_sha256 由库计算，不接受输入）"),
            ));
        }
    }

    let get_str = |key: &str| -> BinanceResult<String> {
        let val = obj.get(key).ok_or_else(|| {
            BinanceError::new(BinanceErrorKind::Missing, format!("证据工件缺 `{key}`"))
        })?;
        let s = val.as_str().ok_or_else(|| {
            BinanceError::new(
                BinanceErrorKind::Invalid,
                format!("证据工件 `{key}` 须为字符串"),
            )
        })?;
        if s.is_empty() {
            return Err(BinanceError::new(
                BinanceErrorKind::Invalid,
                format!("证据工件 `{key}` 为空串"),
            ));
        }
        Ok(s.to_owned())
    };

    let evidence_id = get_str("evidence_id")?;
    let scope = get_str("scope")?;
    let signed_by = get_str("signed_by")?;
    let valid_from_str = get_str("valid_from")?;
    let valid_until_str = get_str("valid_until")?;

    let parse_date = |s: &str| -> BinanceResult<Date> {
        let parts: Vec<&str> = s.split('-').collect();
        if parts.len() != 3 {
            return Err(BinanceError::new(
                BinanceErrorKind::Invalid,
                format!("日期 `{s}` 须为 YYYY-MM-DD"),
            ));
        }
        let year: i32 = parts[0]
            .parse()
            .map_err(|_| BinanceError::new(BinanceErrorKind::Invalid, "日期年份非法"))?;
        let month: u8 = parts[1]
            .parse()
            .map_err(|_| BinanceError::new(BinanceErrorKind::Invalid, "日期月份非法"))?;
        let day: u8 = parts[2]
            .parse()
            .map_err(|_| BinanceError::new(BinanceErrorKind::Invalid, "日期天数非法"))?;
        Date::new(year, month, day)
    };

    let valid_from = parse_date(&valid_from_str)?;
    let valid_until = parse_date(&valid_until_str)?;

    if valid_from > valid_until {
        return Err(BinanceError::new(
            BinanceErrorKind::Invalid,
            "valid_from > valid_until",
        ));
    }

    // content_sha256 由库对原始 UTF-8 字节计算
    let sha = Sha256::digest(raw.as_bytes());
    let content_sha256 = format!("{sha:x}");

    Ok(AuthorizationEvidence {
        evidence_id,
        scope,
        signed_by,
        valid_from,
        valid_until,
        content_sha256,
    })
}

/// 证据感知的授权判定（fail-closed，形状 v2）。
///
/// 判定规则（按序，`Denied` 均携带对应可读 `reason`）：
///
/// | 情形 | 结论 |
/// | --- | --- |
/// | 存在结构非法证据 | `Denied`（证据结构非法） |
/// | `evidence` 为空 | `Denied`（无登记证据） |
/// | 无任一证据满足 `valid_from ≤ as_of ≤ valid_until` | `Denied`（证据过期或未生效） |
/// | 多条时序有效证据的 `scope` 不一致 | `Denied`（证据范围冲突） |
/// | 存在时序有效且 `scope` 一致的证据 | `Authorized { scope }` |
pub fn current_authorization(
    as_of: Date,
    evidence: &[AuthorizationEvidence],
) -> BinanceAuthorization {
    // 1. 结构复验（防绕过登记直接构造）
    if evidence.iter().any(|e| !e.is_structurally_valid()) {
        return BinanceAuthorization::Denied {
            reason: "证据结构非法".to_owned(),
        };
    }
    // 2. 空证据
    if evidence.is_empty() {
        return BinanceAuthorization::Denied {
            reason: "无登记证据".to_owned(),
        };
    }
    // 3. 时序过滤
    let temporally_valid: Vec<&AuthorizationEvidence> = evidence
        .iter()
        .filter(|e| e.valid_from <= as_of && as_of <= e.valid_until)
        .collect();
    if temporally_valid.is_empty() {
        return BinanceAuthorization::Denied {
            reason: "证据过期或未生效".to_owned(),
        };
    }
    // 4. 范围冲突（全集判定，不挑子集，与顺序无关）
    let scopes: std::collections::HashSet<&str> =
        temporally_valid.iter().map(|e| e.scope.as_str()).collect();
    if scopes.len() > 1 {
        return BinanceAuthorization::Denied {
            reason: "证据范围冲突".to_owned(),
        };
    }
    // 5. Authorized
    let scope = temporally_valid[0].scope.clone();
    BinanceAuthorization::Authorized { scope }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_raw(id: &str, scope: &str, from: &str, until: &str) -> String {
        format!(
            r#"{{"evidence_id":"{id}","scope":"{scope}","signed_by":"owner@example","valid_from":"{from}","valid_until":"{until}"}}"#
        )
    }

    #[test]
    fn empty_evidence_always_denied() {
        let as_of = Date::new(2026, 9, 23).unwrap();
        let verdict = current_authorization(as_of, &[]);
        assert!(!verdict.is_authorized());
    }

    #[test]
    fn valid_evidence_authorized() {
        let as_of = Date::new(2026, 9, 23).unwrap();
        let ev =
            registered_evidence(&make_raw("E1", "spot:read", "2026-01-01", "2026-12-31")).unwrap();
        let verdict = current_authorization(as_of, &[ev]);
        assert!(verdict.is_authorized());
    }

    #[test]
    fn expired_evidence_denied() {
        let as_of = Date::new(2026, 9, 23).unwrap();
        let ev =
            registered_evidence(&make_raw("E1", "spot:read", "2025-01-01", "2025-12-31")).unwrap();
        let verdict = current_authorization(as_of, &[ev]);
        assert!(!verdict.is_authorized());
    }

    #[test]
    fn scope_conflict_denied() {
        let as_of = Date::new(2026, 9, 23).unwrap();
        let a =
            registered_evidence(&make_raw("E1", "spot:read", "2026-01-01", "2026-12-31")).unwrap();
        let b =
            registered_evidence(&make_raw("E2", "usdm:read", "2026-01-01", "2026-12-31")).unwrap();
        let verdict = current_authorization(as_of, &[a, b]);
        assert!(!verdict.is_authorized());
    }

    #[test]
    fn same_scope_authorized() {
        let as_of = Date::new(2026, 9, 23).unwrap();
        let a =
            registered_evidence(&make_raw("E1", "spot:read", "2026-01-01", "2026-12-31")).unwrap();
        let b =
            registered_evidence(&make_raw("E2", "spot:read", "2026-06-01", "2026-12-31")).unwrap();
        let verdict = current_authorization(as_of, &[a, b]);
        assert!(verdict.is_authorized());
    }

    #[test]
    fn registered_evidence_rejects_content_sha256_input() {
        let raw = r#"{"evidence_id":"E1","scope":"s","signed_by":"x","valid_from":"2026-01-01","valid_until":"2026-12-31","content_sha256":"abc"}"#;
        let result = registered_evidence(raw);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind(), BinanceErrorKind::UnknownField);
    }

    #[test]
    fn registered_evidence_computes_sha256() {
        let ev = registered_evidence(&make_raw("E1", "s", "2026-01-01", "2026-12-31")).unwrap();
        assert_eq!(ev.content_sha256().len(), 64);
    }
}
