# UI Module Specification

## MODIFIED Requirements

### Requirement: Locale-Aware Text Rendering

All `draw_*` functions SHALL accept a `Locale` parameter and render text using locale-specific strings.

#### Scenario: Menu screen in English
- **WHEN** `draw_menu` is called with `Locale::En`
- **THEN** displays "Welcome to Rust Trivia!" and "Press ENTER to start playing"

#### Scenario: Menu screen in Chinese
- **WHEN** `draw_menu` is called with `Locale::Zh`
- **THEN** displays "欢迎来到 Rust 答题游戏！" and "按回车键开始游戏"

### Requirement: Progress Indicator Localization

The progress indicator SHALL display locale-specific labels.

#### Scenario: Question counter in English
- **WHEN** locale is `En` and displaying question 5 of 10
- **THEN** displays "Question 5/10 | Score: 3/4"

#### Scenario: Question counter in Chinese
- **WHEN** locale is `Zh` and displaying question 5 of 10
- **THEN** displays "问题 5/10 | 得分 3/4"

### Requirement: Result Screen Localization

The correct/incorrect result messages SHALL be locale-aware.

#### Scenario: Correct answer in English
- **WHEN** locale is `En` and answer is correct
- **THEN** displays "✅ Correct!"

#### Scenario: Correct answer in Chinese
- **WHEN** locale is `Zh` and answer is correct
- **THEN** displays "✅ 正确！"

### Requirement: Performance Message Localization

Game over performance messages SHALL be locale-aware.

#### Scenario: Excellent performance in Chinese
- **WHEN** locale is `Zh` and score is 95%
- **THEN** displays "🏆 太棒了！你是答题大师！"

#### Scenario: Great performance in Chinese
- **WHEN** locale is `Zh` and score is 85%
- **THEN** displays "🌟 太棒了！非常出色！"

### Requirement: Footer Help Text Localization

Footer text SHALL reflect the current game state with locale-appropriate wording.

#### Scenario: Menu footer in Chinese
- **WHEN** locale is `Zh` and state is `Menu`
- **THEN** footer displays "按回车键开始 • 按 'q' 退出"

#### Scenario: Question footer in Chinese
- **WHEN** locale is `Zh` and state is `Question`
- **THEN** footer displays "按 1-4 选择答案 • 按 'q' 退出"
