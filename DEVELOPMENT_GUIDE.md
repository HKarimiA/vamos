# Vamos - Development Guide (Current)

## Overview

**Vamos** is a client-side Spanish learning app built with **Rust (edition 2024) + Leptos 0.8 (CSR)** and bundled with **Trunk**.

Core product constraints:

- No backend, no authentication, no database.
- Data is static JSON embedded at build time (`include_str!()`).
- Runtime state is session-only (in-memory signals/context) — refreshing clears everything.
- Mobile-first touch UX; the app is designed and tested primarily for phone screens.
- UI is localized (English/German); learning content is always Spanish.

> For AI-agent-specific conventions, gotchas, and "how to add content" checklists, see
> [.github/copilot-instructions.md](.github/copilot-instructions.md) — that file is the source of truth
> for day-to-day implementation patterns and should be kept in sync with this guide.

---

## Current Architecture

### Tech Stack

```toml
# Cargo.toml
[package]
edition = "2024"

leptos = { version = "0.8.15", features = ["csr"] }
leptos_router = "0.8"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
wasm-bindgen = "0.2"
rand = { version = "0.8", features = ["small_rng"] }
getrandom = { version = "0.2", features = ["js"] }
```

### App Routes

Defined in `src/main.rs`:

```rust
<Route path=path!("/") view=Home/>
<Route path=path!("/settings") view=Settings/>
<Route path=path!("/vocabulary") view=Vocabulary/>
<Route path=path!("/vocabulary/favorites") view=Favorites/>
<Route path=path!("/vocabulary/:stage") view=VocabularyCards/>
<Route path=path!("/grammar") view=Grammar/>
<Route path=path!("/grammar/:id") view=GrammarTopic/>
<Route path=path!("/expressions") view=Expressions/>
<Route path=path!("/expressions/:unit") view=ExpressionCards/>
<Route path=path!("/quiz") view=Quiz/>
```

Important:

- Keep static routes (e.g. `/vocabulary/favorites`) **before** parameterized siblings (`/vocabulary/:stage`).

### Global State

Two contexts are provided at the app root (`src/main.rs`) and read via `expect_context`:

```rust
provide_context(FavoritesContext::new());
provide_context(LanguageContext::new());
```

- `FavoritesContext` — favorites storage shape: `HashSet<(u32, u32)>` as `(stage, global_card_id)`.
- `LanguageContext` — wraps `RwSignal<Language>` for the **UI** language (English/German), defaulting to German. The Spanish content being *learned* is not affected by this — see `LEARNING_LANGUAGE` in `core/language.rs`.

---

## Current Project Structure

```text
src/
  main.rs
  components/
    mod.rs
    vocabulary_card.rs         # Shared card with swipe + nav + reveal UX (used by vocabulary/favorites/expression pages)
  core/
    favorites.rs               # FavoritesContext
    language.rs                # Language enum + LanguageContext (UI language switching)
    mod.rs
  data/
    vocabulary.rs               # Vocabulary models + stage loading (1..=33) + card pairing
    expressions.rs               # Everyday-expression models + unit loading (1..=27)
    grammar.rs                   # Grammar topics/content loaders (1..=27, en only)
    quiz.rs                      # Dynamic 20-question quiz session generator
    ui_translations.rs           # UiStrings struct + get_ui_strings(Language), loads translations/ui/*.json
    mod.rs
  pages/
    home.rs                     # 4 nav buttons: Vocabulary, Expressions, Grammar, Quiz
    vocabulary.rs
    vocabulary_cards.rs
    favorites.rs
    expressions.rs
    expression_cards.rs
    grammar.rs
    grammar_topic.rs             # Quiz + explanation/hint modals + results
    quiz.rs                      # Global mixed quiz: start screen / MC + fill-in-blank / results
    settings.rs                  # UI language selector (🇬🇧/🇩🇪)
    mod.rs

translations/
  vocabulary/1..=33/{es,en,de}.json
  expressions/1..=27/{es,en,de}.json
  grammar/1..=27/en.json          # English-only; German UI users still see English grammar content
  ui/{en,de}.json                 # UI chrome strings, keys match UiStrings field names exactly
```

---

## Vocabulary System

### Stages and IDs

- Implemented stages: **1..=33**.
- Card ID pattern per stage: blocks of 20 (Stage N: `((N-1)*20 + 1) .. (N*20)`).

Stage-relative index conversion:

```rust
let card_idx = (card_id - ((stage - 1) * 20 + 1)) as usize;
```

### Direction Query Param

Direction is controlled via `dir` query value: `es-to-en` / `en-to-es` (independent of the UI language).

### Data Loading

Vocabulary stage loading is macro-generated in `src/data/vocabulary.rs` (`include_vocabulary_stages!`), supporting `es`/`en`/`de` per stage.

---

## Expressions System

Same shape as vocabulary but simpler: `ExpressionCard { id, phrase }` — no example sentence field.

- Implemented units: **1..=27**, loaded via `include_expression_units!` macro in `src/data/expressions.rs`.
- Pages: `/expressions` (unit list) and `/expressions/:unit` (card browser, reuses `VocabularyCard` component).

---

## Grammar System

- Grammar topic list (`/grammar`) with difficulty color coding.
- Topic detail (`/grammar/:id`) with: explanation modal, hint modal, multiple-choice quiz, shuffled answers (`SmallRng`), score + results screen.
- Content loader supports topics `1..=27` from `translations/grammar/*/en.json` (no `de`/`es` variants yet).

---

## Quiz System (`/quiz`)

`src/data/quiz.rs` builds a fresh **20-question mixed session** on demand — no new content files needed, it samples from the existing vocabulary/expression/grammar pools:

- 5 Vocabulary questions → `MultipleChoice` (translate a Spanish word; distractors ranked by length-similarity to the correct answer + random jitter, so options vary each session).
- 8 Expression questions → `FillInTheBlank` (1-2 blanked words in a Spanish phrase, checked via `normalize_answer` — lenient on case/accents/punctuation, but `ñ` is preserved as distinct from `n`).
- 7 Grammar questions → `MultipleChoice`, sampled from all topics' existing quiz questions.

All 20 are shuffled together. `src/pages/quiz.rs` is split into small components (`QuizStart`, `QuizResults`, `MultipleChoiceCard`, `FillInBlankCard`) to avoid Leptos `view!{}` type-inference issues — see the Leptos gotchas in [.github/copilot-instructions.md](.github/copilot-instructions.md).

---

## i18n / UI Translations

- `Language` enum (`core/language.rs`): `Spanish`, `English`, `German`. `LanguageContext` holds the **UI** language (default German); `LEARNING_LANGUAGE` is always `Spanish`.
- `UiStrings` (`data/ui_translations.rs`) is `Deserialize`d directly from `translations/ui/{en,de}.json` — every field must exist in **both** files or JSON parsing fails at runtime for that language.
- Content loaders (vocabulary/expressions/grammar) key off plain `"es"`/`"en"`/`"de"` string literals, not the `Language` enum — convert with a small `match` where needed (see `quiz.rs`'s `ui_lang` closure).

---

## Shared Vocabulary Card UX

`src/components/vocabulary_card.rs` now centralizes:

- Progressive reveal flow: word → example → translation
- Favorite toggle
- Audio buttons (Web Speech API bridge)
- Previous/Next buttons
- Swipe gestures (pointer events)
- Card transition animation + content reveal timing

### Swipe Logic Constants

```rust
const SWIPE_THRESHOLD: f64 = 32.0;
const SWIPE_HORIZONTAL_RATIO: f64 = 0.75;
const MAX_DRAG_OFFSET: f64 = 80.0;
const MAX_ROTATE_DEG: f64 = 2.0;
```

Gesture behavior:

- Drag follows pointer on horizontal intent.
- Left swipe triggers next card.
- Right swipe triggers previous card.
- State resets on `pointerup`, `pointercancel`, and `pointerleave`.

### Animation Notes

CSS classes in `styles.css` coordinate card transitions:

- `card-animator`
- `card-enter-next`, `card-enter-prev`
- `card-content`, `card-content-reveal-*`

This provides:

- Directional card-change motion
- Brief blank/reveal phase for incoming content
- Smooth reset when drag ends

---

## Favorites Behavior

Favorites page (`src/pages/favorites.rs`):

- Reads all favorites from context, filters to valid stage/card ID ranges (stages `1..=33`).
- Sorts by global `card_id`, converts global IDs to stage-relative indices before calling `get_card_pair`.
- Uses the shared `VocabularyCard` component for identical interaction behavior.

---

## Mobile UX Guidelines (Keep)

- Target viewport: phone widths; the app is not designed for desktop use.
- Touch-friendly controls (≥44px targets), `touch-action: manipulation` on interactive elements.
- Prefer `:active` press feedback over hover-dependent UX.
- Reuse existing breakpoints in `styles.css` (`@media (max-width: 375px)`, `@media (max-height: 600px)`) rather than adding new ad-hoc ones.
- Keep navigation and reveal flow simple and quick.

---

## Development Workflow

### Run locally

```bash
trunk serve
```

Configured in `Trunk.toml`: host `127.0.0.1`, port `8087`.

### Build

```bash
trunk build --release
```

### Checks

```bash
cargo check --target wasm32-unknown-unknown
cargo clippy
```

**PowerShell quirk**: piping `cargo`/`trunk` output through `Select-Object` (or similar) can print a spurious `NativeCommandError` wrapper and a nonzero exit code even when the build succeeded. Trust the presence of `Finished \`dev\`/\`release\` profile` (cargo) or `INFO success` (trunk) in the output over the raw exit code.

---

## Adding New Vocabulary Stage

1. Add `translations/vocabulary/{N}/{es,en,de}.json`.
2. Keep global card ID block consistent (20 per stage).
3. Extend the stage list in the `include_vocabulary_stages!` macro call in `src/data/vocabulary.rs`.
4. Extend any hardcoded stage-range bounds (e.g. favorites filtering, stage grid in `src/pages/vocabulary.rs`).

## Adding New Expression Unit

Same pattern as vocabulary, in `src/data/expressions.rs` / `translations/expressions/`.

## Adding New Grammar Topic

1. Add `translations/grammar/{id}/en.json` with a valid `GrammarContent` shape.
2. Add topic metadata in `get_all_topics()` in `src/data/grammar.rs`.
3. Extend the topic ID list in the grammar include macro call in `src/data/grammar.rs`.

## Adding a New UI String

1. Add the field to `UiStrings` in `src/data/ui_translations.rs`.
2. Add the matching key to **both** `translations/ui/en.json` and `translations/ui/de.json`.

---

## Practical Notes

- `core/language.rs` defines the UI language enum/context; content language codes elsewhere are plain `"es"/"en"/"de"` strings, not this enum.
- Session-only persistence is intentional: refreshing the page clears favorites and any in-progress quiz.
- For router changes, always check route ordering for static vs parameterized paths.
- For Leptos-specific implementation patterns and known compiler-error workarounds (event handler closures, `StoredValue`, nested `view!{}` type inference, `rand`'s `gen` keyword conflict on edition 2024), see [.github/copilot-instructions.md](.github/copilot-instructions.md).

---

## Quick Troubleshooting

- **Favorites page shows empty unexpectedly**
  - Verify `(stage, card_id)` pair is in the expected range and the stage exists in the vocabulary loader list.
- **Card not found / out of bounds**
  - Re-check the global ID → stage index conversion formula.
- **Swipe feels off**
  - Tune constants in `vocabulary_card.rs` (`SWIPE_THRESHOLD`, ratio, drag cap).
- **A new UI string breaks the page for one language only**
  - You likely added the field to `UiStrings` but forgot to add the key to one of `translations/ui/{en,de}.json`.
- **`on:click`/`on:input` closure fails to compile with "expected Fn, found FnOnce"**
  - You're moving owned non-`Copy` data (e.g. `Vec<String>`) into the closure; wrap it in `StoredValue::new(...)` instead.
- **Dev server issues**
  - Run `trunk clean` then `trunk serve`.

---

## Status Snapshot (Current)

Implemented:

- Vocabulary stages 1..=33, Expression units 1..=27, Grammar topics 1..=27
- Favorites with shared card UX
- Direction toggle via query params
- Swipe + animated card transitions
- Grammar topic quiz flow + global mixed Quiz feature (`/quiz`)
- UI language switching (English/German) via Settings page

Not implemented:

- Backend/auth/persistence
- Grammar content in German/Spanish (English only)
