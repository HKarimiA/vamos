use crate::components::VocabularyCard;
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
    let (card_count, set_card_count) = signal(0usize);

    // Initialize card count when stage changes
    Effect::new(move |_| {
        let current_stage = stage();
        if let Ok(count) = get_stage_card_count(current_stage) {
            set_card_count.set(count);
            set_card_index.set(0);
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
        }
    };

    let go_prev = move |_| {
        if card_index.get() > 0 {
            set_card_index.update(|i| *i -= 1);
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

    view! {
        <div class="page-container">
            <header class="page-header">
                <A href={move || format!("/vocabulary?dir={}", if direction() == LearningDirection::EnglishToSpanish { "en-to-es" } else { "es-to-en" })} attr:class="back-button">"❮"</A>
                <h1>"Stage " {move || stage()}</h1>
            </header>

            <div class="card-learning-container">
                {move || {
                    match current_card() {
                        Ok((source, target)) => {
                            view! {
                                <div class="card-wrapper">
                                    <VocabularyCard
                                        source_word={source.word.clone()}
                                        source_example={source.example.clone()}
                                        target_word={target.word.clone()}
                                        target_example={target.example.clone()}
                                        card_index={card_index.get()}
                                        card_count={card_count.get()}
                                        is_favorite={is_favorite()}
                                        direction={direction()}
                                        on_toggle_favorite=move || toggle_favorite(())
                                    />

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
                                <A href="/vocabulary" attr:class="back-button">"❮"</A>
                            </div>
                        }.into_any()
                    }
                }}
            </div>
        </div>
    }
}
