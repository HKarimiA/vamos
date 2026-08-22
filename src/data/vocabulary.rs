use serde::{Deserialize, Serialize};

/// Represents a single vocabulary card with translations
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VocabularyCard {
    pub id: u32,
    pub word: String,
    pub example: String,
}

/// Language direction for learning
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LearningDirection {
    SpanishToEnglish,
    EnglishToSpanish,
}

/// Macro to dynamically generate match arms for vocabulary stage loading
macro_rules! include_vocabulary_stages {
    ($stage:expr, $language:expr, [$($num:literal),* $(,)?]) => {
        match ($stage, $language) {
            $(
                ($num, "es") => include_str!(concat!("../../translations/vocabulary/", stringify!($num), "/es.json")),
                ($num, "en") => include_str!(concat!("../../translations/vocabulary/", stringify!($num), "/en.json")),
                ($num, "de") => include_str!(concat!("../../translations/vocabulary/", stringify!($num), "/de.json")),
            )*
            _ => {
                return Err(format!(
                    "Stage {} for language {} not found",
                    $stage, $language
                ));
            }
        }
    };
}

/// Load vocabulary cards for a specific stage and language
pub fn load_vocabulary_stage(stage: u32, language: &str) -> Result<Vec<VocabularyCard>, String> {
    let json_data = include_vocabulary_stages!(
        stage,
        language,
        [
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
            24, 25, 26, 27, 28, 29, 30, 31, 32, 33
        ]
    );

    serde_json::from_str(json_data)
        .map_err(|e| format!("Failed to parse JSON for stage {}: {}", stage, e))
}

/// Get a pair of cards (source and target language) for a specific stage and card index.
/// `ui_lang` selects the non-Spanish side: "en" for English, "de" for German.
pub fn get_card_pair(
    stage: u32,
    card_index: usize,
    direction: LearningDirection,
    ui_lang: &str,
) -> Result<(VocabularyCard, VocabularyCard), String> {
    let spanish_cards = load_vocabulary_stage(stage, "es")?;
    let native_cards = load_vocabulary_stage(stage, ui_lang)?;

    if card_index >= spanish_cards.len() || card_index >= native_cards.len() {
        return Err("Card index out of bounds".to_string());
    }

    match direction {
        LearningDirection::SpanishToEnglish => Ok((
            spanish_cards[card_index].clone(),
            native_cards[card_index].clone(),
        )),
        LearningDirection::EnglishToSpanish => Ok((
            native_cards[card_index].clone(),
            spanish_cards[card_index].clone(),
        )),
    }
}

/// Get total number of cards in a stage
pub fn get_stage_card_count(stage: u32) -> Result<usize, String> {
    let cards = load_vocabulary_stage(stage, "es")?;
    Ok(cards.len())
}
