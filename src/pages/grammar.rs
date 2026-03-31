use crate::core::LanguageContext;
use crate::data::{get_all_topics, get_difficulty_class, get_ui_strings, load_grammar_content};
use leptos::prelude::*;
use leptos_router::components::A;

/// Grammar learning page
#[component]
pub fn Grammar() -> impl IntoView {
    let lang_ctx = expect_context::<LanguageContext>();
    let ui = move || get_ui_strings(lang_ctx.language.get());
    let lang = move || match lang_ctx.language.get() {
        crate::core::Language::German => "de",
        _ => "en",
    };
    let topics = move || get_all_topics();

    view! {
        <div class="page-container">
            <header class="page-header">
                <A href="/" attr:class="back-button">"❮"</A>
                <h1>{move || ui().grammar}</h1>
            </header>

            <div class="grammar-topics-container">
                {move || topics().into_iter().map(|topic| {
                    let difficulty_class = get_difficulty_class(topic.difficulty);
                    let name = load_grammar_content(topic.id, lang())
                        .map(|c| c.topic.name)
                        .unwrap_or_default();
                    view! {
                        <A
                            href=format!("/grammar/{}", topic.id)
                            attr:class=format!("grammar-topic-row {}", difficulty_class)
                        >
                            <span class="topic-number">{topic.id}</span>
                            <span class="topic-name">{name}</span>
                            <span class="topic-arrow">"›"</span>
                        </A>
                    }
                }).collect_view()}
            </div>
        </div>
    }
}
