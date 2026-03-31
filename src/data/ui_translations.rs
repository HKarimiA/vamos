use crate::core::Language;
use serde::Deserialize;

/// All UI strings for the application, loaded from a translation JSON file
#[derive(Debug, Clone, Deserialize)]
pub struct UiStrings {
    pub vocabulary: String,
    pub grammar: String,
    pub settings: String,
    pub favorites: String,
    pub stage_prefix: String,
    pub language_label: String,
    pub content_coming_soon: String,
    pub topic_not_found: String,
    pub question_of: String,
    pub hint_label: String,
    pub hint_title: String,
    pub next_question: String,
    pub see_results: String,
    pub quiz_complete: String,
    pub excellent_work: String,
    pub good_job: String,
    pub not_bad: String,
    pub keep_practicing: String,
    pub try_again: String,
    pub back_to_topics: String,
    pub no_favorites: String,
    pub add_favorites_hint: String,
    pub error_loading_cards: String,
}

impl UiStrings {
    /// Format a "question X of Y" string using the translation template
    pub fn format_question_of(&self, current: usize, total: usize) -> String {
        let mut result = self.question_of.clone();
        if let Some(pos) = result.find("{}") {
            result.replace_range(pos..pos + 2, &current.to_string());
        }
        if let Some(pos) = result.find("{}") {
            result.replace_range(pos..pos + 2, &total.to_string());
        }
        result
    }
}

/// Load UI strings for the given language (embedded at compile time)
pub fn get_ui_strings(lang: Language) -> UiStrings {
    let json = match lang {
        Language::English => include_str!("../../translations/ui/en.json"),
        Language::German => include_str!("../../translations/ui/de.json"),
        Language::Spanish => include_str!("../../translations/ui/en.json"),
    };
    serde_json::from_str(json).expect("Failed to parse UI translation JSON")
}
