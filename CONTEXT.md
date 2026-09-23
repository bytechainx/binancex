# 维护上下文

## 交付边界

本库交付类型层。四族模块按冻结响应树表达 wire 形状；共享值对象表达身份、时间、数量与快照身份。离线解析不触发采集，也不授予生产权限，`production_decision = NO-GO`。

权威规格位于元仓库 `specs/binancex/`：修改响应类型时读取 `contracts/type-map.json`、`contracts/response-structures.json` 和 `contracts/response-structure-contract.md`；修改公共值对象或授权判定时读取 `contracts/source-library-contract.md`；修改快照身份时读取 `contracts/whitelist-contract.md`。当前行为与开放点见 [标准](docs/标准.md)。

## rust-version 推导

2026-09-23 在当前 `Cargo.lock` 上执行以下只读命令，按 `resolve.nodes` 中出现的 package ID 汇总全部依赖包的 manifest 声明，包含过程宏和目标平台条件依赖：

```bash
cargo metadata --locked --offline --format-version 1
```

下面每项的版本和 `rust-version` 均来自该命令的 `packages` 字段。`—` 表示该包未声明，不能解读为已证明兼容任意旧编译器。

| 依赖 | 锁定版本 | 声明 rust-version |
| --- | --- | --- |
| block-buffer | 0.10.4 | — |
| cfg-if | 1.0.5 | 1.32 |
| cpufeatures | 0.2.17 | — |
| crypto-common | 0.1.7 | — |
| digest | 0.10.7 | — |
| generic-array | 0.14.7 | — |
| itoa | 1.0.18 | 1.68 |
| libc | 0.2.189 | 1.65 |
| memchr | 2.8.3 | 1.61 |
| proc-macro2 | 1.0.107 | 1.71 |
| quote | 1.0.47 | 1.71 |
| serde | 1.0.229 | 1.56 |
| serde_core | 1.0.229 | 1.56 |
| serde_derive | 1.0.229 | 1.71 |
| serde_json | 1.0.151 | 1.71 |
| sha2 | 0.10.9 | — |
| syn | 3.0.6 | 1.71 |
| thiserror | 2.0.21 | 1.77 |
| thiserror-impl | 2.0.21 | 1.77 |
| typenum | 1.20.1 | 1.41.0 |
| unicode-ident | 1.0.26 | 1.71 |
| version_check | 0.9.5 | — |
| zmij | 1.0.23 | 1.71 |

已声明值的最大值为 **1.77**，来自 `thiserror` 和 `thiserror-impl`，故本 crate 的 `rust-version` 设为 `1.77`。这是一项依赖声明推导，不代表已经用 Rust 1.77 完成构建验证。依赖锁定版本变化后须重新推导，不能沿用本表结论。

## 依赖用途

`serde` / `serde_json` 用于离线解析，`serde_json` 的 `arbitrary_precision` 保留 JSON number 的无损表示；`thiserror` 用于错误模型；`sha2` 只用于原始输入字节的内容摘要。依赖边界不包含网络客户端、异步运行时或其他工作区 crate。
