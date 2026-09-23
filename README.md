# binancex

Binance 公开 REST 数据的 Rust 类型层，提供 Spot、USDM、COINM、Options 四族响应值对象、离线 JSON 解析和授权证据判定。

`production_decision = NO-GO`。解析成功仅表示满足当前类型检查；库内的授权判定也不替代 Owner 签核与生产准入。

## 安装

使用 Git 源码依赖；交付时将 `rev` 固定到经 review 的提交。

```toml
[dependencies]
binancex = { git = "https://github.com/bytechainx/binancex", version = "0.1.0" }
```

当前 `rust-version = "1.77"`，依赖图推导见 [CONTEXT.md](CONTEXT.md)。

## 用法

以下为内联合成样本，用于演示离线解析，不是真实行情或源证据。

```rust
use binancex::parse::spot::parse_spot_ticker_price;
use binancex::value::spot::SpotTickerPrice;
use binancex::{current_authorization, BinanceResult, Date, Quantity, QuantityUnit, Sign};

fn main() -> BinanceResult<()> {
    let parsed = parse_spot_ticker_price(r#"{"symbol":"BTCUSDT","price":"123.4500"}"#)?;
    match parsed {
        SpotTickerPrice::Object(item) => {
            assert_eq!(item.price.as_deref(), Some("123.4500"));
        }
        SpotTickerPrice::Array(_) => unreachable!(),
    }

    let quantity = Quantity::new("-0.2500", QuantityUnit::BaseAsset)?;
    assert_eq!(quantity.value().as_str(), "-0.2500");
    assert_eq!(quantity.sign(), Sign::Negative);

    let decision = current_authorization(Date::new(2026, 9, 23)?, &[]);
    assert!(!decision.is_authorized());
    Ok(())
}
```

四族入口与白名单身份用法见 [API 文档](docs/API.md)；响应形状的开放点及当前限制见 [标准](docs/标准.md)。

## 非目标

本库不提供 HTTP 客户端、联网采集、认证或凭据读取、重试调度、缓存、存储、单位换算、跨源合并或派生指标。公开 REST 观测没有官方 vintage 面，不能据此获得正式 PIT 资格。

元仓库 `specs/binancex/` 管理端点清单、源证据、采集契约和签核；这些材料不随本 crate 打包。合成夹具不构成 live 或生产授权。

## 门禁

从 crate 根目录运行；执行前核对工作树归属。以下是应执行的检查命令，不表示当前提交已经通过。

```bash
cargo fmt --all -- --check
cargo clippy --locked --all-targets --all-features -- -D warnings
cargo test --locked --all-features
cargo package --locked --no-verify --offline
```

打包检查用于核对元数据与文件完整性。共享工作树的未提交变更导致打包拒绝时，先提交本任务变更或交由隔离检出验证；不得清理其他会话的文件来消除失败。

开发约定见 [CONTRIBUTING.md](CONTRIBUTING.md)。
