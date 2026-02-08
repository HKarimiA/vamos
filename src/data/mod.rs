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

/// Load vocabulary cards for a specific stage and language
pub fn load_vocabulary_stage(stage: u32, language: &str) -> Result<Vec<VocabularyCard>, String> {
    let json_data = match (stage, language) {
        (1, "es") => include_str!("../../translations/vocabulary/1/es.json"),
        (1, "en") => include_str!("../../translations/vocabulary/1/en.json"),
        (2, "es") => include_str!("../../translations/vocabulary/2/es.json"),
        (2, "en") => include_str!("../../translations/vocabulary/2/en.json"),
        (3, "es") => include_str!("../../translations/vocabulary/3/es.json"),
        (3, "en") => include_str!("../../translations/vocabulary/3/en.json"),
        // Add more stages here as you create them
        _ => return Err(format!("Stage {} for language {} not found", stage, language)),
    };

    serde_json::from_str(json_data)
        .map_err(|e| format!("Failed to parse JSON for stage {}: {}", stage, e))
}

/// Get a pair of cards (source and target language) for a specific stage and card index
pub fn get_card_pair(
    stage: u32,
    card_index: usize,
    direction: LearningDirection,
) -> Result<(VocabularyCard, VocabularyCard), String> {
    let spanish_cards = load_vocabulary_stage(stage, "es")?;
    let english_cards = load_vocabulary_stage(stage, "en")?;

    if card_index >= spanish_cards.len() || card_index >= english_cards.len() {
        return Err("Card index out of bounds".to_string());
    }

    match direction {
        LearningDirection::SpanishToEnglish => {
            Ok((spanish_cards[card_index].clone(), english_cards[card_index].clone()))
        }
        LearningDirection::EnglishToSpanish => {
            Ok((english_cards[card_index].clone(), spanish_cards[card_index].clone()))
        }
    }
}

/// Get total number of cards in a stage
pub fn get_stage_card_count(stage: u32) -> Result<usize, String> {
    let cards = load_vocabulary_stage(stage, "es")?;
    Ok(cards.len())
}
