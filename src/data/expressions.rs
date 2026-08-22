use crate::data::vocabulary::LearningDirection;
use serde::{Deserialize, Serialize};

/// Represents a single everyday expression card (phrase only, no example sentence)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ExpressionCard {
    pub id: u32,
    pub phrase: String,
}

/// Macro to dynamically generate match arms for expression unit loading
macro_rules! include_expression_units {
    ($unit:expr, $language:expr, [$($num:literal),* $(,)?]) => {
        match ($unit, $language) {
            $(
                ($num, "es") => include_str!(concat!("../../translations/expressions/", stringify!($num), "/es.json")),
                ($num, "en") => include_str!(concat!("../../translations/expressions/", stringify!($num), "/en.json")),
                ($num, "de") => include_str!(concat!("../../translations/expressions/", stringify!($num), "/de.json")),
            )*
            _ => {
                return Err(format!(
                    "Unit {} for language {} not found",
                    $unit, $language
                ));
            }
        }
    };
}

/// Load expression cards for a specific unit and language
pub fn load_expression_unit(unit: u32, language: &str) -> Result<Vec<ExpressionCard>, String> {
    let json_data = include_expression_units!(
        unit,
        language,
        [
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
            25, 26, 27
        ]
    );

    serde_json::from_str(json_data)
        .map_err(|e| format!("Failed to parse JSON for unit {}: {}", unit, e))
}

/// Get a pair of cards (source and target language) for a specific unit and card index.
/// `ui_lang` selects the non-Spanish side: "en" for English, "de" for German.
pub fn get_expression_pair(
    unit: u32,
    card_index: usize,
    direction: LearningDirection,
    ui_lang: &str,
) -> Result<(ExpressionCard, ExpressionCard), String> {
    let spanish_cards = load_expression_unit(unit, "es")?;
    let native_cards = load_expression_unit(unit, ui_lang)?;

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

/// Get total number of cards in an expression unit
pub fn get_expression_unit_card_count(unit: u32) -> Result<usize, String> {
    let cards = load_expression_unit(unit, "es")?;
    Ok(cards.len())
}
