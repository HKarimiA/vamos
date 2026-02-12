use crate::components::VocabularyCard;
use crate::core::FavoritesContext;
use crate::data::{LearningDirection, get_card_pair};
use leptos::prelude::*;
use leptos_router::components::A;
use leptos_router::hooks::use_query_map;

/// Favorites page - Shows all favorited cards with card navigation
#[component]
pub fn Favorites() -> impl IntoView {
    let favorites_ctx = expect_context::<FavoritesContext>();
    let query = use_query_map();

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

    // Get filtered list of valid favorites, sorted by card_id
    let favorite_cards = move || {
        let all = favorites_ctx.get_all();
        let mut filtered: Vec<_> = all
            .into_iter()
            .filter(|(stage, card_id)| {
                // Stage 1: IDs 1-20, Stage 2: IDs 21-40, Stage 3: IDs 41-60, etc.
                match stage {
                    i if (1..=21).contains(i) => {
                        let start_id = (i - 1) * 20 + 1;
                        let end_id = i * 20;
                        *card_id >= start_id && *card_id <= end_id
                    }
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
        let card_idx = match stage {
            i if (1..=21).contains(&i) => (card_id - ((i - 1) * 20 + 1)) as usize,
            _ => return Err("Invalid stage".to_string()),
        };
        get_card_pair(stage, card_idx, direction()).map(|(source, target)| (stage, source, target))
    };

    // Navigation handlers
    let go_next = move |_| {
        let cards = favorite_cards();
        if card_index.get() < cards.len() - 1 {
            set_card_index.update(|i| *i += 1);
        }
    };

    let go_prev = move |_| {
        if card_index.get() > 0 {
            set_card_index.update(|i| *i -= 1);
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
        }
    };

    view! {
        <div class="page-container">
            <header class="page-header">
                <A href={move || format!("/vocabulary?dir={}", if direction() == LearningDirection::EnglishToSpanish { "en-to-es" } else { "es-to-en" })} attr:class="back-button">"❮"</A>
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
                            </div>
                        }.into_any()
                    } else {
                        match current_card() {
                            Ok((stage, source, target)) => {
                                view! {
                                    <div class="card-wrapper">
                                        <VocabularyCard
                                            source_word={source.word.clone()}
                                            source_example={source.example.clone()}
                                            target_word={target.word.clone()}
                                            target_example={target.example.clone()}
                                            card_index={card_index.get()}
                                            card_count={favorite_cards().len()}
                                            is_favorite={true}
                                            direction={direction()}
                                            stage=stage
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
                                    <A href="/vocabulary" attr:class="back-button">"❮"</A>
                                </div>
                            }.into_any()
                        }
                    }
                }}
            </div>
        </div>
    }
}
