use crate::data::get_all_topics;
use leptos::prelude::*;
use leptos_router::{components::A, hooks::use_params_map};

/// Grammar topic detail page (placeholder for lessons)
#[component]
pub fn GrammarTopic() -> impl IntoView {
    let params = use_params_map();

    let topic = move || {
        params
            .read()
            .get("id")
            .and_then(|id| id.parse::<u32>().ok())
            .and_then(|id| {
                get_all_topics()
                    .into_iter()
                    .find(|t| t.id == id)
            })
    };

    view! {
        <div class="page-container">
            {move || match topic() {
                Some(t) => view! {
                    <header class="page-header">
                        <A href="/grammar" attr:class="back-button">"❮"</A>
                        <h1>{format!("{}. {}", t.id, t.name)}</h1>
                    </header>

                    <div class="content" style="padding: 2rem 1.5rem;">
                        <div style="text-align: center; padding: 3rem 1rem;">
                            <p style="font-size: 3rem; margin-bottom: 1rem;">"📚"</p>
                            <p style="font-size: 1.25rem; color: #666; margin-bottom: 0.5rem;">
                                "Lessons coming soon!"
                            </p>
                            <p style="color: #999;">
                                "This grammar topic will be available in a future update."
                            </p>
                        </div>
                    </div>
                }.into_any(),
                None => view! {
                    <header class="page-header">
                        <A href="/grammar" attr:class="back-button">"❮"</A>
                        <h1>"Grammar"</h1>
                    </header>

                    <div class="error-message">
                        <p>"Topic not found"</p>
                        <A href="/grammar" attr:class="back-button">"Back to Grammar"</A>
                    </div>
                }.into_any()
            }}
        </div>
    }
}
