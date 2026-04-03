#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Locale {
    En,
    Zh,
}

pub struct LocaleStrings {
    // Menu
    pub welcome_title: &'static str,
    pub welcome_subtitle: &'static str,
    pub menu_instruction: &'static str,

    // Category selection
    pub all_categories: &'static str,
    pub select_category_title: &'static str,

    // Loading
    pub loading_title: &'static str,
    pub loading_message: &'static str,

    // Translating
    pub translating_title: &'static str,
    pub translating_format: &'static str,

    // Question
    pub answers_title: &'static str,
    pub progress_title: &'static str,

    // Game over
    pub game_over_title: &'static str,
    pub final_score: &'static str,
    pub percentage: &'static str,

    // Footer
    pub footer_menu: &'static str,
    pub footer_select_category: &'static str,
    pub footer_question: &'static str,
    pub footer_game_over: &'static str,
    pub footer_quit: &'static str,

    // Performance messages
    pub perf_excellent: &'static str,
    pub perf_great: &'static str,
    pub perf_good: &'static str,
    pub perf_not_bad: &'static str,
    pub perf_better_luck: &'static str,

    // Header
    pub header_title: &'static str,
}

impl LocaleStrings {
    pub fn get(locale: Locale) -> &'static LocaleStrings {
        match locale {
            Locale::En => &ENGLISH,
            Locale::Zh => &CHINESE,
        }
    }
}

static ENGLISH: LocaleStrings = LocaleStrings {
    welcome_title: "Welcome to Rust Trivia!",
    welcome_subtitle: "Test your knowledge with questions from OpenTDB",
    menu_instruction: "Press ENTER to select a category",

    all_categories: "All Categories",
    select_category_title: "Select Category",

    loading_title: "Loading",
    loading_message: "Please wait while we fetch trivia questions",

    translating_title: "Translating Questions",
    translating_format: "Translating Questions {}/{}",

    answers_title: "Answers",
    progress_title: "Progress",

    game_over_title: "Game Over! 🎉",
    final_score: "Final Score:",
    percentage: "Percentage:",

    footer_menu: "Press ENTER to start • Press 'q' to quit",
    footer_select_category: "↑/↓ to navigate • ENTER to confirm • 'q' to quit",
    footer_question: "Press 1-4 to select answer • Press 'q' to quit",
    footer_game_over: "Press ENTER to play again • Press 'q' to quit",
    footer_quit: "Press 'q' to quit",

    perf_excellent: "🏆 Excellent! You're a trivia master!",
    perf_great: "🌟 Great job! Very impressive!",
    perf_good: "👍 Good work! Keep it up!",
    perf_not_bad: "😊 Not bad! Room for improvement!",
    perf_better_luck: "😅 Better luck next time!",

    header_title: "🧠 Rust Trivia Game 🧠",
};

static CHINESE: LocaleStrings = LocaleStrings {
    welcome_title: "欢迎来到 Rust 答题游戏！",
    welcome_subtitle: "用 OpenTDB 的问题测试你的知识",
    menu_instruction: "按回车键选择一个类别",

    all_categories: "所有类别",
    select_category_title: "选择类别",

    loading_title: "加载中",
    loading_message: "正在获取答题题目，请稍候",

    translating_title: "正在翻译题目",
    translating_format: "正在翻译题目 {}/{}",

    answers_title: "答案",
    progress_title: "进度",

    game_over_title: "游戏结束！🎉",
    final_score: "最终得分：",
    percentage: "正确率：",

    footer_menu: "按回车键开始 • 按 'q' 退出",
    footer_select_category: "↑/↓ 浏览 • 回车确认 • 'q' 退出",
    footer_question: "按 1-4 选择答案 • 按 'q' 退出",
    footer_game_over: "按回车键再玩一次 • 按 'q' 退出",
    footer_quit: "按 'q' 退出",

    perf_excellent: "🏆 太棒了！你是答题大师！",
    perf_great: "🌟 太棒了！非常出色！",
    perf_good: "👍 做得好！继续加油！",
    perf_not_bad: "😊 不错！还有进步空间！",
    perf_better_luck: "😅 下次好运！",

    header_title: "🧠 Rust 答题游戏 🧠",
};
