/// Represents available languages for learning or UI
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Language {
    Spanish,
    English,
    // Future: French, German, Italian, Portuguese, etc.
}

#[allow(dead_code)]
impl Language {
    /// Display name in English
    pub fn display_name(&self) -> &'static str {
        match self {
            Language::Spanish => "Spanish",
            Language::English => "English",
        }
    }

    /// Native name of the language
    pub fn native_name(&self) -> &'static str {
        match self {
            Language::Spanish => "Español",
            Language::English => "English",
        }
    }

    /// Flag emoji representation
    pub fn flag_emoji(&self) -> &'static str {
        match self {
            Language::Spanish => "🇪🇸",
            Language::English => "🇺🇸",
        }
    }
}

// === CONFIGURATION ===
// Change these constants to switch languages globally

/// The language being learned by the user
#[allow(dead_code)]
pub const LEARNING_LANGUAGE: Language = Language::Spanish;

/// The language used for UI and instructions
#[allow(dead_code)]
pub const UI_LANGUAGE: Language = Language::English;
