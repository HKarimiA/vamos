use leptos::prelude::*;
use leptos_router::components::A;

/// Home page with two main navigation buttons
#[component]
pub fn Home() -> impl IntoView {
    view! {
        <div class="home-container">
            <header class="home-header">
                <img src="/vamos-icon.png" alt="Vamos!" style="max-width: 300px; height: auto; margin-bottom: 1rem;" />
                <p class="subtitle">"Learn Spanish"</p>
            </header>

            <div class="button-container">
                <A href="/vocabulary" attr:class="nav-button">
                    <div class="button-icon">"📚"</div>
                    <div class="button-text">"Vocabulary"</div>
                </A>

                <A href="/grammar" attr:class="nav-button">
                    <div class="button-icon">"✏️"</div>
                    <div class="button-text">"Grammar"</div>
                </A>
            </div>
        </div>
    }
}
