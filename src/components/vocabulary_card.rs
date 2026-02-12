use crate::data::LearningDirection;
use leptos::prelude::*;

/// Shared vocabulary card component
#[component]
pub fn VocabularyCard<F>(
    source_word: String,
    source_example: String,
    target_word: String,
    target_example: String,
    card_index: usize,
    card_count: usize,
    is_favorite: bool,
    direction: LearningDirection,
    #[prop(optional)] stage: Option<u32>,
    on_toggle_favorite: F,
) -> impl IntoView
where
    F: Fn() + 'static,
{
    // State management
    let (show_example, set_show_example) = signal(false);
    let (show_translation, set_show_translation) = signal(false);

    // Reset state when card changes
    Effect::new(move |_| {
        let _ = card_index;
        set_show_example.set(false);
        set_show_translation.set(false);
    });

    // Speak word using Web Speech API
    #[allow(unused_variables)]
    let speak = move |text: String, lang: &str| {
        #[allow(unused_variables)]
        let lang = lang.to_string();
        #[cfg(target_arch = "wasm32")]
        {
            use wasm_bindgen::prelude::*;
            #[wasm_bindgen]
            extern "C" {
                #[wasm_bindgen(js_namespace = window)]
                fn speak_text(text: &str, lang: &str);
            }
            speak_text(&text, &lang);
        }
    };

    let source_lang = match direction {
        LearningDirection::SpanishToEnglish => "es-ES",
        LearningDirection::EnglishToSpanish => "en-US",
    };

    let source_word_clone = source_word.clone();

    view! {
        <div class="vocabulary-card">
            <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 1rem;">
                <div class="card-progress">
                    {move || {
                        if let Some(s) = stage {
                            format!("{} / {} (Stage {})", card_index + 1, card_count, s)
                        } else {
                            format!("{} / {}", card_index + 1, card_count)
                        }
                    }}
                </div>
                <div class="card-actions" style="display: flex; gap: 0.5rem;">
                    <button
                        class="audio-button"
                        style="font-size: 1.2rem; padding: 0.3rem 0.6rem;"
                        on:click=move |_| speak(source_word_clone.clone(), source_lang)
                    >
                        "🔊"
                    </button>
                    <button
                        class=move || if is_favorite { "favorite-button favorite-active" } else { "favorite-button" }
                        style="font-size: 1.2rem; padding: 0.3rem 0.6rem;"
                        on:click=move |_| on_toggle_favorite()
                    >
                        {move || if is_favorite { "⭐" } else { "☆" }}
                    </button>
                </div>
            </div>
            <div class="card-main">
                <h2 class="card-word">{source_word}</h2>
            </div>

            {move || (!show_example.get()).then(|| view! {
                <button
                    class="reveal-button"
                    on:click=move |_| set_show_example.set(true)
                >
                    "Show Example"
                </button>
            })}

            {move || show_example.get().then(|| view! {
                <p class="card-example">{source_example.clone()}</p>
            })}

            {move || (!show_translation.get()).then(|| view! {
                <button
                    class="reveal-button translation-button"
                    on:click=move |_| set_show_translation.set(true)
                >
                    "Show Translation"
                </button>
            })}

            {move || show_translation.get().then(|| view! {
                <div class="card-translation">
                    <p class="translation-word">{target_word.clone()}</p>
                    <p class="translation-example">{target_example.clone()}</p>
                </div>
            })}
        </div>
    }
}
