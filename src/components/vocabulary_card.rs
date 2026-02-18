use crate::data::LearningDirection;
use leptos::prelude::*;
use std::rc::Rc;

const SWIPE_THRESHOLD: f64 = 32.0;
const SWIPE_HORIZONTAL_RATIO: f64 = 0.75;
const MAX_DRAG_OFFSET: f64 = 80.0;
const MAX_ROTATE_DEG: f64 = 2.0;

/// Shared vocabulary card component
#[component]
pub fn VocabularyCard<FToggle, FPrev, FNext>(
    source_word: String,
    source_example: String,
    target_word: String,
    target_example: String,
    card_index: usize,
    card_count: usize,
    is_favorite: bool,
    direction: LearningDirection,
    show_example: RwSignal<bool>,
    show_translation: RwSignal<bool>,
    #[prop(optional)] stage: Option<u32>,
    on_toggle_favorite: FToggle,
    on_prev: FPrev,
    on_next: FNext,
) -> impl IntoView
where
    FToggle: Fn() + 'static,
    FPrev: Fn() + 'static,
    FNext: Fn() + 'static,
{
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

    let target_lang = match direction {
        LearningDirection::SpanishToEnglish => "en-US",
        LearningDirection::EnglishToSpanish => "es-ES",
    };

    let source_word_clone = source_word.clone();
    let source_example_clone = source_example.clone();
    let target_word_clone = target_word.clone();
    let target_example_clone = target_example.clone();

    let on_prev = Rc::new(on_prev);
    let on_next = Rc::new(on_next);

    let nav_direction = RwSignal::new(None::<&'static str>);

    let pointer_start_x = RwSignal::new(None::<f64>);
    let pointer_start_y = RwSignal::new(None::<f64>);
    let is_dragging = RwSignal::new(false);
    let drag_offset_x = RwSignal::new(0.0f64);

    let reset_drag_state = Rc::new(move || {
        pointer_start_x.set(None);
        pointer_start_y.set(None);
        is_dragging.set(false);
        drag_offset_x.set(0.0);
    });

    let on_pointer_down = move |ev: leptos::ev::PointerEvent| {
        pointer_start_x.set(Some(ev.client_x() as f64));
        pointer_start_y.set(Some(ev.client_y() as f64));
        is_dragging.set(true);
    };

    let on_pointer_move = move |ev: leptos::ev::PointerEvent| {
        if !is_dragging.get_untracked() {
            return;
        }

        let Some(start_x) = pointer_start_x.get_untracked() else {
            return;
        };
        let Some(start_y) = pointer_start_y.get_untracked() else {
            return;
        };

        let delta_x = ev.client_x() as f64 - start_x;
        let delta_y = ev.client_y() as f64 - start_y;

        if delta_x.abs() > delta_y.abs() * SWIPE_HORIZONTAL_RATIO {
            drag_offset_x.set(delta_x.clamp(-MAX_DRAG_OFFSET, MAX_DRAG_OFFSET));
        } else {
            drag_offset_x.set(0.0);
        }
    };

    let on_pointer_up = {
        let on_next = on_next.clone();
        let on_prev = on_prev.clone();
        let reset_drag_state = reset_drag_state.clone();
        move |ev: leptos::ev::PointerEvent| {
            let (Some(start_x), Some(start_y)) = (
                pointer_start_x.get_untracked(),
                pointer_start_y.get_untracked(),
            ) else {
                reset_drag_state();
                return;
            };

            let delta_x = ev.client_x() as f64 - start_x;
            let delta_y = ev.client_y() as f64 - start_y;

            if delta_x.abs() > SWIPE_THRESHOLD
                && delta_x.abs() > delta_y.abs() * SWIPE_HORIZONTAL_RATIO
            {
                if delta_x < 0.0 {
                    nav_direction.set(Some("next"));
                    on_next();
                } else {
                    nav_direction.set(Some("prev"));
                    on_prev();
                }
            }

            reset_drag_state();
        }
    };

    let on_pointer_cancel = {
        let reset_drag_state = reset_drag_state.clone();
        move |_| reset_drag_state()
    };

    let on_pointer_leave = {
        let reset_drag_state = reset_drag_state.clone();
        move |_| reset_drag_state()
    };

    view! {
        <div class={move || {
            let pulse = if card_index.is_multiple_of(2) {
                "card-anim-a"
            } else {
                "card-anim-b"
            };

            match nav_direction.get() {
                Some("next") => format!("card-animator card-enter-next {pulse}"),
                Some("prev") => format!("card-animator card-enter-prev {pulse}"),
                _ => "card-animator".to_string(),
            }
        }}>
        <div
            class="vocabulary-card"
            on:pointerdown=on_pointer_down
            on:pointermove=on_pointer_move
            on:pointerup=on_pointer_up
            on:pointercancel=on_pointer_cancel
            on:pointerleave=on_pointer_leave
            style={move || {
                let offset = drag_offset_x.get();
                let rotate = (offset / 40.0).clamp(-MAX_ROTATE_DEG, MAX_ROTATE_DEG);
                let transition = if is_dragging.get() {
                    "none"
                } else {
                    "transform 220ms cubic-bezier(0.22, 1, 0.36, 1)"
                };
                format!("transform: translateX({offset}px) rotate({rotate}deg); transition: {transition};")
            }}
        >
            <div class={move || {
                match nav_direction.get() {
                    Some(_) => {
                        if card_index.is_multiple_of(2) {
                            "card-content card-content-reveal-a".to_string()
                        } else {
                            "card-content card-content-reveal-b".to_string()
                        }
                    }
                    None => "card-content".to_string(),
                }
            }}>
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
                    on:click=move |_| show_example.set(true)
                >
                    "Show Example"
                </button>
            })}

            {move || show_example.get().then(|| {
                let example_audio = source_example_clone.clone();
                view! {
                    <div class="card-example" style="display: flex; align-items: center; gap: 0.5rem; margin: 0;">
                        <p style="margin: 0; flex: 1;">{source_example.clone()}</p>
                        <button
                            class="audio-button-small"
                            on:click=move |_| speak(example_audio.clone(), source_lang)
                        >
                            "🔉"
                        </button>
                    </div>
                }
            })}

            {move || (!show_translation.get()).then(|| view! {
                <button
                    class="reveal-button translation-button"
                    on:click=move |_| show_translation.set(true)
                >
                    "Show Translation"
                </button>
            })}

            {move || show_translation.get().then(|| {
                let word_audio = target_word_clone.clone();
                let example_audio = target_example_clone.clone();
                view! {
                    <div class="card-translation">
                        <div style="display: flex; align-items: center; gap: 0.5rem;">
                            <p class="translation-word" style="margin: 0; flex: 1;">{target_word.clone()}</p>
                            <button
                                class="audio-button-small"
                                on:click=move |_| speak(word_audio.clone(), target_lang)
                            >
                                "🔉"
                            </button>
                        </div>
                        <div style="display: flex; align-items: center; gap: 0.5rem; margin-top: 1.0rem;">
                            <p class="translation-example" style="margin: 0; flex: 1;">{target_example.clone()}</p>
                            <button
                                class="audio-button-small"
                                on:click=move |_| speak(example_audio.clone(), target_lang)
                            >
                                "🔉"
                            </button>
                        </div>
                    </div>
                }
            })}

            <div class="card-navigation">
                <button
                    class="nav-btn"
                    on:click={
                        let on_prev = on_prev.clone();
                        move |_| {
                            nav_direction.set(Some("prev"));
                            on_prev();
                        }
                    }
                    disabled={move || card_index == 0}
                >
                    "← Previous"
                </button>
                <button
                    class="nav-btn"
                    on:click={
                        let on_next = on_next.clone();
                        move |_| {
                            nav_direction.set(Some("next"));
                            on_next();
                        }
                    }
                    disabled={move || card_index + 1 >= card_count}
                >
                    "Next →"
                </button>
            </div>
            </div>
        </div>
        </div>
    }
}
