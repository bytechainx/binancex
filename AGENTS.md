# binancex 执行约定

本目录是类型层 crate，生产决定固定为 `NO-GO`；执行范围由当前任务授权确定。

- 修改响应类型或解析器前，读取元仓库 `specs/binancex/contracts/{type-map,response-structures}.json` 与 `response-structure-contract.md`；以字段路径和 wire 类型核对变更，保留开放点。
- 修改共享值对象、错误模型或授权判定前，读取同目录 `source-library-contract.md`；修改白名单身份另读 `whitelist-contract.md`。合同缺证时先报告差异。
- 开发、隔离检出、提交和门禁遵循 [CONTRIBUTING.md](CONTRIBUTING.md)；依赖 MSRV 及文档入口见 [CONTEXT.md](CONTEXT.md)。只写当前任务允许的文件，保留其他会话改动。
- 合成夹具与当前解析限制按 [标准](docs/标准.md) 处理。实现通过编译不等于合同验收通过；结果须附实际命令、退出码与未通过项。
