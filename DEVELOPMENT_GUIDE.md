# Vamos - Language Learning App Development Guide

## Project Overview

**Vamos** is a serverless, client-side Spanish learning application built with Rust and Leptos 0.8, targeting smartphone users. The app focuses on vocabulary acquisition through staged learning with progressive reveal cards and pronunciation support. All content is static JSON, and user progress (favorites) exists only in browser memory during the active session.

---

## Core Principles

### 1. **Stateless by Design**

- No backend servers, no databases, no user authentication
- All state exists only in browser memory during active session
- Learning content is embedded in the application bundle
- Progress resets on page reload (intentional design choice)

### 2. **Mobile-First, Mobile-Only**

- Optimized viewport: 320px - 428px width
- Touch-friendly UI elements (minimum 44x44px tap targets)
- Vertical scrolling preferred over horizontal
- No hover states (use active/pressed states instead)

### 3. **Progressive Reveal Learning**

- Cards show information in stages: word → example → translation
- Navigation between cards with next/previous buttons
- Direction toggle: Spanish→English or English→Spanish
- Web Speech API for pronunciation support

### 4. **Global Card ID System**

- Each vocabulary card has a globally unique ID across all stages
- Stage 1: IDs 1-20, Stage 2: IDs 21-40, Stage 3: IDs 41-60
- Favorites system uses (stage, card_id) tuples
- Conversion needed when loading: card_id to stage-relative index

---

## Project Structure

```
vamos/
├── src/
│   ├── main.rs                    # App entry point, routing setup
│   │
│   ├── core/                      # Core types and business logic
│   │   ├── mod.rs
│   │   ├── language.rs            # Language enum and constants
│   │   └── favorites.rs           # FavoritesContext state management
│   │
│   ├── data/                      # Data loading and models
│   │   └── mod.rs                 # VocabularyCard, CardPair, get_card_pair()
│   │
│   └── pages/                     # Page-level components
│       ├── mod.rs
│       ├── home.rs                # Landing page
│       ├── vocabulary.rs          # Stage selection page
│       ├── vocabulary_cards.rs    # Card learning interface
│       ├── favorites.rs           # Favorites card navigation
│       └── grammar.rs             # Grammar page (placeholder)
│
├── translations/                  # Vocabulary JSON data
│   └── vocabulary/
│       ├── 1/                     # Stage 1 (IDs 1-20)
│       │   ├── es.json
│       │   └── en.json
│       ├── 2/                     # Stage 2 (IDs 21-40)
│       │   ├── es.json
│       │   └── en.json
│       └── 3/                     # Stage 3 (IDs 41-60)
│           ├── es.json
│           └── en.json
│
├── Cargo.toml                     # Rust dependencies
├── Trunk.toml                     # Trunk configuration (port 8087)
├── index.html                     # HTML entry point
├── styles.css                     # Global styles
├── .gitignore                     # Git ignore rules
├── DEVELOPMENT_GUIDE.md           # This file
└── README.md                      # Project description
```

---

## Architecture Decisions

### Technology Stack

```toml
# Cargo.toml
leptos = { version = "0.8.15", features = ["csr"] }  # Client-side rendering
leptos_router = "0.8"                                 # Routing
serde = { version = "1.0", features = ["derive"] }   # JSON serialization
serde_json = "1.0"                                    # JSON parsing
wasm-bindgen = "0.2"                                  # JavaScript interop
```

### Routing Structure

```rust
// All routes defined in main.rs
<Routes fallback=|| "Page not found">
    <Route path=path!("/") view=Home/>
    <Route path=path!("/vocabulary") view=Vocabulary/>
    <Route path=path!("/vocabulary/favorites") view=Favorites/>  // Before :stage!
    <Route path=path!("/vocabulary/:stage") view=VocabularyCards/>
    <Route path=path!("/grammar") view=Grammar/>
</Routes>
```

**Important**: `/vocabulary/favorites` must be defined BEFORE `/vocabulary/:stage` to prevent favorites being matched as a stage number.

### Global State Pattern (Leptos 0.8)

```rust
// Use provide_context at App level (NOT Provider component)
fn App() -> impl IntoView {
    provide_context(FavoritesContext::new());
    view! { <Router>...</Router> }
}

// Access in any component with expect_context
#[component]
pub fn SomeComponent() -> impl IntoView {
    let favorites_ctx = expect_context::<FavoritesContext>();
    // Use favorites_ctx methods
}
```

### Favorites System Architecture

```rust
// FavoritesContext stores (stage, card_id) tuples
pub struct FavoritesContext {
    favorites: RwSignal<HashSet<(u32, u32)>>,  // (stage, global_card_id)
}

impl FavoritesContext {
    pub fn toggle(&self, stage: u32, card_id: u32) { }
    pub fn is_favorite(&self, stage: u32, card_id: u32) -> bool { }
    pub fn get_all(&self) -> Vec<(u32, u32)> { }
    pub fn remove(&self, stage: u32, card_id: u32) { }
    pub fn count(&self) -> usize { }
}

// Usage in components
favorites_ctx.toggle(stage, source.id);  // source.id is global ID
let is_fav = favorites_ctx.is_favorite(stage, source.id);
```

### Card ID Mapping System

**Critical**: Card IDs are globally unique, but `get_card_pair()` expects stage-relative indices:

```rust
// Global ID ranges
// Stage 1: IDs 1-20   → indices 0-19
// Stage 2: IDs 21-40  → indices 0-19
// Stage 3: IDs 41-60  → indices 0-19

// Conversion formula
let card_idx = match stage {
    1 => (card_id - 1) as usize,
    2 => (card_id - 21) as usize,
    3 => (card_id - 41) as usize,
    _ => 0,
};

get_card_pair(stage, card_idx, direction)
```

### Data Loading Pattern

```rust
// JSON structure
[
  {"id": 1, "word": "hola", "example": "Hola, ¿cómo estás?"},
  {"id": 2, "word": "adiós", "example": "Adiós, hasta luego"}
]

// Loading function
pub fn get_card_pair(
    stage: u32,
    card_index: usize,  // 0-based, stage-relative
    direction: LearningDirection
) -> Result<(VocabularyCard, VocabularyCard), String>
```

### Vocabulary Content Guidelines

When creating or editing vocabulary JSON files, follow these standards:

**1. Gendered Nouns with Slash Notation**

```json
// ✅ CORRECT
{"id": 1, "word": "el/la hermano/a", "example": "Tengo un hermano mayor"}
{"id": 2, "word": "el/la amigo/a", "example": "Mi amigo vive en Madrid"}

// ❌ WRONG - separate entries for each gender
{"id": 1, "word": "hermano", "example": "..."}
{"id": 2, "word": "hermana", "example": "..."}
```

**2. Always Include Articles with Nouns**

```json
// ✅ CORRECT
{"id": 1, "word": "la casa", "example": "La casa está cerca"}
{"id": 2, "word": "el agua", "example": "El agua está fría"}

// ❌ WRONG - noun without article
{"id": 1, "word": "casa", "example": "..."}
```

**3. Adjectives with Gender Conjugation**

```json
// ✅ CORRECT
{"id": 1, "word": "nuevo/a", "example": "Compré un coche nuevo"}
{"id": 2, "word": "pequeño/a", "example": "Es un apartamento pequeño"}

// Note: Some adjectives don't change (e.g., "grande" for both genders)
{"id": 3, "word": "grande", "example": "Esta ciudad es muy grande"}
```

**4. Word Type Distribution**

Maintain a balanced mix across each stage:

- **Nouns**: ~40% (with articles: el/la/los/las)
- **Verbs**: ~35% (infinitive form)
- **Adjectives**: ~20% (with gender notation when applicable)
- **Adverbs/Other**: ~5%

**5. Avoid Easy Cognates**

Remove words that are nearly identical to English:

- ❌ hospital, restaurant, hotel, música, radio, televisión
- ✅ Include words that require actual learning

**6. Example Sentences**

- Use natural, everyday contexts
- Keep sentences short (5-10 words)
- Ensure examples demonstrate proper word usage
- Match the word form in the example (if word shows "nuevo/a", example can show either gender)

**Example Stage Structure:**

```json
[
  { "id": 1, "word": "el agua", "example": "Quiero beber el agua fría" },
  { "id": 2, "word": "comer", "example": "Me gusta comer frutas frescas" },
  { "id": 3, "word": "bueno/a", "example": "Es una buena idea" },
  { "id": 4, "word": "el/la amigo/a", "example": "Mi amigo vive en Madrid" }
]
```

---

## Coding Conventions

### Naming

- **Files**: `snake_case.rs`
- **Types/Structs/Enums**: `PascalCase`
- **Functions/Variables**: `snake_case`
- **Constants**: `SCREAMING_SNAKE_CASE`
- **Components**: `PascalCase` (Leptos convention)

### Component Guidelines

1. **Props**: Use `#[prop(into)]` for strings and simple conversions
2. **Signals**: Use `signal()` for local state, `RwSignal` for shared state
3. **Context**: Use `provide_context()` at App level, `expect_context()` in components
4. **Event Handlers**: Keep them simple, extract complex logic to functions
5. **Styling**: Use inline styles or CSS classes from styles.css

### URL Parameters and Navigation

```rust
// Reading URL params
use leptos_router::hooks::use_params;

let params = use_params::<VocabularyParams>();
let stage = move || {
    params.with(|p| p.as_ref().ok().map(|p| p.stage).unwrap_or(1))
};

// Reading query params
use leptos_router::hooks::use_query;

#[derive(serde::Deserialize, Clone)]
struct QueryParams {
    direction: Option<String>,
}

let query = use_query::<QueryParams>();
let direction = move || {
    query.with(|q| {
        q.as_ref().ok().and_then(|q| q.direction.as_deref())
    })
};

// Navigation with query params
use leptos_router::components::A;

view! {
    <A href=format!("/vocabulary/{}?direction=es-en", stage)>
        "Link"
    </A>
}
```

### Web Speech API Integration

```rust
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = window)]
    fn speak_text(text: &str, lang: &str);
}

// In component
let play_audio = move |_| {
    let text = current_word.get();
    let lang = if current_direction == "es-en" { "es-ES" } else { "en-US" };
    speak_text(&text, lang);
};
```

Corresponding JavaScript in index.html:

```javascript
window.speak_text = function (text, lang) {
  const utterance = new SpeechSynthesisUtterance(text);
  utterance.lang = lang;
  window.speechSynthesis.speak(utterance);
};
```

---

## Development Workflow

### Adding a New Vocabulary Stage

1. Create JSON files: `translations/vocabulary/{N}/es.json` and `en.json`
2. Use globally unique IDs: Stage 4 = IDs 61-80, Stage 5 = IDs 81-100, etc.
3. Update `data/mod.rs` load_vocabulary_stage() with new match arm
4. Update `favorites.rs` filter logic with new ID range
5. Test card navigation and favorites functionality

Example JSON structure:

```json
[
  { "id": 61, "word": "ejemplo", "example": "Este es un ejemplo." },
  { "id": 62, "word": "palabra", "example": "Esta palabra es nueva." }
]
```

### Adding a New Page

1. Create file in `src/pages/`
2. Define component with `#[component]`
3. Export from `pages/mod.rs`
4. Add route in `main.rs`
5. Add navigation links in other pages

Example:

```rust
// src/pages/my_page.rs
use leptos::prelude::*;

#[component]
pub fn MyPage() -> impl IntoView {
    view! {
        <div class="page">
            <h1>"My Page"</h1>
        </div>
    }
}

// src/pages/mod.rs
pub mod my_page;
pub use my_page::MyPage;

// src/main.rs
<Route path=path!("/my-page") view=MyPage/>
```

### Debugging Favorites Issues

Common issues and solutions:

1. **Cards not showing**: Check if card_id falls within correct range for stage
2. **Wrong card displays**: Verify card_idx calculation uses correct stage offset
3. **Empty favorites**: Use browser console to check `favorites_ctx.get_all()`
4. **Favorites not persisting**: Expected behavior - state resets on page reload

---

## Best Practices

### Progressive Reveal Pattern

The vocabulary cards use a progressive reveal pattern:

```rust
let (show_example, set_show_example) = signal(false);
let (show_translation, set_show_translation) = signal(false);

// Initially show only the word
view! {
    <div class="word">{current_word}</div>

    <Show when=move || show_example.get()>
        <div class="example">{current_example}</div>
    </Show>

    <Show when=move || show_translation.get()>
        <div class="translation">{translation_word}</div>
    </Show>
}
```

### Button Disabled State (Leptos 0.8)

Use signal closures for reactive disabled state:

```rust
// ✅ CORRECT
<button disabled={move || card_index.get() == 0}>
    "Previous"
</button>

// ❌ WRONG - these don't work in Leptos 0.8
<button disabled=is_disabled>          // Not reactive
<button prop:disabled=is_disabled>     // Old syntax
<button disabled={is_disabled}>        // Static value
```

### Sorting Collections

When displaying favorites or any list that should be sorted:

```rust
let sorted_items = move || {
    let mut items = get_items();
    items.sort_by_key(|(_, id)| *id);  // Sort by ID
    items
};
```

### Error Handling in Components

Use `Result` types and display errors clearly:

```rust
let data = move || -> Result<CardPair, String> {
    get_card_pair(stage, index, direction)
        .map_err(|e| format!("Error loading card: {}", e))
};

view! {
    {move || match data() {
        Ok((source, target)) => view! { /* Render card */ }.into_any(),
        Err(e) => view! { <p class="error">{e}</p> }.into_any(),
    }}
}
```

---

## Current Implementation Status

### ✅ Completed Features

- **Vocabulary System**
  - 3 stages with 20 cards each (60 total cards)
  - Global card ID system (Stage 1: 1-20, Stage 2: 21-40, Stage 3: 41-60)
  - Spanish and English translations
  - JSON-based content structure

- **Progressive Reveal UI**
  - Word → Example → Translation reveal pattern
  - Touch-friendly buttons for progression
  - Card navigation (next/previous)

- **Direction Toggle**
  - Spanish→English or English→Spanish
  - Persisted in URL query parameters
  - Flag emoji indicators (🇪🇸→🇬🇧 or 🇬🇧→🇪🇸)

- **Favorites System**
  - Global favorites context with RwSignal
  - Add/remove favorites from any stage
  - Dedicated favorites page with card navigation
  - Sorted by card ID
  - Session-only persistence (resets on reload)

- **Web Speech API**
  - Text-to-speech pronunciation
  - Language-specific voices (es-ES, en-US)
  - Audio button on each card

- **Routing**
  - Home page
  - Vocabulary stage selection
  - Stage card view
  - Favorites view
  - Grammar page (placeholder)

- **Build Configuration**
  - Trunk.toml with port 8087
  - Separate CSS file (styles.css)
  - .gitignore for build artifacts

### 🚧 Planned/Future Features

- **More Stages**: Currently 3/20 stages implemented
- **Grammar Section**: Placeholder exists, needs content
- **Additional Languages**: Currently Spanish only
- **Audio Recording**: Compare pronunciation
- **Spaced Repetition**: Smart card ordering
- **PWA Support**: Offline capability
- **Animations**: Smooth transitions between cards

### ❌ Not Planned

- User accounts or authentication
- Backend server or database
- Progress persistence across sessions
- Social features or sharing

---

## Build and Deployment

### Development

```bash
# Start development server with hot reload
trunk serve

# App runs on http://localhost:8087 (configured in Trunk.toml)
# Changes to Rust files trigger automatic recompilation
```

### Production Build

```bash
# Create optimized production build
trunk build --release

# Output directory: dist/
# Files: index.html, vamos-*.wasm, vamos-*.js, styles.css
```

### Deployment

The `dist/` folder can be deployed to any static hosting service:

- **Netlify**: Drag and drop `dist/` folder
- **Vercel**: Import repository, set output directory to `dist`
- **GitHub Pages**: Push `dist/` to `gh-pages` branch
- **Cloudflare Pages**: Connect repository, build command: `trunk build --release`

No server-side configuration needed - everything runs in the browser.

---

## Common Patterns and Examples

### Creating a New Vocabulary Stage

```bash
# 1. Create JSON files
mkdir translations/vocabulary/4
cd translations/vocabulary/4

# 2. Create es.json with IDs 61-80
[
  {"id": 61, "word": "palabra1", "example": "Ejemplo 1"},
  {"id": 62, "word": "palabra2", "example": "Ejemplo 2"},
  ...
]

# 3. Create en.json with matching IDs
[
  {"id": 61, "word": "word1", "example": "Example 1"},
  {"id": 62, "word": "word2", "example": "Example 2"},
  ...
]
```

```rust
// 4. Update src/data/mod.rs
fn load_vocabulary_stage(stage: u32, lang: &str) -> Result<Vec<VocabularyCard>, String> {
    let json = match stage {
        1 => include_str!("../../translations/vocabulary/1/es.json"),
        2 => include_str!("../../translations/vocabulary/2/es.json"),
        3 => include_str!("../../translations/vocabulary/3/es.json"),
        4 => include_str!("../../translations/vocabulary/4/es.json"),  // Add this
        // ... more stages
        _ => return Err("Stage not found".to_string()),
    };
    serde_json::from_str(json).map_err(|e| e.to_string())
}

// 5. Update src/pages/favorites.rs filter logic
let valid = match stage {
    1 => *card_id >= 1 && *card_id <= 20,
    2 => *card_id >= 21 && *card_id <= 40,
    3 => *card_id >= 41 && *card_id <= 60,
    4 => *card_id >= 61 && *card_id <= 80,  // Add this
    _ => false,
};

// 6. Update card_idx calculation in favorites.rs
let card_idx = match stage {
    1 => (card_id - 1) as usize,
    2 => (card_id - 21) as usize,
    3 => (card_id - 41) as usize,
    4 => (card_id - 61) as usize,  // Add this
    _ => 0,
};
```

### Signal Patterns

```rust
// Local state - resets on component unmount
let (count, set_count) = signal(0);

// Computed/derived signals
let doubled = move || count.get() * 2;

// Reactive updates
set_count.set(count.get() + 1);
set_count.update(|n| *n += 1);

// Global context state - shared across components
provide_context(MyContext::new());
let ctx = expect_context::<MyContext>();
```

### Navigation with State

```rust
use leptos_router::components::A;

// Simple link
view! { <A href="/vocabulary">"Vocabulary"</A> }

// With URL parameters
view! { <A href=format!("/vocabulary/{}", stage_num)>"Stage"</A> }

// With query parameters
view! {
    <A href=format!("/vocabulary/{}?direction=en-es", stage)>
        "Stage with direction"
    </A>
}

// Programmatic navigation
use leptos_router::hooks::use_navigate;
let navigate = use_navigate();
navigate("/vocabulary", Default::default());
```

---

## Troubleshooting Guide

### Compilation Errors

**Error**: "Provider component not found"

- **Solution**: Use `provide_context()` function instead of `<Provider>` component (Leptos 0.8)

**Error**: "Type mismatch in view macro"

- **Solution**: Use `.into_any()` when returning different view types from match arms

**Error**: "Signal not found in scope"

- **Solution**: Ensure signal is created in the right scope; use `move ||` closures to capture

### Runtime Issues

**Problem**: Buttons not responding to clicks

- **Check**: Event handler syntax: `on:click=move |_| { }`
- **Check**: No compile errors preventing WASM generation

**Problem**: Disabled buttons not updating reactively

- **Solution**: Use closure: `disabled={move || condition.get()}`
- **Wrong**: `disabled=condition` (not reactive)

**Problem**: Favorites not showing cards from Stage 2/3

- **Check**: Card ID ranges in filter match actual JSON IDs
- **Check**: Card index calculation uses correct stage offset
- **Debug**: Add `leptos::logging::log!()` to trace data flow

**Problem**: "Card not found" error

- **Check**: Stage number exists in load_vocabulary_stage() match
- **Check**: Card index is within 0-19 range
- **Check**: JSON files exist and are valid

### Development Server

**Problem**: Port 8087 already in use

- **Solution**: Kill existing process or change port in Trunk.toml

```powershell
# Windows PowerShell
Get-NetTCPConnection -LocalPort 8087 | ForEach-Object {
    Stop-Process -Id $_.OwningProcess -Force
}
```

**Problem**: Changes not reflected in browser

- **Solution**: Hard refresh (Ctrl+Shift+R) or clear browser cache
- **Solution**: Check terminal for compilation errors

**Problem**: WASM file not loading

- **Check**: No JavaScript errors in browser console
- **Check**: File paths in index.html are correct
- **Solution**: `trunk clean` then `trunk serve`

---

## Key Learnings and Decisions

### Why Global Card IDs?

Each vocabulary card has a globally unique ID rather than per-stage IDs (1-20 for each stage). This design:

- **Prevents conflicts**: No ambiguity when favoriting cards from different stages
- **Simplifies favorites**: Can store just (stage, card_id) without additional metadata
- **Scales better**: Adding stages doesn't require renumbering existing cards

Trade-off: Requires conversion when loading cards since `get_card_pair()` expects stage-relative indices.

### Why Route Order Matters

```rust
// ✅ CORRECT order
<Route path="/vocabulary/favorites" view=Favorites/>
<Route path="/vocabulary/:stage" view=VocabularyCards/>

// ❌ WRONG order - favorites matches as :stage = "favorites"
<Route path="/vocabulary/:stage" view=VocabularyCards/>
<Route path="/vocabulary/favorites" view=Favorites/>
```

Leptos router matches routes in order. More specific routes must come before parameterized routes.

### Why Session-Only State?

Favorites and progress reset on page reload by design:

- **No backend complexity**: Pure client-side application
- **Privacy focused**: No data stored or tracked
- **Simpler implementation**: No localStorage sync or state management
- **Educational focus**: Encourages regular practice rather than gamification

This aligns with the app's goal of being a lightweight learning tool, not a progress-tracking platform.

### Why Leptos 0.8 Changes?

Leptos 0.8 introduced breaking changes from 0.7:

- `signal()` instead of `create_signal()` for local state
- `provide_context()` instead of `<Provider>` component
- Different prop syntax for reactive attributes
- `into_any()` for type unification in match arms

Always check Leptos version when following examples online.

---

## Questions to Ask Before Implementation

1. **Does this need global state or local state?**
   - Global → Use context (FavoritesContext)
   - Local → Use signal within component

2. **What's the data flow?**
   - JSON → load_vocabulary_stage() → get_card_pair() → Component

3. **How does this interact with card IDs?**
   - Storing → Use global card_id
   - Loading → Convert to stage-relative index

4. **Should this survive page reload?**
   - No → Current design (session-only)
   - Yes → Would need localStorage (not implemented)

5. **Is this mobile-friendly?**
   - Touch targets ≥44px
   - No hover states
   - Works in portrait viewport

---

**Remember**: This is a learning app focused on simplicity and immediate feedback. Keep features lightweight, avoid over-engineering, and prioritize the learning experience over complex state management.
