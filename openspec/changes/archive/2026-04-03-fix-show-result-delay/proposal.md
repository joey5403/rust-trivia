## Why

用户在答题后感到明显的卡顿感。原因是 `ShowResult` 状态后有一个 2 秒的 `sleep()` 阻塞，期间完全不响应用户输入。这降低了用户体验的流畅度。

## What Changes

- 移除 `ShowResult` 状态后的自动 2 秒等待
- 在结果页面期间（`ShowResult` 状态），所有键盘输入直接忽略，不进入下一题
- 用户按任意键（1-4 或 Enter）立即进入下一题，无需等待

## Capabilities

这是一个 bug 修复，修改现有行为，不涉及新 capabilities。

### New Capabilities
（无）

### Modified Capabilities
（无——现有行为修改不属于 capability 层面）

## Impact

- **src/main.rs**: 移除第 95-99 行的 `sleep(2秒)` 逻辑
- 用户体验：答题后立即显示下一题，无延迟
