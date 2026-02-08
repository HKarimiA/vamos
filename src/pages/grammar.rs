use leptos::prelude::*;
use leptos_router::components::A;

/// Grammar learning page
#[component]
pub fn Grammar() -> impl IntoView {
    view! {
        <div class="page-container">
            <header class="page-header">
                <A href="/" attr:class="back-button">"← Back"</A>
                <h1>"Grammar"</h1>
            </header>

            <div class="content">
                <p>"Grammar exercises coming soon..."</p>
            </div>
        </div>
    }
}
