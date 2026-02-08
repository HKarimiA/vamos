use crate::core::FavoritesContext;
use crate::data::{LearningDirection, get_card_pair};
use leptos::prelude::*;
use leptos_router::components::A;

/// Favorites page - Shows all favorited cards with card navigation
#[component]
pub fn Favorites() -> impl IntoView {
    let favorites_ctx = expect_context::<FavoritesContext>();

    // State management
    let (card_index, set_card_index) = signal(0usize);
    let (show_example, set_show_example) = signal(false);
    let (show_translation, set_show_translation) = signal(false);

    // Get filtered list of valid favorites, sorted by card_id
    let favorite_cards = move || {
        let all = favorites_ctx.get_all();
        let mut filtered: Vec<_> = all
            .into_iter()
            .filter(|(stage, card_id)| {
                // Stage 1: IDs 1-20, Stage 2: IDs 21-40, Stage 3: IDs 41-60
                match stage {
                    1 => *card_id >= 1 && *card_id <= 20,
                    2 => *card_id >= 21 && *card_id <= 40,
                    3 => *card_id >= 41 && *card_id <= 60,
                    _ => false,
                }
            })
            .collect();
        filtered.sort_by_key(|(_, card_id)| *card_id);
        filtered
    };

    // Get current favorite card
    let current_card = move || {
        let cards = favorite_cards();
        if cards.is_empty() || card_index.get() >= cards.len() {
            return Err("No favorites available".to_string());
        }
        let (stage, card_id) = cards[card_index.get()];
        // Convert global ID to stage-relative index
        // Stage 1: IDs 1-20 -> index 0-19
        // Stage 2: IDs 21-40 -> index 0-19
        // Stage 3: IDs 41-60 -> index 0-19
        let card_idx = match stage {
            1 => (card_id - 1) as usize,
            2 => (card_id - 21) as usize,
            3 => (card_id - 41) as usize,
            _ => return Err("Invalid stage".to_string()),
        };
        get_card_pair(stage, card_idx, LearningDirection::SpanishToEnglish)
            .map(|(source, target)| (stage, source, target))
    };

    // Navigation handlers
    let go_next = move |_| {
        let cards = favorite_cards();
        if card_index.get() < cards.len() - 1 {
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

    // Toggle favorite (remove from favorites)
    let toggle_favorite = move |_| {
        let cards = favorite_cards();
        if let Some((stage, card_id)) = cards.get(card_index.get()) {
            favorites_ctx.toggle(*stage, *card_id);
            // If this was the last card or we're at the end, go to previous
            let new_count = favorite_cards().len();
            if new_count == 0 {
                set_card_index.set(0);
            } else if card_index.get() >= new_count {
                set_card_index.set(new_count - 1);
            }
            set_show_example.set(false);
            set_show_translation.set(false);
        }
    };

    // Speak word using Web Speech API
    let speak = move |text: String, lang: &str| {
        let lang = lang.to_string();
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
                <A href="/vocabulary" attr:class="back-button">"← Stages"</A>
                <h1>"Favorites"</h1>
            </header>

            <div class="card-learning-container">
                {move || {
                    let cards = favorite_cards();

                    if cards.is_empty() {
                        view! {
                            <div class="error-message">
                                <p>"No favorites yet!"</p>
                                <p style="color: #666; font-size: 1rem;">"Add cards to favorites by clicking the ☆ icon"</p>
                                <A href="/vocabulary" attr:class="back-button">"← Back to Stages"</A>
                            </div>
                        }.into_any()
                    } else {
                        match current_card() {
                            Ok((stage, source, target)) => {
                                let source_word = source.word.clone();
                                let source_lang = "es-ES";

                                view! {
                                    <div class="card-wrapper">
                                        <div class="card-progress">
                                            {move || format!("{} / {} (Stage {})", card_index.get() + 1, favorite_cards().len(), stage)}
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
                                                        class="favorite-button favorite-active"
                                                        on:click=toggle_favorite
                                                    >
                                                        "⭐"
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
                                                disabled={move || card_index.get() >= favorite_cards().len() - 1}
                                            >
                                                "Next →"
                                            </button>
                                        </div>
                                    </div>
                                }.into_any()
                            }
                            Err(e) => view! {
                                <div class="error-message">
                                    <p>"Error loading card: " {e}</p>
                                    <A href="/vocabulary" attr:class="back-button">"← Back to Stages"</A>
                                </div>
                            }.into_any()
                        }
                    }
                }}
            </div>
        </div>
    }
}
