use leptos::prelude::*;
use leptos_router::components::A;
use leptos_router::hooks::{use_navigate, use_query_map};

/// Vocabulary learning page - Shows stage selection grid
#[component]
pub fn Vocabulary() -> impl IntoView {
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
        navigate(&format!("/vocabulary?dir={}", new_dir), Default::default());
    };

    view! {
        <div class="page-container">
            <header class="page-header">
                <A href="/" attr:class="back-button">"❮"</A>
                <h1>"Vocabulary"</h1>
                <button
                    class="direction-toggle"
                    on:click=toggle_direction
                >
                    {move || {
                        if direction.get() == "es-to-en" {
                            "🇪🇸 → 🇬🇧"
                        } else {
                            "🇬🇧 → 🇪🇸"
                        }
                    }}
                </button>
            </header>

            <div class="vocab-content">
                <div class="stage-grid">
                    {move || (1..=21).map(|stage| {
                        let href = format!("/vocabulary/{}?dir={}", stage, direction.get());
                        view! {
                            <A href=href attr:class="stage-button">
                                {stage.to_string()}
                            </A>
                        }
                    }).collect::<Vec<_>>()}

                    <A href={move || format!("/vocabulary/favorites?dir={}", direction.get())} attr:class="stage-button favorites-button">
                        "⭐"
                    </A>
                </div>
            </div>
        </div>
    }
}
