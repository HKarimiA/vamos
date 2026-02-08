use crate::core::FavoritesContext;
use crate::data::{LearningDirection, get_card_pair, get_stage_card_count};
use leptos::prelude::*;
use leptos_router::{components::A, hooks::use_params_map, hooks::use_query_map};

/// Vocabulary card learning component
#[component]
pub fn VocabularyCards() -> impl IntoView {
    let params = use_params_map();
    let query = use_query_map();
    let favorites_ctx = expect_context::<FavoritesContext>();

    // Extract stage from URL params
    let stage = move || {
        params
            .read()
            .get("stage")
            .and_then(|s| s.parse::<u32>().ok())
            .unwrap_or(1)
    };

    // Extract direction from query params
    let direction = move || {
        query
            .read()
            .get("dir")
            .map(|d| {
                if d == "en-to-es" {
                    LearningDirection::EnglishToSpanish
                } else {
                    LearningDirection::SpanishToEnglish
                }
            })
            .unwrap_or(LearningDirection::SpanishToEnglish)
    };

    // State management
    let (card_index, set_card_index) = signal(0usize);
    let (show_example, set_show_example) = signal(false);
    let (show_translation, set_show_translation) = signal(false);
    let (card_count, set_card_count) = signal(0usize);

    // Initialize card count when stage changes
    Effect::new(move |_| {
        let current_stage = stage();
        if let Ok(count) = get_stage_card_count(current_stage) {
            set_card_count.set(count);
            set_card_index.set(0);
            set_show_example.set(false);
            set_show_translation.set(false);
        }
    });

    // Get current card
    let current_card = move || {
        let current_stage = stage();
        let index = card_index.get();
        get_card_pair(current_stage, index, direction())
    };

    // Navigation handlers
    let go_next = move |_| {
        if card_index.get() < card_count.get() - 1 {
            set_card_index.update(|i| *i += 1);
            set_show_example.set(false);
            set_show_translation.set(false);
        }
    };

    let go_prev = move |_| {
        if card_index.get() > 0 {
            set_card_index.update(|i| *i -= 1);
            set_show_example.set(false);
            set_show_translation.set(false);
        }
    };

    // Toggle favorite
    let toggle_favorite = move |_| {
        let current_stage = stage();
        if let Ok((source, _)) = current_card() {
            favorites_ctx.toggle(current_stage, source.id);
        }
    };

    // Check if current card is favorite
    let is_favorite = move || {
        let current_stage = stage();
        current_card()
            .ok()
            .map(|(source, _)| favorites_ctx.is_favorite(current_stage, source.id))
            .unwrap_or(false)
    };

    // Speak word using Web Speech API
    let speak = move |text: String, lang: &str| {
        let lang = lang.to_string();
        leptos::logging::log!("Speaking: {} in {}", text, lang);
        // We'll use JavaScript interop for Web Speech API
        #[cfg(target_arch = "wasm32")]
        {
            use wasm_bindgen::prelude::*;
            #[wasm_bindgen]
            unsafe extern "C" {
                #[wasm_bindgen(js_namespace = window)]
                fn speak_text(text: &str, lang: &str);
            }
            unsafe {
                speak_text(&text, &lang);
            }
        }
    };

    view! {
        <div class="page-container">
            <header class="page-header">
                <A href={move || format!("/vocabulary?dir={}", if direction() == LearningDirection::EnglishToSpanish { "en-to-es" } else { "es-to-en" })} attr:class="back-button">"← Stages"</A>
                <h1>"Stage " {move || stage()}</h1>
            </header>

            <div class="card-learning-container">
                {move || {
                    match current_card() {
                        Ok((source, target)) => {
                            let source_lang = match direction() {
                                LearningDirection::SpanishToEnglish => "es-ES",
                                LearningDirection::EnglishToSpanish => "en-US",
                            };
                            let source_word = source.word.clone();

                            view! {
                                <div class="card-wrapper">
                                    <div class="card-progress">
                                        {move || format!("{} / {}", card_index.get() + 1, card_count.get())}
                                    </div>

                                    <div class="vocabulary-card">
                                        <div class="card-main">
                                            <h2 class="card-word">{source.word.clone()}</h2>
                                            <div class="card-actions">
                                                <button
                                                    class="audio-button"
                                                    on:click=move |_| speak(source_word.clone(), source_lang)
                                                >
                                                    "🔊"
                                                </button>
                                                <button
                                                    class=move || if is_favorite() { "favorite-button favorite-active" } else { "favorite-button" }
                                                    on:click=toggle_favorite
                                                >
                                                    {move || if is_favorite() { "⭐" } else { "☆" }}
                                                </button>
                                            </div>
                                        </div>

                                        {move || (!show_example.get()).then(|| view! {
                                            <button
                                                class="reveal-button"
                                                on:click=move |_| set_show_example.set(true)
                                            >
                                                "Show Example"
                                            </button>
                                        })}

                                        {move || show_example.get().then(|| view! {
                                            <p class="card-example">{source.example.clone()}</p>
                                        })}

                                        {move || (!show_translation.get()).then(|| view! {
                                            <button
                                                class="reveal-button translation-button"
                                                on:click=move |_| set_show_translation.set(true)
                                            >
                                                "Show Translation"
                                            </button>
                                        })}

                                        {move || show_translation.get().then(|| view! {
                                            <div class="card-translation">
                                                <p class="translation-word">{target.word.clone()}</p>
                                                <p class="translation-example">{target.example.clone()}</p>
                                            </div>
                                        })}
                                    </div>

                                    <div class="card-navigation">
                                        <button
                                            class="nav-btn"
                                            on:click=go_prev
                                            disabled={move || card_index.get() == 0}
                                        >
                                            "← Previous"
                                        </button>
                                        <button
                                            class="nav-btn"
                                            on:click=go_next
                                            disabled={move || card_index.get() >= card_count.get() - 1}
                                        >
                                            "Next →"
                                        </button>
                                    </div>
                                </div>
                            }.into_any()
                        }
                        Err(e) => view! {
                            <div class="error-message">
                                <p>"Error loading cards: " {e}</p>
                                <A href="/vocabulary" attr:class="back-button">"← Back to Stages"</A>
                            </div>
                        }.into_any()
                    }
                }}
            </div>
        </div>
    }
}
