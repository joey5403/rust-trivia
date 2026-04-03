## Context

当前游戏循环在 `ShowResult` 状态时，执行 `sleep(Duration::from_secs(2)).await` 阻塞 2 秒。这期间 UI 仍正常渲染（显示结果），但键盘事件完全不响应。用户必须等待 2 秒后才能进入下一题。

## Goals / Non-Goals

**Goals:**
- 消除答题后的卡顿感
- 用户按任意键立即进入下一题

**Non-Goals:**
- 不改变 UI 显示内容
- 不改变其他状态转换逻辑

## Decisions

### Decision 1: 移除 sleep 逻辑

**Choice:** 删除 `main.rs` 第 95-99 行的 `sleep(2秒)` 逻辑。

**Rationale:** 这是造成卡顿的唯一原因。移除后，用户按任意键直接进入下一题。

### Decision 2: 忽略 ShowResult 期间的键盘输入

**Choice:** 在 `ShowResult` 状态下，1-4 和 Enter 键不触发任何操作。

**Rationale:** 用户按 1-4 答题后进入 `ShowResult`。如果继续响应 1-4，可能导致意外的状态跳转。Enter 也应该忽略，因为下一题的进入应该由答题动作本身触发，而不是额外的按键。

## Risks / Trade-offs

| 风险 | 说明 | 缓解 |
|------|------|------|
| 用户看不清结果 | 移除等待后，结果页面一闪而过 | 用户可再次按 1-4 查看同一题（代码未限制） |
| 状态跳转逻辑复杂化 | 需要在 key handler 中增加 ShowResult 的判断 | 仅需在现有 match 中增加一个 no-op branch |
