use leptos::prelude::*;

/// Represents available languages for learning or UI
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Language {
    Spanish,
    English,
    German,
}

#[allow(dead_code)]
impl Language {
    /// Display name in English
    pub fn display_name(&self) -> &'static str {
        match self {
            Language::Spanish => "Spanish",
            Language::English => "English",
            Language::German => "German",
        }
    }

    /// Native name of the language
    pub fn native_name(&self) -> &'static str {
        match self {
            Language::Spanish => "Español",
            Language::English => "English",
            Language::German => "Deutsch",
        }
    }

    /// Flag emoji representation
    pub fn flag_emoji(&self) -> &'static str {
        match self {
            Language::Spanish => "🇪🇸",
            Language::English => "🇬🇧",
            Language::German => "🇩🇪",
        }
    }
}

/// Global context for the current UI language
#[derive(Clone, Copy)]
pub struct LanguageContext {
    pub language: RwSignal<Language>,
}

impl LanguageContext {
    pub fn new() -> Self {
        Self {
            language: RwSignal::new(Language::English),
        }
    }

    pub fn set_language(&self, lang: Language) {
        self.language.set(lang);
    }
}

// === CONFIGURATION ===

/// The language being learned by the user (always Spanish)
#[allow(dead_code)]
pub const LEARNING_LANGUAGE: Language = Language::Spanish;
