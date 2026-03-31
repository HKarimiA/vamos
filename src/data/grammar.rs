use serde::{Deserialize, Serialize};

/// Grammar topic information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrammarTopic {
    pub id: u32,
    pub name: String,
    pub difficulty: u32, // 1-4, where 1 is easiest
}

/// Get all grammar topics with their metadata
pub fn get_all_topics(lang: &str) -> Vec<GrammarTopic> {
    let de = lang == "de";
    vec![
        GrammarTopic {
            id: 1,
            name: if de {
                "Laute & Schreibsystem"
            } else {
                "Sounds & Writing"
            }
            .to_string(),
            difficulty: 2,
        },
        GrammarTopic {
            id: 2,
            name: if de { "Zahlen" } else { "Numbers" }.to_string(),
            difficulty: 1,
        },
        GrammarTopic {
            id: 3,
            name: if de {
                "Datum & Uhrzeit"
            } else {
                "Dates & Time"
            }
            .to_string(),
            difficulty: 2,
        },
        GrammarTopic {
            id: 4,
            name: if de {
                "Nomen & Artikel"
            } else {
                "Nouns & Articles"
            }
            .to_string(),
            difficulty: 3,
        },
        GrammarTopic {
            id: 5,
            name: if de { "Adjektive" } else { "Adjectives" }.to_string(),
            difficulty: 3,
        },
        GrammarTopic {
            id: 6,
            name: if de { "Determinatoren" } else { "Determiners" }.to_string(),
            difficulty: 2,
        },
        GrammarTopic {
            id: 7,
            name: if de { "Pronomen" } else { "Pronouns" }.to_string(),
            difficulty: 2,
        },
        GrammarTopic {
            id: 8,
            name: if de { "Präsens" } else { "Present Tense" }.to_string(),
            difficulty: 1,
        },
        GrammarTopic {
            id: 9,
            name: "Ser vs Estar".to_string(),
            difficulty: 2,
        },
        GrammarTopic {
            id: 10,
            name: if de {
                "Vergangenheitstempora"
            } else {
                "Past Tenses"
            }
            .to_string(),
            difficulty: 2,
        },
        GrammarTopic {
            id: 11,
            name: if de {
                "Futur & Konditional"
            } else {
                "Future & Conditional"
            }
            .to_string(),
            difficulty: 3,
        },
        GrammarTopic {
            id: 12,
            name: if de { "Konjunktiv" } else { "Subjunctive" }.to_string(),
            difficulty: 4,
        },
        GrammarTopic {
            id: 13,
            name: if de {
                "Imperativ (Befehle)"
            } else {
                "Commands"
            }
            .to_string(),
            difficulty: 2,
        },
        GrammarTopic {
            id: 14,
            name: if de {
                "Infinite Verbformen"
            } else {
                "Infinitive & Participles"
            }
            .to_string(),
            difficulty: 1,
        },
        GrammarTopic {
            id: 15,
            name: if de {
                "Periphrastische & Aspektuelle Konstruktionen"
            } else {
                "Verb Combinations"
            }
            .to_string(),
            difficulty: 2,
        },
        GrammarTopic {
            id: 16,
            name: if de {
                "Reflexive & Pronominale Verben"
            } else {
                "Reflexive Verbs"
            }
            .to_string(),
            difficulty: 2,
        },
        GrammarTopic {
            id: 17,
            name: if de {
                "Besondere Verbmuster"
            } else {
                "Special Verbs"
            }
            .to_string(),
            difficulty: 1,
        },
        GrammarTopic {
            id: 18,
            name: if de { "Adverbien" } else { "Adverbs" }.to_string(),
            difficulty: 2,
        },
        GrammarTopic {
            id: 19,
            name: if de { "Präpositionen" } else { "Prepositions" }.to_string(),
            difficulty: 3,
        },
        GrammarTopic {
            id: 20,
            name: if de {
                "Konjunktionen & Konnektoren"
            } else {
                "Conjunctions"
            }
            .to_string(),
            difficulty: 3,
        },
        GrammarTopic {
            id: 21,
            name: if de {
                "Satzbau & Syntax"
            } else {
                "Sentence Structure"
            }
            .to_string(),
            difficulty: 3,
        },
        GrammarTopic {
            id: 22,
            name: if de {
                "Konditionalsätze"
            } else {
                "Conditionals"
            }
            .to_string(),
            difficulty: 4,
        },
        GrammarTopic {
            id: 23,
            name: if de {
                "Indirekte Rede"
            } else {
                "Reported Speech"
            }
            .to_string(),
            difficulty: 3,
        },
        GrammarTopic {
            id: 24,
            name: if de {
                "Kongruenz & Übereinstimmung"
            } else {
                "Agreement"
            }
            .to_string(),
            difficulty: 3,
        },
        GrammarTopic {
            id: 25,
            name: if de {
                "Passiv & Unpersönliche Konstruktionen"
            } else {
                "Passive Voice"
            }
            .to_string(),
            difficulty: 3,
        },
        GrammarTopic {
            id: 26,
            name: if de { "Verneinung" } else { "Negation" }.to_string(),
            difficulty: 2,
        },
        GrammarTopic {
            id: 27,
            name: if de {
                "Register & Sprachebene"
            } else {
                "Formal vs Informal"
            }
            .to_string(),
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
