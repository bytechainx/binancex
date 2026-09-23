# 开发约定

1. 先读取 [AGENTS.md](AGENTS.md) 与 [CONTEXT.md](CONTEXT.md)，按变更对象定位元仓库冻结合同；字段证据不足时登记开放点，不补造源事实。
2. 在 feature branch 上实现单一变更。共享检出不切换分支；需隔离时按工作区 `docs/worktree.md` 使用既有 worktree 管理工具。只暂存本任务拥有的路径。
3. 保持四族响应模块与共享值对象边界；新增字段、单双形态或数量解释时同步说明合同依据。注释、文档及面向人的错误消息使用简体中文，文件使用 UTF-8 无 BOM 和 LF。
4. 按 [标准](docs/标准.md) 编写合成夹具与必要的正负向测试，执行 [README 门禁](README.md#门禁)，保存实际命令、退出码和失败信息。工作区缓存按 `docs/worktree.md` 约定使用。
5. 更新 `CHANGELOG.md` 和受影响的 API 文档，经 PR review 后使用 merge commit 合入。版本及依赖变化同步复核 `CONTEXT.md` 的推导。

`docs/标准.md` 登记的实现限制不能靠修改文档降为已通过；验收需同时检查代码、测试和冻结合同。授权登记中的姓名字段仅是声明，签署者身份权威由元仓库 Owner 台账维护。
