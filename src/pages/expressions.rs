use crate::core::{Language, LanguageContext};
use crate::data::get_ui_strings;
use leptos::prelude::*;
use leptos_router::components::A;
use leptos_router::hooks::{use_navigate, use_query_map};

/// Expressions learning page - Shows unit selection grid
#[component]
pub fn Expressions() -> impl IntoView {
    let lang_ctx = expect_context::<LanguageContext>();
    let ui = move || get_ui_strings(lang_ctx.language.get());
    let query = use_query_map();
    let navigate = use_navigate();

    // State for learning direction - sync with URL query param
    let direction = Memo::new(move |_| {
        query
            .read()
            .get("dir")
            .filter(|d| d == "en-to-es" || d == "es-to-en")
            .unwrap_or("es-to-en".to_string())
    });

    // Toggle direction handler
    let toggle_direction = move |_| {
        let new_dir = if direction.get() == "es-to-en" {
            "en-to-es"
        } else {
            "es-to-en"
        };
        // Update URL to persist direction in browser history
        navigate(&format!("/expressions?dir={}", new_dir), Default::default());
    };

    view! {
        <div class="page-container">
            <header class="page-header">
                <A href="/" attr:class="back-button">"❮"</A>
                <h1>{move || ui().expressions}</h1>
                <button
                    class="direction-toggle"
                    on:click=toggle_direction
                >
                    {move || {
                        let native_flag = match lang_ctx.language.get() {
                            Language::German => "🇩🇪",
                            _ => "🇬🇧",
                        };
                        if direction.get() == "es-to-en" {
                            format!("🇪🇸 → {}", native_flag)
                        } else {
                            format!("{} → 🇪🇸", native_flag)
                        }
                    }}
                </button>
            </header>

            <div class="vocab-content">
                <div class="stage-grid">
                    {move || (1..=27).map(|unit| {
                        let href = format!("/expressions/{}?dir={}", unit, direction.get());
                        view! {
                            <A href=href attr:class="stage-button">
                                {unit.to_string()}
                            </A>
                        }
                    }).collect::<Vec<_>>()}
                </div>
            </div>
        </div>
    }
}
