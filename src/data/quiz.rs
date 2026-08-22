use crate::data::expressions::load_expression_unit;
use crate::data::grammar::load_grammar_content;
use crate::data::vocabulary::load_vocabulary_stage;
use rand::Rng;
use rand::SeedableRng;
use rand::rngs::SmallRng;
use rand::seq::SliceRandom;
use std::collections::HashSet;

const VOCAB_STAGE_COUNT: u32 = 33;
const EXPRESSION_UNIT_COUNT: u32 = 27;
const GRAMMAR_TOPIC_COUNT: u32 = 27;

const VOCAB_QUESTION_COUNT: usize = 5;
const EXPRESSION_QUESTION_COUNT: usize = 8;
const GRAMMAR_QUESTION_COUNT: usize = 7;

/// Which feature area a quiz question was generated from
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum QuizSource {
    Vocabulary,
    Expression,
    Grammar,
}

/// A single dynamically generated quiz question
#[derive(Debug, Clone)]
pub enum QuizQuestion {
    MultipleChoice {
        source: QuizSource,
        prompt: String,
        options: Vec<String>,
        correct_index: usize,
    },
    FillInTheBlank {
        source: QuizSource,
        /// Native-language (en/de) translation shown to the user
        prompt: String,
        /// Spanish phrase with blanked words replaced by "____"
        display_phrase: String,
        /// Correct word(s) for each blank, left to right
        blanks: Vec<String>,
    },
}

/// Build a brand-new set of 20 quiz questions (5 vocabulary, 8 expressions, 7 grammar),
/// combined and shuffled. `ui_lang` is "en" or "de" and selects the non-Spanish side.
pub fn build_quiz_session(ui_lang: &str) -> Vec<QuizQuestion> {
    let mut rng = SmallRng::from_entropy();

    let mut questions = Vec::with_capacity(20);
    questions.extend(build_vocabulary_questions(ui_lang, &mut rng));
    questions.extend(build_expression_questions(ui_lang, &mut rng));
    questions.extend(build_grammar_questions(ui_lang, &mut rng));

    questions.shuffle(&mut rng);
    questions
}

/// Normalize a typed answer for lenient comparison: lowercased, punctuation stripped,
/// acute-accented vowels folded to their plain form. `ñ` is intentionally NOT folded to
/// `n` since it represents a distinct letter/sound in Spanish (e.g. "año" vs "ano").
pub fn normalize_answer(input: &str) -> String {
    input
        .chars()
        .filter(|c| c.is_alphanumeric())
        .flat_map(|c| {
            let folded = match c {
                'á' | 'Á' => 'a',
                'é' | 'É' => 'e',
                'í' | 'Í' => 'i',
                'ó' | 'Ó' => 'o',
                'ú' | 'Ú' | 'ü' | 'Ü' => 'u',
                'ñ' | 'Ñ' => 'ñ',
                other => other,
            };
            folded.to_lowercase()
        })
        .collect()
}

/// Load every vocabulary card across all stages as (spanish word, translated word) pairs
fn load_all_vocabulary_pairs(ui_lang: &str) -> Vec<(String, String)> {
    let mut pairs = Vec::new();
    for stage in 1..=VOCAB_STAGE_COUNT {
        if let (Ok(es_cards), Ok(native_cards)) = (
            load_vocabulary_stage(stage, "es"),
            load_vocabulary_stage(stage, ui_lang),
        ) {
            for (es, native) in es_cards.iter().zip(native_cards.iter()) {
                pairs.push((es.word.clone(), native.word.clone()));
            }
        }
    }
    pairs
}

fn build_vocabulary_questions(ui_lang: &str, rng: &mut SmallRng) -> Vec<QuizQuestion> {
    let mut pool = load_all_vocabulary_pairs(ui_lang);
    pool.shuffle(rng);

    let take = VOCAB_QUESTION_COUNT.min(pool.len());
    let mut questions = Vec::with_capacity(take);

    for i in 0..take {
        let (es_word, correct_translation) = &pool[i];
        let correct_len = correct_translation.chars().count() as i32;

        // Collect distractor candidates distinct from the correct translation, ranked by
        // how close in length they are to the correct answer (a cheap "plausibility"
        // proxy) with a random tiebreaker so equally-similar candidates aren't always
        // picked in the same order. This keeps options varying question-to-question
        // instead of always surfacing whichever words happen to sit first in the pool.
        let mut seen: HashSet<String> = HashSet::new();
        seen.insert(correct_translation.clone());
        let mut candidates: Vec<(i32, u32, String)> = Vec::new();
        for (idx, (_, native)) in pool.iter().enumerate() {
            if idx == i {
                continue;
            }
            if seen.insert(native.clone()) {
                let len_diff = (native.chars().count() as i32 - correct_len).abs();
                candidates.push((len_diff, rng.r#gen::<u32>(), native.clone()));
            }
        }
        candidates.sort_by_key(|(diff, jitter, _)| (*diff, *jitter));

        let mut options: Vec<String> = candidates.into_iter().take(3).map(|(_, _, w)| w).collect();
        options.push(correct_translation.clone());
        options.shuffle(rng);
        let correct_index = options
            .iter()
            .position(|o| o == correct_translation)
            .unwrap_or(0);

        questions.push(QuizQuestion::MultipleChoice {
            source: QuizSource::Vocabulary,
            prompt: es_word.clone(),
            options,
            correct_index,
        });
    }

    questions
}

/// Load every expression card across all units as (spanish phrase, translated phrase) pairs
fn load_all_expression_pairs(ui_lang: &str) -> Vec<(String, String)> {
    let mut pairs = Vec::new();
    for unit in 1..=EXPRESSION_UNIT_COUNT {
        if let (Ok(es_cards), Ok(native_cards)) = (
            load_expression_unit(unit, "es"),
            load_expression_unit(unit, ui_lang),
        ) {
            for (es, native) in es_cards.iter().zip(native_cards.iter()) {
                pairs.push((es.phrase.clone(), native.phrase.clone()));
            }
        }
    }
    pairs
}

fn build_expression_questions(ui_lang: &str, rng: &mut SmallRng) -> Vec<QuizQuestion> {
    let mut pool = load_all_expression_pairs(ui_lang);
    pool.shuffle(rng);

    let mut questions = Vec::with_capacity(EXPRESSION_QUESTION_COUNT);
    for (es_phrase, native_phrase) in pool.into_iter() {
        if questions.len() == EXPRESSION_QUESTION_COUNT {
            break;
        }
        if let Some(question) = build_fill_in_blank(&es_phrase, &native_phrase, rng) {
            questions.push(question);
        }
    }

    questions
}

/// Extract the "core" word from a token by trimming leading/trailing punctuation
fn core_of(token: &str) -> String {
    token.trim_matches(|c: char| !c.is_alphanumeric()).to_string()
}

/// Build a fill-in-the-blank question from a Spanish phrase, blanking out 1-2 words
fn build_fill_in_blank(
    es_phrase: &str,
    native_phrase: &str,
    rng: &mut SmallRng,
) -> Option<QuizQuestion> {
    let tokens: Vec<&str> = es_phrase.split_whitespace().collect();
    if tokens.is_empty() {
        return None;
    }

    let mut candidate_indices: Vec<usize> = tokens
        .iter()
        .enumerate()
        .filter(|(_, t)| core_of(t).chars().count() >= 3)
        .map(|(i, _)| i)
        .collect();

    if candidate_indices.is_empty() {
        // Fallback: blank the longest available token, as long as it has a usable core
        let longest = tokens
            .iter()
            .enumerate()
            .max_by_key(|(_, t)| core_of(t).chars().count());
        match longest {
            Some((idx, t)) if !core_of(t).is_empty() => candidate_indices.push(idx),
            _ => return None,
        }
    }

    candidate_indices.shuffle(rng);
    let blank_count = if candidate_indices.len() >= 4 { 2 } else { 1 };
    let mut chosen: Vec<usize> = candidate_indices.into_iter().take(blank_count).collect();
    chosen.sort_unstable();

    let mut display_tokens: Vec<String> = tokens.iter().map(|t| t.to_string()).collect();
    let mut blanks = Vec::with_capacity(chosen.len());

    for &idx in &chosen {
        let token = tokens[idx];
        let core = core_of(token);
        if let Some(start) = token.find(core.as_str()) {
            let end = start + core.len();
            display_tokens[idx] = format!("{}____{}", &token[..start], &token[end..]);
        } else {
            display_tokens[idx] = "____".to_string();
        }
        blanks.push(core);
    }

    Some(QuizQuestion::FillInTheBlank {
        source: QuizSource::Expression,
        prompt: native_phrase.to_string(),
        display_phrase: display_tokens.join(" "),
        blanks,
    })
}

fn build_grammar_questions(ui_lang: &str, rng: &mut SmallRng) -> Vec<QuizQuestion> {
    let mut pool = Vec::new();
    for topic_id in 1..=GRAMMAR_TOPIC_COUNT {
        if let Ok(content) = load_grammar_content(topic_id, ui_lang) {
            pool.extend(content.questions);
        }
    }
    pool.shuffle(rng);

    pool.into_iter()
        .take(GRAMMAR_QUESTION_COUNT)
        .map(|q| {
            let correct_text = q
                .answers
                .iter()
                .find(|a| a.correct)
                .map(|a| a.text.clone())
                .unwrap_or_default();

            let mut options: Vec<String> = q.answers.into_iter().map(|a| a.text).collect();
            options.shuffle(rng);
            let correct_index = options.iter().position(|o| *o == correct_text).unwrap_or(0);

            QuizQuestion::MultipleChoice {
                source: QuizSource::Grammar,
                prompt: q.question,
                options,
                correct_index,
            }
        })
        .collect()
}
