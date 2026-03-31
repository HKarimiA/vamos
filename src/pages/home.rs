use crate::core::LanguageContext;
use crate::data::get_ui_strings;
use leptos::prelude::*;
use leptos_router::components::A;

/// Home page with two main navigation buttons
#[component]
pub fn Home() -> impl IntoView {
    let lang_ctx = expect_context::<LanguageContext>();
    let ui = move || get_ui_strings(lang_ctx.language.get());

    view! {
        <div class="home-container">
            <header class="home-header">
                <img src="/vamos-icon.png" alt="Vamos!" style="max-width: 300px; height: auto; margin-bottom: 1rem;" />
            </header>

            <A href="/settings" attr:class="settings-button">"⚙️"</A>

            <div class="button-container">
                <A href="/vocabulary" attr:class="nav-button">
                    <div class="button-icon">"📚"</div>
                    <div class="button-text">{move || ui().vocabulary}</div>
                </A>

                <A href="/grammar" attr:class="nav-button">
                    <div class="button-icon">"✏️"</div>
                    <div class="button-text">{move || ui().grammar}</div>
                </A>
            </div>
        </div>
    }
}
