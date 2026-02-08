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
        (4, "es") => include_str!("../../translations/vocabulary/4/es.json"),
        (4, "en") => include_str!("../../translations/vocabulary/4/en.json"),
        (5, "es") => include_str!("../../translations/vocabulary/5/es.json"),
        (5, "en") => include_str!("../../translations/vocabulary/5/en.json"),
        (6, "es") => include_str!("../../translations/vocabulary/6/es.json"),
        (6, "en") => include_str!("../../translations/vocabulary/6/en.json"),
        (7, "es") => include_str!("../../translations/vocabulary/7/es.json"),
        (7, "en") => include_str!("../../translations/vocabulary/7/en.json"),
        (8, "es") => include_str!("../../translations/vocabulary/8/es.json"),
        (8, "en") => include_str!("../../translations/vocabulary/8/en.json"),
        (9, "es") => include_str!("../../translations/vocabulary/9/es.json"),
        (9, "en") => include_str!("../../translations/vocabulary/9/en.json"),
        (10, "es") => include_str!("../../translations/vocabulary/10/es.json"),
        (10, "en") => include_str!("../../translations/vocabulary/10/en.json"),
        (11, "es") => include_str!("../../translations/vocabulary/11/es.json"),
        (11, "en") => include_str!("../../translations/vocabulary/11/en.json"),
        (12, "es") => include_str!("../../translations/vocabulary/12/es.json"),
        (12, "en") => include_str!("../../translations/vocabulary/12/en.json"),
        (13, "es") => include_str!("../../translations/vocabulary/13/es.json"),
        (13, "en") => include_str!("../../translations/vocabulary/13/en.json"),
        (14, "es") => include_str!("../../translations/vocabulary/14/es.json"),
        (14, "en") => include_str!("../../translations/vocabulary/14/en.json"),
        (15, "es") => include_str!("../../translations/vocabulary/15/es.json"),
        (15, "en") => include_str!("../../translations/vocabulary/15/en.json"),
        (16, "es") => include_str!("../../translations/vocabulary/16/es.json"),
        (16, "en") => include_str!("../../translations/vocabulary/16/en.json"),
        (17, "es") => include_str!("../../translations/vocabulary/17/es.json"),
        (17, "en") => include_str!("../../translations/vocabulary/17/en.json"),
        (18, "es") => include_str!("../../translations/vocabulary/18/es.json"),
        (18, "en") => include_str!("../../translations/vocabulary/18/en.json"),
        (19, "es") => include_str!("../../translations/vocabulary/19/es.json"),
        (19, "en") => include_str!("../../translations/vocabulary/19/en.json"),
        (20, "es") => include_str!("../../translations/vocabulary/20/es.json"),
        (20, "en") => include_str!("../../translations/vocabulary/20/en.json"),
        (21, "es") => include_str!("../../translations/vocabulary/21/es.json"),
        (21, "en") => include_str!("../../translations/vocabulary/21/en.json"),
        _ => {
            return Err(format!(
                "Stage {} for language {} not found",
                stage, language
            ));
        }
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
        LearningDirection::SpanishToEnglish => Ok((
            spanish_cards[card_index].clone(),
            english_cards[card_index].clone(),
        )),
        LearningDirection::EnglishToSpanish => Ok((
            english_cards[card_index].clone(),
            spanish_cards[card_index].clone(),
        )),
    }
}

/// Get total number of cards in a stage
pub fn get_stage_card_count(stage: u32) -> Result<usize, String> {
    let cards = load_vocabulary_stage(stage, "es")?;
    Ok(cards.len())
}
