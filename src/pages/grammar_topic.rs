use crate::data::{GrammarContent, get_all_topics, load_grammar_content};
use leptos::prelude::*;
use leptos_router::{components::A, hooks::use_params_map};
use rand::SeedableRng;
use rand::rngs::SmallRng;
use rand::seq::SliceRandom;

/// Grammar topic detail page with quiz interface
#[component]
pub fn GrammarTopic() -> impl IntoView {
    let params = use_params_map();

    // Get topic metadata
    let topic = move || {
        params
            .read()
            .get("id")
            .and_then(|id| id.parse::<u32>().ok())
            .and_then(|id| get_all_topics().into_iter().find(|t| t.id == id))
    };

    // Load grammar content
    let content = move || -> Result<GrammarContent, String> {
        params
            .read()
            .get("id")
            .and_then(|id| id.parse::<u32>().ok())
            .ok_or_else(|| "Invalid topic ID".to_string())
            .and_then(load_grammar_content)
    };

    view! {
        <div class="page-container">
            {move || match (topic(), content()) {
                (Some(t), Ok(c)) => view! {
                    <QuizInterface topic=t content=c/>
                }
                .into_any(),
                (Some(t), Err(e)) => view! {
                    <header class="page-header">
                        <A href="/grammar" attr:class="back-button">"❮"</A>
                        <h1>{format!("{}. {}", t.id, t.name)}</h1>
                    </header>

                    <div class="content" style="padding: 2rem 1.5rem;">
                        <div style="text-align: center; padding: 3rem 1rem;">
                            <p style="font-size: 3rem; margin-bottom: 1rem;">"📚"</p>
                            <p style="font-size: 1.25rem; color: #666; margin-bottom: 0.5rem;">
                                "Content coming soon!"
                            </p>
                            <p style="color: #999;">{e}</p>
                        </div>
                    </div>
                }
                .into_any(),
                _ => view! {
                    <header class="page-header">
                        <A href="/grammar" attr:class="back-button">"❮"</A>
                        <h1>"Grammar"</h1>
                    </header>

                    <div class="error-message">
                        <p>"Topic not found"</p>
                        <A href="/grammar" attr:class="back-button">"Back to Grammar"</A>
                    </div>
                }
                .into_any(),
            }}
        </div>
    }
}

/// Quiz interface component with questions and answers
#[component]
fn QuizInterface(topic: crate::data::GrammarTopic, content: GrammarContent) -> impl IntoView {
    // Store content and topic for use in closures
    let content = StoredValue::new(content);
    let topic = StoredValue::new(topic);
    let total_questions = content.get_value().questions.len();

    // Quiz state
    let current_question_idx = RwSignal::new(0usize);
    let score = RwSignal::new(0u32);
    let selected_answer = RwSignal::new(None::<usize>);
    let show_feedback = RwSignal::new(false);
    let quiz_completed = RwSignal::new(false);

    // Modal states
    let show_topic_explanation = RwSignal::new(false);
    let show_question_hint = RwSignal::new(false);

    // Shuffled answer indices for current question
    let shuffled_answer_indices = RwSignal::new(Vec::<usize>::new());

    // Shuffle answers when question changes
    Effect::new(move |_| {
        let idx = current_question_idx.get();
        let questions = &content.get_value().questions;

        if let Some(question) = questions.get(idx) {
            let mut indices: Vec<usize> = (0..question.answers.len()).collect();
            let mut rng = SmallRng::from_entropy();
            indices.shuffle(&mut rng);
            shuffled_answer_indices.set(indices);
        }
    });

    // Handle answer selection
    let select_answer = move |display_idx: usize| {
        if show_feedback.get() {
            return;
        }

        selected_answer.set(Some(display_idx));
        show_feedback.set(true);

        // Check if correct and update score
        let q_idx = current_question_idx.get();
        let questions = &content.get_value().questions;

        if let Some(original_idx) = shuffled_answer_indices.get().get(display_idx)
            && questions[q_idx].answers[*original_idx].correct
        {
            score.update(|s| *s += 1);
        }
    };

    // Move to next question or complete quiz
    let next_question = move |_| {
        show_feedback.set(false);
        selected_answer.set(None);
        show_question_hint.set(false);

        if current_question_idx.get() + 1 < total_questions {
            current_question_idx.update(|idx| *idx += 1);
        } else {
            quiz_completed.set(true);
        }
    };

    // Restart quiz
    let restart_quiz = move |_| {
        current_question_idx.set(0);
        score.set(0);
        selected_answer.set(None);
        show_feedback.set(false);
        quiz_completed.set(false);
        show_question_hint.set(false);
    };

    view! {
        <>
            <Show
                when=move || !quiz_completed.get()
                fallback=move || {
                    view! {
                        <ResultsScreen
                            topic=topic.get_value()
                            score=score.get()
                            total=total_questions as u32
                            on_restart=restart_quiz
                        />
                    }
                }
            >

                <header class="page-header">
                    <A href="/grammar" attr:class="back-button">"❮"</A>
                    <h1>{move || format!("{}. {}", topic.get_value().id, topic.get_value().name)}</h1>
                    <button
                        class="hint-button-header"
                        on:click=move |_| show_topic_explanation.set(true)
                    >
                        "💡"
                    </button>
                </header>

                <div class="quiz-container">
                    // Progress bar
                    <div class="quiz-progress">
                        <div class="progress-text">
                            {move || {
                                format!(
                                    "Question {} of {}",
                                    current_question_idx.get() + 1,
                                    total_questions,
                                )
                            }}

                        </div>
                        <div class="progress-bar">
                            <div
                                class="progress-fill"
                                style:width=move || {
                                    format!(
                                        "{}%",
                                        ((current_question_idx.get() + 1) as f32 / total_questions as f32
                                            * 100.0) as u32,
                                    )
                                }
                            ></div>
                        </div>
                    </div>

                    // Question and answers
                    <div class="question-card">
                        <h2 class="question-text">
                            {move || {
                                content.get_value().questions[current_question_idx.get()].question.clone()
                            }}

                        </h2>

                        <div class="answers-grid">
                            {move || {
                                let q_idx = current_question_idx.get();
                                let questions = &content.get_value().questions;
                                let indices = shuffled_answer_indices.get();

                                indices
                                    .iter()
                                    .enumerate()
                                    .map(|(display_idx, &original_idx)| {
                                        let answer_text = questions[q_idx].answers[original_idx].text.clone();
                                        let is_correct = questions[q_idx].answers[original_idx].correct;

                                        view! {
                                            <button
                                                class="answer-button"
                                                class:selected=move || selected_answer.get() == Some(display_idx)
                                                class:correct=move || {
                                                    show_feedback.get()
                                                        && selected_answer.get() == Some(display_idx)
                                                        && is_correct
                                                }
                                                class:incorrect=move || {
                                                    show_feedback.get()
                                                        && selected_answer.get() == Some(display_idx)
                                                        && !is_correct
                                                }
                                                disabled=move || show_feedback.get()
                                                on:click=move |_| select_answer(display_idx)
                                            >
                                                {answer_text}
                                            </button>
                                        }
                                    })
                                    .collect_view()
                            }}


                        </div>

                        // Question hint button
                        <button
                            class="hint-button-question"
                            on:click=move |_| show_question_hint.set(true)
                        >
                            "💡 Hint"
                        </button>

                        // Feedback and next button
                        <Show when=move || show_feedback.get()>
                            <div class="feedback-container">
                                {move || {
                                    selected_answer.get().map(|display_idx| {
                                        let q_idx = current_question_idx.get();
                                        let indices = shuffled_answer_indices.get();
                                        let questions = &content.get_value().questions;

                                        if let Some(&original_idx) = indices.get(display_idx) {
                                            let is_correct = questions[q_idx].answers[original_idx].correct;

                                            if is_correct {
                                                view! { <p class="feedback correct">"✓ Correct!"</p> }.into_any()
                                            } else {
                                                view! { <p class="feedback incorrect">"✗ Incorrect"</p> }.into_any()
                                            }
                                        } else {
                                            view! { <p></p> }.into_any()
                                        }
                                    }).unwrap_or_else(|| view! { <p></p> }.into_any())
                                }}

                                <button class="next-button" on:click=next_question>
                                    {move || {
                                        if current_question_idx.get() + 1 < total_questions {
                                            "Next Question →"
                                        } else {
                                            "See Results →"
                                        }
                                    }}
                                </button>
                            </div>
                        </Show>
                    </div>

                </div>

            </Show>

            // Topic explanation modal
            <Show when=move || show_topic_explanation.get()>
                <div class="modal-overlay" on:click=move |_| show_topic_explanation.set(false)>
                    <div class="modal-content" on:click=|e| e.stop_propagation()>
                        <button
                            class="modal-close"
                            on:click=move |_| show_topic_explanation.set(false)
                        >
                            "×"
                        </button>
                        <h2>{content.get_value().explanation.title.clone()}</h2>
                        {content
                            .get_value()
                            .explanation
                            .sections
                            .iter()
                            .map(|section| {
                                view! {
                                    <div class="explanation-section">
                                        <h3>{section.subtitle.clone()}</h3>
                                        <p>{section.content.clone()}</p>
                                    </div>
                                }
                            })
                            .collect_view()}
                    </div>
                </div>
            </Show>

            // Question hint modal
            <Show when=move || show_question_hint.get()>
                <div class="modal-overlay" on:click=move |_| show_question_hint.set(false)>
                    <div class="modal-content" on:click=|e| e.stop_propagation()>
                        <button
                            class="modal-close"
                            on:click=move |_| show_question_hint.set(false)
                        >
                            "×"
                        </button>
                        <h2>"Hint"</h2>
                        <p class="hint-text">
                            {move || {
                                content.get_value().questions[current_question_idx.get()].hint.clone()
                            }}

                        </p>
                    </div>
                </div>
            </Show>
        </>
    }
}

/// Results screen shown after completing the quiz
#[component]
fn ResultsScreen<F>(
    topic: crate::data::GrammarTopic,
    score: u32,
    total: u32,
    on_restart: F,
) -> impl IntoView
where
    F: Fn(leptos::ev::MouseEvent) + 'static,
{
    let percentage = (score as f32 / total as f32 * 100.0) as u32;
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
            <A href="/grammar" attr:class="back-button">"❮"</A>
            <h1>{format!("{}. {}", topic.id, topic.name)}</h1>
        </header>

        <div class="results-container">
            <div class="results-card">
                <p class="results-emoji">{emoji}</p>
                <h2>"Quiz Complete!"</h2>
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
                            "Excellent work! You've mastered this topic."
                        } else if percentage >= 70 {
                            "Good job! You understand most of the material."
                        } else if percentage >= 50 {
                            "Not bad! Review the explanations and try again."
                        } else {
                            "Keep practicing! Review the topic explanation."
                        }
                    }}

                </p>
                <div class="results-buttons">
                    <button class="restart-button" on:click=on_restart>
                        "Try Again"
                    </button>
                    <A href="/grammar" attr:class="back-to-topics-button">
                        "Back to Topics"
                    </A>
                </div>
            </div>
        </div>
    }
}
