use crate::core::{Language, LanguageContext};
use crate::data::{QuizQuestion, QuizSource, build_quiz_session, get_ui_strings, normalize_answer};
use leptos::prelude::*;
use leptos_router::components::A;

/// Dynamic quiz page: 20 mixed questions from Vocabulary, Expressions and Grammar
#[component]
pub fn Quiz() -> impl IntoView {
    let lang_ctx = expect_context::<LanguageContext>();
    let ui = move || get_ui_strings(lang_ctx.language.get());
    let ui_lang = move || match lang_ctx.language.get() {
        Language::German => "de",
        _ => "en",
    };

    let questions = RwSignal::new(Vec::<QuizQuestion>::new());
    let started = RwSignal::new(false);
    let current_idx = RwSignal::new(0usize);
    let score = RwSignal::new(0u32);
    let quiz_completed = RwSignal::new(false);

    let selected_answer = RwSignal::new(None::<usize>);
    let show_feedback = RwSignal::new(false);

    let blank_inputs = RwSignal::new(Vec::<String>::new());
    let blank_checked = RwSignal::new(false);
    let blank_correctness = RwSignal::new(Vec::<bool>::new());

    // Reset the answer inputs whenever the current question changes
    Effect::new(move |_| {
        let idx = current_idx.get();
        let qs = questions.get();
        match qs.get(idx) {
            Some(QuizQuestion::FillInTheBlank { blanks, .. }) => {
                blank_inputs.set(vec![String::new(); blanks.len()]);
            }
            _ => blank_inputs.set(Vec::new()),
        }
    });

    let start_new_session = move || {
        questions.set(build_quiz_session(ui_lang()));
        current_idx.set(0);
        score.set(0);
        quiz_completed.set(false);
        selected_answer.set(None);
        show_feedback.set(false);
        blank_checked.set(false);
        blank_correctness.set(Vec::new());
        started.set(true);
    };

    let total_questions = move || questions.get().len();

    let next_question = move |_| {
        selected_answer.set(None);
        show_feedback.set(false);
        blank_checked.set(false);
        blank_correctness.set(Vec::new());

        if current_idx.get() + 1 < total_questions() {
            current_idx.update(|i| *i += 1);
        } else {
            quiz_completed.set(true);
        }
    };

    view! {
        <div class="page-container">
            <Show
                when=move || started.get()
                fallback=move || {
                    view! { <QuizStart on_start=move |_| start_new_session()/> }
                }
            >

                <Show
                    when=move || !quiz_completed.get()
                    fallback=move || {
                        view! {
                            <QuizResults
                                score=score.get()
                                total=total_questions() as u32
                                on_restart=move |_| start_new_session()
                            />
                        }
                    }
                >

                    <header class="page-header">
                        <A href="/" attr:class="back-button">
                            "❮"
                        </A>
                        <h1>{move || ui().quiz}</h1>
                    </header>

                    <div class="quiz-container">
                        <div class="quiz-progress">
                            <div class="progress-text">
                                {move || {
                                    ui().format_question_of(current_idx.get() + 1, total_questions())
                                }}
                            </div>
                            <div class="progress-bar">
                                <div
                                    class="progress-fill"
                                    style:width=move || {
                                        let total = total_questions().max(1);
                                        format!(
                                            "{}%",
                                            ((current_idx.get() + 1) as f32 / total as f32 * 100.0) as u32,
                                        )
                                    }
                                ></div>
                            </div>
                        </div>

                        <div class="question-card">
                            {move || {
                                let q_idx = current_idx.get();
                                match questions.get().get(q_idx).cloned() {
                                    Some(
                                        QuizQuestion::MultipleChoice { source, prompt, options, correct_index },
                                    ) => {
                                        view! {
                                            <MultipleChoiceCard
                                                source=source
                                                prompt=prompt
                                                options=options
                                                correct_index=correct_index
                                                selected_answer=selected_answer
                                                show_feedback=show_feedback
                                                score=score
                                            />
                                        }
                                            .into_any()
                                    }
                                    Some(
                                        QuizQuestion::FillInTheBlank { source, prompt, display_phrase, blanks },
                                    ) => {
                                        view! {
                                            <FillInBlankCard
                                                source=source
                                                prompt=prompt
                                                display_phrase=display_phrase
                                                blanks=blanks
                                                blank_inputs=blank_inputs
                                                blank_checked=blank_checked
                                                blank_correctness=blank_correctness
                                                show_feedback=show_feedback
                                                score=score
                                            />
                                        }
                                            .into_any()
                                    }
                                    None => view! { <p>"..."</p> }.into_any(),
                                }
                            }}

                            <Show when=move || show_feedback.get()>
                                <div class="feedback-container">
                                    <button class="next-button" on:click=next_question>
                                        {move || {
                                            if current_idx.get() + 1 < total_questions() {
                                                ui().next_question
                                            } else {
                                                ui().see_results
                                            }
                                        }}
                                    </button>
                                </div>
                            </Show>
                        </div>
                    </div>
                </Show>
            </Show>
        </div>
    }
}

/// Quiz start / intro screen
#[component]
fn QuizStart<F>(on_start: F) -> impl IntoView
where
    F: Fn(leptos::ev::MouseEvent) + 'static,
{
    let lang_ctx = expect_context::<LanguageContext>();
    let ui = move || get_ui_strings(lang_ctx.language.get());

    view! {
        <header class="page-header">
            <A href="/" attr:class="back-button">
                "❮"
            </A>
            <h1>{move || ui().quiz}</h1>
        </header>
        <div class="results-container">
            <div class="results-card">
                <p class="results-emoji">"🧩"</p>
                <h2>{move || ui().quiz}</h2>
                <p class="results-message">{move || ui().quiz_start_subtitle}</p>
                <div class="results-buttons">
                    <button class="restart-button" on:click=on_start>
                        {move || ui().start_quiz}
                    </button>
                </div>
            </div>
        </div>
    }
}

/// Results screen shown after completing the quiz
#[component]
fn QuizResults<F>(score: u32, total: u32, on_restart: F) -> impl IntoView
where
    F: Fn(leptos::ev::MouseEvent) + 'static,
{
    let lang_ctx = expect_context::<LanguageContext>();
    let ui = move || get_ui_strings(lang_ctx.language.get());

    let total = total.max(1);
    let percentage = (score * 100) / total;
    let emoji = if percentage >= 90 {
        "🎉"
    } else if percentage >= 70 {
        "😊"
    } else if percentage >= 50 {
        "🙂"
    } else {
        "📚"
    };

    view! {
        <header class="page-header">
            <A href="/" attr:class="back-button">
                "❮"
            </A>
            <h1>{move || ui().quiz}</h1>
        </header>

        <div class="results-container">
            <div class="results-card">
                <p class="results-emoji">{emoji}</p>
                <h2>{move || ui().quiz_complete}</h2>
                <div class="results-score">
                    <span class="score-large">
                        {score}
                        "/"
                        {total}
                    </span>
                    <span class="score-percentage">{percentage}"%"</span>
                </div>
                <p class="results-message">
                    {move || {
                        if percentage >= 90 {
                            ui().excellent_work
                        } else if percentage >= 70 {
                            ui().good_job
                        } else if percentage >= 50 {
                            ui().not_bad
                        } else {
                            ui().keep_practicing
                        }
                    }}
                </p>
                <div class="results-buttons">
                    <button class="restart-button" on:click=on_restart>
                        {move || ui().try_again}
                    </button>
                    <A href="/" attr:class="back-to-topics-button">
                        {move || ui().back_to_topics}
                    </A>
                </div>
            </div>
        </div>
    }
}

/// Multiple-choice question card (used by Vocabulary and Grammar questions)
#[component]
fn MultipleChoiceCard(
    source: QuizSource,
    prompt: String,
    options: Vec<String>,
    correct_index: usize,
    selected_answer: RwSignal<Option<usize>>,
    show_feedback: RwSignal<bool>,
    score: RwSignal<u32>,
) -> impl IntoView {
    let lang_ctx = expect_context::<LanguageContext>();
    let ui = move || get_ui_strings(lang_ctx.language.get());

    let label = match source {
        QuizSource::Vocabulary => ui().quiz_vocab_label,
        QuizSource::Expression => ui().quiz_expression_label,
        QuizSource::Grammar => ui().quiz_grammar_label,
    };
    let is_vocab = source == QuizSource::Vocabulary;

    view! {
        <p class="hint-label" style="color:#888; font-size:0.95rem; margin-bottom:0.5rem;">
            {label}
        </p>
        <Show when=move || is_vocab>
            <p class="results-message" style="margin-bottom: 0.25rem;">
                {move || ui().quiz_translate_prompt}
            </p>
        </Show>
        <h2 class="question-text">{prompt}</h2>
        <div class="answers-grid">
            {options
                .into_iter()
                .enumerate()
                .map(|(idx, text)| {
                    view! {
                        <button
                            class="answer-button"
                            class:selected=move || selected_answer.get() == Some(idx)
                            class:correct=move || show_feedback.get() && idx == correct_index
                            class:incorrect=move || {
                                show_feedback.get() && selected_answer.get() == Some(idx)
                                    && idx != correct_index
                            }
                            disabled=move || show_feedback.get()
                            on:click=move |_| {
                                if show_feedback.get() {
                                    return;
                                }
                                selected_answer.set(Some(idx));
                                show_feedback.set(true);
                                if idx == correct_index {
                                    score.update(|s| *s += 1);
                                }
                            }
                        >
                            {text}
                        </button>
                    }
                })
                .collect_view()}
        </div>
    }
}

/// Fill-in-the-blank question card (used by Expression questions)
#[component]
fn FillInBlankCard(
    source: QuizSource,
    prompt: String,
    display_phrase: String,
    blanks: Vec<String>,
    blank_inputs: RwSignal<Vec<String>>,
    blank_checked: RwSignal<bool>,
    blank_correctness: RwSignal<Vec<bool>>,
    show_feedback: RwSignal<bool>,
    score: RwSignal<u32>,
) -> impl IntoView {
    let lang_ctx = expect_context::<LanguageContext>();
    let ui = move || get_ui_strings(lang_ctx.language.get());

    let label = match source {
        QuizSource::Vocabulary => ui().quiz_vocab_label,
        QuizSource::Expression => ui().quiz_expression_label,
        QuizSource::Grammar => ui().quiz_grammar_label,
    };

    let segments: Vec<String> = display_phrase.split("____").map(|s| s.to_string()).collect();
    let blanks = StoredValue::new(blanks);
    let blank_count = blanks.with_value(|b| b.len());

    let check_answer = move |_| {
        if blank_checked.get() {
            return;
        }
        let inputs = blank_inputs.get();
        let mut all_correct = true;
        let mut correctness = Vec::new();
        blanks.with_value(|blanks| {
            for (i, correct) in blanks.iter().enumerate() {
                let user_input = inputs.get(i).cloned().unwrap_or_default();
                let is_correct = normalize_answer(&user_input) == normalize_answer(correct);
                all_correct &= is_correct;
                correctness.push(is_correct);
            }
        });
        blank_correctness.set(correctness);
        if all_correct {
            score.update(|s| *s += 1);
        }
        blank_checked.set(true);
        show_feedback.set(true);
    };

    view! {
        <p class="hint-label" style="color:#888; font-size:0.95rem; margin-bottom:0.5rem;">
            {label}
        </p>
        <p class="results-message" style="margin-bottom: 0.25rem;">
            {move || ui().quiz_complete_phrase}
        </p>
        <h2 class="question-text" style="font-style: italic;">
            {prompt}
        </h2>
        <div class="fill-blank-phrase">
            {segments
                .into_iter()
                .enumerate()
                .map(|(i, segment)| {
                    if i < blank_count {
                        view! {
                            <>
                                <span>{segment}</span>
                                <span class="fill-blank-wrap">
                                    <input
                                        type="text"
                                        class="fill-blank-input"
                                        class:correct=move || {
                                            blank_checked.get()
                                                && blank_correctness.get().get(i).copied().unwrap_or(false)
                                        }
                                        class:incorrect=move || {
                                            blank_checked.get()
                                                && !blank_correctness.get().get(i).copied().unwrap_or(true)
                                        }
                                        disabled=move || blank_checked.get()
                                        prop:value=move || {
                                            blank_inputs.get().get(i).cloned().unwrap_or_default()
                                        }
                                        placeholder=move || ui().type_your_answer
                                        on:input=move |ev| {
                                            let value = event_target_value(&ev);
                                            blank_inputs
                                                .update(|v| {
                                                    if let Some(slot) = v.get_mut(i) {
                                                        *slot = value;
                                                    }
                                                });
                                        }
                                    />
                                    <Show when=move || {
                                        blank_checked.get()
                                            && !blank_correctness.get().get(i).copied().unwrap_or(true)
                                    }>
                                        <span class="fill-blank-correct-answer">
                                            {move || ui().correct_answer_label} " "
                                            {move || blanks.with_value(|b| b.get(i).cloned().unwrap_or_default())}
                                        </span>
                                    </Show>
                                </span>
                            </>
                        }
                            .into_any()
                    } else {
                        view! { <><span>{segment}</span></> }.into_any()
                    }
                })
                .collect_view()}
        </div>
        <Show when=move || !blank_checked.get()>
            <button class="hint-button-question" on:click=check_answer>
                {move || ui().check_answer}
            </button>
        </Show>
    }
}
