use crate::components::VocabularyCard;
use crate::data::{LearningDirection, get_expression_pair, get_expression_unit_card_count, get_ui_strings};
use leptos::prelude::*;
use leptos_router::{components::A, hooks::use_params_map, hooks::use_query_map};

/// Expression card learning component
#[component]
pub fn ExpressionCards() -> impl IntoView {
    let params = use_params_map();
    let query = use_query_map();
    let lang_ctx = expect_context::<crate::core::LanguageContext>();
    let ui = move || get_ui_strings(lang_ctx.language.get());
    let ui_lang = move || match lang_ctx.language.get() {
        crate::core::Language::German => "de",
        _ => "en",
    };

    // Extract unit from URL params
    let unit = move || {
        params
            .read()
            .get("unit")
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

    // State for showing example and translation - persist at parent level
    let show_example = RwSignal::new(false);
    let show_translation = RwSignal::new(false);

    // Initialize card count when unit changes
    Effect::new(move |_| {
        let current_unit = unit();
        if let Ok(count) = get_expression_unit_card_count(current_unit) {
            set_card_count.set(count);
            set_card_index.set(0);
        }
    });

    // Reset translation visibility when card changes
    Effect::new(move |_| {
        let _ = card_index.get();
        show_translation.set(false);
    });

    // Get current card pair (phrase-only, no example)
    let current_card = move || {
        let current_unit = unit();
        let index = card_index.get();
        get_expression_pair(current_unit, index, direction(), ui_lang())
    };

    // Navigation handlers
    let go_next_card = move || {
        if card_index.get() < card_count.get() - 1 {
            set_card_index.update(|i| *i += 1);
        }
    };

    let go_prev_card = move || {
        if card_index.get() > 0 {
            set_card_index.update(|i| *i -= 1);
        }
    };

    view! {
        <div class="page-container">
            <header class="page-header">
                <A href={move || format!("/expressions?dir={}", if direction() == LearningDirection::EnglishToSpanish { "en-to-es" } else { "es-to-en" })} attr:class="back-button">"❮"</A>
                <h1>{move || format!("{} {}", ui().unit_prefix, unit())}</h1>
            </header>

            <div class="card-learning-container">
                {move || {
                    match current_card() {
                        Ok((source, target)) => {
                            view! {
                                <div class="card-wrapper">
                                    <VocabularyCard
                                        source_word={source.phrase.clone()}
                                        source_example={String::new()}
                                        target_word={target.phrase.clone()}
                                        target_example={String::new()}
                                        card_index={card_index.get()}
                                        card_count={card_count.get()}
                                        is_favorite={false}
                                        direction={direction()}
                                        show_example={show_example}
                                        show_translation={show_translation}
                                        show_example_section={false}
                                        compact_title={true}
                                        show_favorite={false}
                                        on_toggle_favorite=move || {}
                                        on_prev=move || go_prev_card()
                                        on_next=move || go_next_card()
                                    />
                                </div>
                            }.into_any()
                        }
                        Err(e) => view! {
                            <div class="error-message">
                                <p>{move || ui().error_loading_cards.clone()} {e}</p>
                            </div>
                        }.into_any()
                    }
                }}
            </div>
        </div>
    }
}
