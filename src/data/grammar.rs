use serde::{Deserialize, Serialize};

/// Grammar topic information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrammarTopic {
    pub id: u32,
    pub difficulty: u32, // 1-4, where 1 is easiest
}

/// Get all grammar topics with their metadata
pub fn get_all_topics() -> Vec<GrammarTopic> {
    vec![
        GrammarTopic {
            id: 1,
            difficulty: 2,
        },
        GrammarTopic {
            id: 2,
            difficulty: 1,
        },
        GrammarTopic {
            id: 3,
            difficulty: 2,
        },
        GrammarTopic {
            id: 4,
            difficulty: 3,
        },
        GrammarTopic {
            id: 5,
            difficulty: 3,
        },
        GrammarTopic {
            id: 6,
            difficulty: 2,
        },
        GrammarTopic {
            id: 7,
            difficulty: 2,
        },
        GrammarTopic {
            id: 8,
            difficulty: 1,
        },
        GrammarTopic {
            id: 9,
            difficulty: 2,
        },
        GrammarTopic {
            id: 10,
            difficulty: 2,
        },
        GrammarTopic {
            id: 11,
            difficulty: 3,
        },
        GrammarTopic {
            id: 12,
            difficulty: 4,
        },
        GrammarTopic {
            id: 13,
            difficulty: 2,
        },
        GrammarTopic {
            id: 14,
            difficulty: 1,
        },
        GrammarTopic {
            id: 15,
            difficulty: 2,
        },
        GrammarTopic {
            id: 16,
            difficulty: 2,
        },
        GrammarTopic {
            id: 17,
            difficulty: 1,
        },
        GrammarTopic {
            id: 18,
            difficulty: 2,
        },
        GrammarTopic {
            id: 19,
            difficulty: 3,
        },
        GrammarTopic {
            id: 20,
            difficulty: 3,
        },
        GrammarTopic {
            id: 21,
            difficulty: 3,
        },
        GrammarTopic {
            id: 22,
            difficulty: 4,
        },
        GrammarTopic {
            id: 23,
            difficulty: 3,
        },
        GrammarTopic {
            id: 24,
            difficulty: 3,
        },
        GrammarTopic {
            id: 25,
            difficulty: 3,
        },
        GrammarTopic {
            id: 26,
            difficulty: 2,
        },
        GrammarTopic {
            id: 27,
            difficulty: 3,
        },
    ]
}

/// Get difficulty color class based on difficulty level
pub fn get_difficulty_class(difficulty: u32) -> &'static str {
    match difficulty {
        1 => "difficulty-1", // Lightest blue-purple (A1-A2)
        2 => "difficulty-2", // Light-medium (up to B1)
        3 => "difficulty-3", // Medium-dark (up to B2)
        4 => "difficulty-4", // Darkest (B2+ complex topics)
        _ => "difficulty-2",
    }
}

/// Answer option for a quiz question
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Answer {
    pub text: String,
    pub correct: bool,
}

/// Quiz question with multiple choice answers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Question {
    pub id: u32,
    pub question: String,
    pub answers: Vec<Answer>,
    pub hint: String,
}

/// Section within a topic explanation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExplanationSection {
    pub subtitle: String,
    pub content: String,
}

/// Topic explanation with title and sections
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopicExplanation {
    pub title: String,
    pub sections: Vec<ExplanationSection>,
}

/// Basic topic info (duplicates GrammarTopic for JSON structure)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopicInfo {
    pub id: u32,
    pub name: String,
}

/// Root structure for grammar topic JSON content
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrammarContent {
    pub topic: TopicInfo,
    pub explanation: TopicExplanation,
    pub questions: Vec<Question>,
}

/// Macro to dynamically generate match arms for grammar topic loading
macro_rules! include_grammar_topics {
    ($topic_id:expr, $lang:expr, [$($num:literal),* $(,)?]) => {
        if $lang == "de" {
            match $topic_id {
                $(
                    $num => include_str!(concat!("../../translations/grammar/", stringify!($num), "/de.json")),
                )*
                _ => return Err(format!("Topic {} not yet available", $topic_id)),
            }
        } else {
            match $topic_id {
                $(
                    $num => include_str!(concat!("../../translations/grammar/", stringify!($num), "/en.json")),
                )*
                _ => return Err(format!("Topic {} not yet available", $topic_id)),
            }
        }
    };
}

/// Load grammar content for a specific topic
pub fn load_grammar_content(topic_id: u32, lang: &str) -> Result<GrammarContent, String> {
    let json = include_grammar_topics!(
        topic_id,
        lang,
        [
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
            25, 26, 27
        ]
    );

    serde_json::from_str(json).map_err(|e| format!("Failed to parse grammar content: {}", e))
}
