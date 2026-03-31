use crate::core::{Language, LanguageContext};
use crate::data::get_ui_strings;
use leptos::prelude::*;
use leptos_router::components::A;

/// Settings page — language selection
#[component]
pub fn Settings() -> impl IntoView {
    let lang_ctx = expect_context::<LanguageContext>();

    let ui = move || get_ui_strings(lang_ctx.language.get());

    view! {
        <div class="page-container">
            <header class="page-header">
                <A href="/" attr:class="back-button">"❮"</A>
                <h1>{move || ui().settings}</h1>
            </header>

            <div class="settings-content">
                <div class="settings-section">
                    <p class="settings-label">{move || ui().language_label}</p>
                    <div class="language-selector">
                        <button
                            class="language-flag-btn"
                            class:active=move || lang_ctx.language.get() == Language::English
                            on:click=move |_| lang_ctx.set_language(Language::English)
                        >
                            "🇬🇧"
                        </button>
                        <button
                            class="language-flag-btn"
                            class:active=move || lang_ctx.language.get() == Language::German
                            on:click=move |_| lang_ctx.set_language(Language::German)
                        >
                            "🇩🇪"
                        </button>
                    </div>
                </div>
            </div>
        </div>
    }
}
