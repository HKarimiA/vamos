use leptos::prelude::*;
use leptos_router::components::A;

/// Vocabulary learning page
#[component]
pub fn Vocabulary() -> impl IntoView {
    view! {
        <div class="page-container">
            <header class="page-header">
                <A href="/" attr:class="back-button">"← Back"</A>
                <h1>"Vocabulary"</h1>
            </header>

            <div class="content">
                <p>"Vocabulary exercises coming soon..."</p>
            </div>
        </div>
    }
}
