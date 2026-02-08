use leptos::prelude::*;
use leptos_router::components::A;

/// Vocabulary learning page - Shows stage selection grid
#[component]
pub fn Vocabulary() -> impl IntoView {
    // State for learning direction
    let (direction, set_direction) = signal("es-to-en".to_string());

    // Toggle direction handler
    let toggle_direction = move |_| {
        set_direction.update(|d| {
            *d = if d == "es-to-en" {
                "en-to-es".to_string()
            } else {
                "es-to-en".to_string()
            };
        });
    };

    view! {
        <div class="page-container">
            <header class="page-header">
                <A href="/" attr:class="back-button">"← Back"</A>
                <h1>"Vocabulary"</h1>
            </header>

            <div class="vocab-content">
                <div class="vocab-controls">
                    <button
                        class="direction-toggle"
                        on:click=toggle_direction
                    >
                        {move || if direction.get() == "es-to-en" {
                            "🇪🇸 → 🇬🇧"
                        } else {
                            "🇬🇧 → 🇪🇸"
                        }}
                    </button>
                </div>

                <div class="stage-grid">
                    {(1..=20).map(|stage| {
                        let href = format!("/vocabulary/{}", stage);
                        view! {
                            <A href=href attr:class="stage-button">
                                {stage.to_string()}
                            </A>
                        }
                    }).collect::<Vec<_>>()}

                    <A href="/vocabulary/favorites" attr:class="stage-button favorites-button">
                        "⭐"
                    </A>
                </div>
            </div>
        </div>
    }
}
