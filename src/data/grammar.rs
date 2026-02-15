use serde::{Deserialize, Serialize};

/// Grammar topic information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrammarTopic {
    pub id: u32,
    pub name: String,
    pub difficulty: u32, // 1-4, where 1 is easiest
}

/// Get all grammar topics with their metadata
pub fn get_all_topics() -> Vec<GrammarTopic> {
    vec![
        GrammarTopic {
            id: 1,
            name: "Sounds & Writing".to_string(),
            difficulty: 2,
        },
        GrammarTopic {
            id: 2,
            name: "Numbers".to_string(),
            difficulty: 1,
        },
        GrammarTopic {
            id: 3,
            name: "Dates & Time".to_string(),
            difficulty: 2,
        },
        GrammarTopic {
            id: 4,
            name: "Nouns & Articles".to_string(),
            difficulty: 3,
        },
        GrammarTopic {
            id: 5,
            name: "Adjectives".to_string(),
            difficulty: 3,
        },
        GrammarTopic {
            id: 6,
            name: "Determiners".to_string(),
            difficulty: 2,
        },
        GrammarTopic {
            id: 7,
            name: "Pronouns".to_string(),
            difficulty: 2,
        },
        GrammarTopic {
            id: 8,
            name: "Present Tense".to_string(),
            difficulty: 1,
        },
        GrammarTopic {
            id: 9,
            name: "Ser vs Estar".to_string(),
            difficulty: 2,
        },
        GrammarTopic {
            id: 10,
            name: "Past Tenses".to_string(),
            difficulty: 2,
        },
        GrammarTopic {
            id: 11,
            name: "Future & Conditional".to_string(),
            difficulty: 3,
        },
        GrammarTopic {
            id: 12,
            name: "Subjunctive".to_string(),
            difficulty: 4,
        },
        GrammarTopic {
            id: 13,
            name: "Commands".to_string(),
            difficulty: 2,
        },
        GrammarTopic {
            id: 14,
            name: "Infinitive & Participles".to_string(),
            difficulty: 1,
        },
        GrammarTopic {
            id: 15,
            name: "Verb Combinations".to_string(),
            difficulty: 2,
        },
        GrammarTopic {
            id: 16,
            name: "Reflexive Verbs".to_string(),
            difficulty: 2,
        },
        GrammarTopic {
            id: 17,
            name: "Special Verbs".to_string(),
            difficulty: 1,
        },
        GrammarTopic {
            id: 18,
            name: "Adverbs".to_string(),
            difficulty: 2,
        },
        GrammarTopic {
            id: 19,
            name: "Prepositions".to_string(),
            difficulty: 3,
        },
        GrammarTopic {
            id: 20,
            name: "Conjunctions".to_string(),
            difficulty: 3,
        },
        GrammarTopic {
            id: 21,
            name: "Sentence Structure".to_string(),
            difficulty: 3,
        },
        GrammarTopic {
            id: 22,
            name: "Conditionals".to_string(),
            difficulty: 4,
        },
        GrammarTopic {
            id: 23,
            name: "Reported Speech".to_string(),
            difficulty: 3,
        },
        GrammarTopic {
            id: 24,
            name: "Agreement".to_string(),
            difficulty: 3,
        },
        GrammarTopic {
            id: 25,
            name: "Passive Voice".to_string(),
            difficulty: 3,
        },
        GrammarTopic {
            id: 26,
            name: "Negation".to_string(),
            difficulty: 2,
        },
        GrammarTopic {
            id: 27,
            name: "Formal vs Informal".to_string(),
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

/// Load grammar content for a specific topic
pub fn load_grammar_content(topic_id: u32) -> Result<GrammarContent, String> {
    let json = match topic_id {
        1 => include_str!("../../translations/grammar/1/en.json"),
        2 => include_str!("../../translations/grammar/2/en.json"),
        3 => include_str!("../../translations/grammar/3/en.json"),
        4 => include_str!("../../translations/grammar/4/en.json"),
        5 => include_str!("../../translations/grammar/5/en.json"),
        6 => include_str!("../../translations/grammar/6/en.json"),
        7 => include_str!("../../translations/grammar/7/en.json"),
        8 => include_str!("../../translations/grammar/8/en.json"),
        9 => include_str!("../../translations/grammar/9/en.json"),
        _ => return Err(format!("Topic {} not yet available", topic_id)),
    };

    serde_json::from_str(json).map_err(|e| format!("Failed to parse grammar content: {}", e))
}
