# API

## 模块与入口

| 模块 | 职责与示例入口 |
| --- | --- |
| `value::spot` / `parse::spot` | Spot 响应；`parse_spot_exchange_info`、`parse_spot_trade`、`parse_spot_kline`、`parse_spot_ticker_price` |
| `value::usdm` / `parse::usdm` | USDM 响应；`parse_usdm_exchange_info`、`parse_usdm_kline`、`parse_usdm_funding_rate`、`parse_usdm_trading_schedule` |
| `value::coinm` / `parse::coinm` | COINM 响应；`parse_coinm_exchange_info`、`parse_coinm_kline`、`parse_coinm_open_interest` |
| `value::options` / `parse::options` | Options 响应；`parse_options_exchange_info`、`parse_options_mark`、`parse_options_kline` |
| `value` / `authz` / `error` | 共享值对象、离线证据判定与 `BinanceResult<T>` |

四族 `parse_*` 接受 `&str`，返回 `BinanceResult<响应类型>`；不接受 URL、网络客户端或凭据。函数名中的单数不表示单行响应，例如 `SpotTrade` 是行数组，`SpotKline` 是定长元组数组。对象或数组双形响应使用 `Object` / `Array` 枚举变体，调用方须匹配实际形态。

双形响应的具体 `parse_*` 入口先按 JSON 根形态选择对象或数组，再对所选类型执行严格解析。直接对导出类型调用 `serde_json::from_str` 不等同于调用这些入口，可能绕过入口中的结构和业务检查。

wire 字段使用源 JSON 键，Rust 字段使用 snake_case。当前对象字段按冻结树的可缺性保留为 `Option<T>`；缺项不是零值。wire string 保留为 `String`，不自动转为 `Decimal`；wire number 采用 `serde_json::Number` 保留表示。各类原始时间字段保持源单位，不自动跨族转换。解析器的结构与语义限制见 [标准](标准.md)。

`parse_spot_avg_price` 要求关键字段 `price` 与 `closeTime` 存在：缺键返回 `Missing`，显式 `null` 或类型错误返回 `Invalid`；`closeTime` 使用小数或指数形式也在此预检拒绝。`referencePrice` 继续允许缺失或 `null`。其他类型组的命名必填字段集合尚未统一裁决。

## 数值、时间与身份

`Decimal::new(&str)` 校验十进制文本并保留原文，接受可选正负号、小数点及科学计数指数；有效数字与出现的指数都必须含数字，拒绝空串、孤立符号和重复小数点。`as_str()` 返回原始表示，`sign()` 返回数值符号。`Quantity::new(&str, QuantityUnit)` 返回 `BinanceResult<Quantity>`，从数值派生只读符号；单位为 `BaseAsset`、`QuoteAsset` 或 `Contracts`。`-0` 的文本得以保留，符号为 `Sign::Zero`。调用方须先依据端点合同确定单位。

`Date::new(year, month, day)` 构造合法日历日期；`TimePrecision` 区分 `Ms`、`S`、`Us`。`Interval` 保留周期字符串。`EndpointId` 与 `DataSeriesId` 表达端点及序列身份，标的与主题枚举位于 `value::identity`。本库的 `PitEligibility` 只有 `NotEligible`。

## 白名单快照

先以族解析器检查响应形状，再用同一份原始字节构造快照。以下是合成的最小结构示例，不是获准采集记录。

```rust
use binancex::parse::{parse_exchange_info, WhitelistObservation};
use binancex::parse::spot::parse_spot_exchange_info;
use binancex::BinanceResult;

fn main() -> BinanceResult<()> {
    let raw = r#"{"timezone":"UTC","symbols":[]}"#;
    let _response = parse_spot_exchange_info(raw)?;
    let snapshot = parse_exchange_info(WhitelistObservation {
        family: "spot",
        raw,
        source_endpoint_version: "/api/v3",
        observed_at_ms: 1_790_121_600_000,
    })?;
    assert_eq!(snapshot.content_sha256.len(), 64);
    assert!(snapshot.snapshot_id().contains("spot/"));
    Ok(())
}
```

快照身份由 `family`、`source_endpoint_version`、`observed_at_ms` 和 `raw` 的 SHA-256 构成。哈希覆盖原始 UTF-8 字节，空白变化也会改变内容摘要；端点版本与观测时间由调用方提供。

通用 `parse_exchange_info` 当前只检查 family 非空和 JSON 语法，再计算摘要；它本身不验证族枚举、端点版本、深层响应结构、标的资格或授权。`WhitelistSnapshot` 字段可公开构造，因此持有快照本身也不是通过验证的凭证。

## 授权判定

`registered_evidence(raw)` 接受 `evidence_id`、`scope`、`signed_by`、`valid_from`、`valid_until` 五个字符串字段；日期格式为 `YYYY-MM-DD`。内容哈希由库计算，输入携带 `content_sha256` 或其他未定义字段会失败。

`current_authorization(as_of, evidence)` 是纯离线函数：结构非法、空证据、没有覆盖 `as_of` 的有效证据、有效证据范围冲突时返回 `Denied`。有效期两端均包含；存在时序有效且范围一致的证据时返回 `Authorized { scope }`。

`Authorized` 只表达传入证据的结构、时序和范围一致性。库不验证签名者身份，不联系 Owner 台账，不核实 scope 是否覆盖某次实际请求；这些判断由调用方及审批流程承担。crate 的 `production_decision = NO-GO` 不因函数返回值自动改变。

## 错误

`BinanceError::kind()` 提供分类，`message()` 提供原因；纯离线库的 `is_retryable()` 恒为 `false`。错误分类与授权结论均为 `#[non_exhaustive]`，匹配时保留通配分支。

具体解析入口将对象未知字段归为 `UnknownField`，结构与字段类型不符归为 `SchemaMismatch`，整数越界或整数位置使用浮点表示归为 `LossyNumeric`，非法 JSON 归为 `Invalid`。对象必须使用 JSON 映射；有已登记字段的对象拒绝空对象，元组长度也必须匹配。双形响应通过根形态分支保留具体对象或数组的错误信息。

`parse_usdm_delivery_price` 额外拒绝 JSON number 的对象伪装，返回 `SchemaMismatch`；合法 JSON 数字仍以无损表示保留。

12 组已有证据支持的 Kline 入口按开盘时间拒绝批次内重复行，返回 `IdentityConflict`，不返回部分结果：Spot Kline、USDM 五组 Kline、COINM 五组 Kline、Options Kline。Spot UiKline 及其他对象批次的身份约束尚未完整裁决。
