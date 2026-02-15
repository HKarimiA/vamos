use crate::data::{get_all_topics, get_difficulty_class};
use leptos::prelude::*;
use leptos_router::components::A;

/// Grammar learning page
#[component]
pub fn Grammar() -> impl IntoView {
    let topics = get_all_topics();

    view! {
        <div class="page-container">
            <header class="page-header">
                <A href="/" attr:class="back-button">"❮"</A>
                <h1>"Grammar"</h1>
            </header>

            <div class="grammar-topics-container">
                {topics.into_iter().map(|topic| {
                    let difficulty_class = get_difficulty_class(topic.difficulty);
                    view! {
                        <A 
                            href=format!("/grammar/{}", topic.id)
                            attr:class=format!("grammar-topic-row {}", difficulty_class)
                        >
                            <span class="topic-number">{topic.id}</span>
                            <span class="topic-name">{topic.name}</span>
                            <span class="topic-arrow">"›"</span>
                        </A>
                    }
                }).collect_view()}
            </div>
        </div>
    }
}
