# Vamos - Development Guide (Current)

## Overview

**Vamos** is a client-side Spanish learning app built with **Rust + Leptos 0.8 (CSR)** and bundled with **Trunk**.

Core product constraints:

- No backend, no authentication, no database.
- Data is static JSON embedded at build time.
- Runtime state is session-only (in-memory signals/context).
- Mobile-first touch UX.

---

## Current Architecture

### Tech Stack

```toml
# Cargo.toml
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
<Route path=path!("/vocabulary") view=Vocabulary/>
<Route path=path!("/vocabulary/favorites") view=Favorites/>
<Route path=path!("/vocabulary/:stage") view=VocabularyCards/>
<Route path=path!("/grammar") view=Grammar/>
<Route path=path!("/grammar/:id") view=GrammarTopic/>
```

Important:

- Keep `/vocabulary/favorites` **before** `/vocabulary/:stage`.

### Global State

`FavoritesContext` is provided at app root and read via `expect_context`.

```rust
provide_context(FavoritesContext::new());
```

Favorites storage shape:

- `HashSet<(u32, u32)>` as `(stage, global_card_id)`.

---

## Current Project Structure

```text
src/
  main.rs
  components/
    mod.rs
    vocabulary_card.rs         # Shared card with swipe + nav + reveal UX
  core/
    favorites.rs               # FavoritesContext
    language.rs                # Language enum/constants (currently mostly unused)
    mod.rs
  data/
    grammar.rs                 # Grammar topics/content loaders
    vocabulary.rs              # Vocabulary models + stage loading + card pairing
    mod.rs
  pages/
    home.rs
    vocabulary.rs
    vocabulary_cards.rs
    favorites.rs
    grammar.rs
    grammar_topic.rs           # Quiz + explanation/hint modals + results
    mod.rs

translations/
  vocabulary/
    1..21/{es.json,en.json}
  grammar/
    1..27/en.json
```

---

## Vocabulary System (Current)

### Stages and IDs

- Implemented stages: **1..=21**.
- Expected card ID pattern per stage: blocks of 20.
  - Stage 1: 1-20
  - Stage 2: 21-40
  - ...
  - Stage N: `((N-1)*20 + 1) .. (N*20)`

Stage-relative index conversion:

```rust
let card_idx = (card_id - ((stage - 1) * 20 + 1)) as usize;
```

### Direction Query Param

Direction is controlled via `dir` query value:

- `es-to-en`
- `en-to-es`

Examples:

- `/vocabulary?dir=es-to-en`
- `/vocabulary/4?dir=en-to-es`

### Data Loading

Vocabulary stage loading is macro-generated in `src/data/vocabulary.rs` and currently supports stages `1..=21` for `es` and `en`.

---

## Grammar System (Current)

Implemented (not placeholder):

- Grammar topic list (`/grammar`) with difficulty color coding.
- Topic detail (`/grammar/:id`) with:
  - Explanation modal
  - Hint modal
  - Multiple-choice quiz
  - Shuffled answers (SmallRng)
  - Score + results screen

Grammar content loader currently supports topics `1..=27` from `translations/grammar/*/en.json`.

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

- Reads all favorites from context.
- Filters entries to valid stage/card ID ranges (currently stages `1..=21`).
- Sorts by global `card_id`.
- Converts global card IDs to stage-relative indices before calling `get_card_pair`.
- Uses shared `VocabularyCard` for identical interaction behavior.

---

## Mobile UX Guidelines (Keep)

- Target viewport: phone widths.
- Touch-friendly controls (>=44px targets where practical).
- Prefer active/pressed interactions over hover-dependent UX.
- Keep navigation and reveal flow simple and quick.

---

## Development Workflow

### Run locally

```bash
trunk serve
```

Configured in `Trunk.toml`:

- Host: `127.0.0.1`
- Port: `8087`

### Build

```bash
trunk build --release
```

### Checks

```bash
cargo check
cargo clippy
```

---

## Adding New Vocabulary Stage

1. Add files:
   - `translations/vocabulary/{N}/es.json`
   - `translations/vocabulary/{N}/en.json`
2. Keep global card ID block consistent (20 per stage).
3. Extend stage list in macro call inside `src/data/vocabulary.rs`.
4. Favorites page already supports formula-based ranges for `1..=21`; extend that bound when adding stages.
5. Verify stage grid range in `src/pages/vocabulary.rs` (`1..=21`).

---

## Adding New Grammar Topic

1. Add `translations/grammar/{id}/en.json` with valid `GrammarContent` shape.
2. Add topic metadata in `get_all_topics()` in `src/data/grammar.rs`.
3. Extend topic ID list in grammar include macro call in `src/data/grammar.rs`.

---

## Practical Notes

- `core/language.rs` currently defines language-level constants/enums and is mostly future-facing.
- Session-only persistence is intentional: refreshing the page clears favorites.
- For router changes, always check route ordering for static vs parameterized paths.

---

## Quick Troubleshooting

- **Favorites page shows empty unexpectedly**
  - Verify `(stage, card_id)` pair is in expected range.
  - Verify stage exists in vocabulary loader list.
- **Card not found / out of bounds**
  - Re-check global ID → stage index conversion formula.
- **Swipe feels off**
  - Tune constants in `vocabulary_card.rs` (`SWIPE_THRESHOLD`, ratio, drag cap).
- **Dev server issues**
  - Run `trunk clean` then `trunk serve`.

---

## Status Snapshot (Current)

Implemented:

- Vocabulary stages 1..21
- Favorites with shared card UX
- Direction toggle via query params
- Swipe + animated card transitions
- Grammar topics 1..27 with quiz flow

Not implemented:

- Backend/auth/persistence
- Multi-language content beyond current Spanish-learning setup
