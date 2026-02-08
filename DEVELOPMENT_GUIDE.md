# Vamos - Language Learning App Development Guide

## Project Overview

**Vamos** is a serverless, client-side language learning application built with Rust and Leptos, targeting smartphone users exclusively. The app focuses on vocabulary and grammar acquisition through bite-sized, progressive exercises without user accounts or data persistence.

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

### 3. **Modular Architecture**

- Small, focused files (max 200-300 lines)
- Single Responsibility Principle for components
- Clear separation of concerns
- Easy to navigate and modify

### 4. **Language Extensibility**

- Languages defined as type-safe enums
- Content structures independent of specific languages
- Adding a new language should require minimal changes

---

## Project Structure

```
vamos/
├── src/
│   ├── main.rs                    # App entry point, routing setup
│   ├── app.rs                     # Root app component
│   │
│   ├── core/                      # Core types and business logic
│   │   ├── mod.rs
│   │   ├── language.rs            # Language enum and metadata
│   │   ├── content.rs             # Content types (Word, Phrase, Exercise)
│   │   └── exercise_types.rs     # Exercise variants (MultipleChoice, FillBlank, etc.)
│   │
│   ├── data/                      # Embedded learning content
│   │   ├── mod.rs
│   │   ├── spanish/               # Spanish-specific content
│   │   │   ├── mod.rs
│   │   │   ├── vocabulary.rs      # Vocabulary lists by category
│   │   │   └── grammar.rs         # Grammar rules and exercises
│   │   └── loader.rs              # Content loading utilities
│   │
│   ├── state/                     # Application state management
│   │   ├── mod.rs
│   │   ├── app_state.rs           # Global app state (current language, session)
│   │   └── session.rs             # Session-scoped state (current lesson, score)
│   │
│   ├── components/                # Reusable UI components
│   │   ├── mod.rs
│   │   ├── layout/                # Layout components
│   │   │   ├── mod.rs
│   │   │   ├── app_shell.rs       # Main app container
│   │   │   ├── header.rs          # Top navigation/title bar
│   │   │   └── bottom_nav.rs      # Bottom navigation bar
│   │   │
│   │   ├── common/                # Common UI elements
│   │   │   ├── mod.rs
│   │   │   ├── button.rs          # Reusable button component
│   │   │   ├── card.rs            # Card container component
│   │   │   └── progress_bar.rs   # Progress indicator
│   │   │
│   │   └── exercise/              # Exercise-specific components
│   │       ├── mod.rs
│   │       ├── multiple_choice.rs # Multiple choice question UI
│   │       ├── fill_blank.rs      # Fill-in-the-blank UI
│   │       └── flashcard.rs       # Flashcard flip component
│   │
│   ├── pages/                     # Page-level components
│   │   ├── mod.rs
│   │   ├── home.rs                # Landing/language selection
│   │   ├── lesson_select.rs       # Choose lesson type
│   │   ├── practice.rs            # Active practice session
│   │   └── session_summary.rs     # End-of-session results
│   │
│   └── styles/                    # Styling utilities
│       ├── mod.rs
│       ├── theme.rs               # Color palette, spacing constants
│       └── responsive.rs          # Responsive utilities
│
├── Cargo.toml
├── index.html                     # HTML entry point
├── DEVELOPMENT_GUIDE.md           # This file
└── README.md                      # Project description
```

---

## Architecture Decisions

### Component Design Pattern

```rust
// ✅ GOOD: Small, focused component with clear props
#[component]
pub fn Button(
    #[prop(into)] text: String,
    #[prop(optional)] on_click: Option<Box<dyn Fn()>>,
    #[prop(default = ButtonVariant::Primary)] variant: ButtonVariant,
) -> impl IntoView {
    view! {
        <button
            class=format!("btn btn-{}", variant.to_class())
            on:click=move |_| {
                if let Some(handler) = &on_click {
                    handler();
                }
            }
        >
            {text}
        </button>
    }
}

// ❌ BAD: Component doing too much
#[component]
pub fn ExercisePageWithNavigationAndStateAndContent() -> impl IntoView {
    // Don't create monolithic components
}
```

### State Management Strategy

```rust
// Use Leptos signals for reactive state
// Global state: use provide_context/use_context
// Local state: use create_signal within components

// Example: Session state
#[derive(Clone)]
pub struct SessionState {
    pub current_exercise: RwSignal<usize>,
    pub score: RwSignal<u32>,
    pub answers: RwSignal<Vec<Answer>>,
}

impl SessionState {
    pub fn new() -> Self {
        Self {
            current_exercise: create_rw_signal(0),
            score: create_rw_signal(0),
            answers: create_rw_signal(Vec::new()),
        }
    }
}
```

### Language Enum Design

```rust
// Centralized, extensible language definition
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Language {
    Spanish,
    // Future: French, German, Italian, etc.
}

impl Language {
    pub fn display_name(&self) -> &'static str {
        match self {
            Language::Spanish => "Spanish",
        }
    }

    pub fn native_name(&self) -> &'static str {
        match self {
            Language::Spanish => "Español",
        }
    }

    pub fn flag_emoji(&self) -> &'static str {
        match self {
            Language::Spanish => "🇪🇸",
        }
    }
}
```

---

## Coding Conventions

### Naming

- **Files**: `snake_case.rs`
- **Types/Structs/Enums**: `PascalCase`
- **Functions/Variables**: `snake_case`
- **Constants**: `SCREAMING_SNAKE_CASE`
- **Components**: `PascalCase` (Leptos convention)

### File Organization

Each module file should follow this order:

1. Imports (grouped: std, external crates, internal modules)
2. Type definitions
3. Implementations
4. Public functions
5. Private helpers

```rust
// Good structure example
use leptos::*;
use crate::core::Language;

#[derive(Clone, PartialEq)]
pub struct Exercise {
    pub id: String,
    pub question: String,
    pub answers: Vec<String>,
}

impl Exercise {
    pub fn new(id: String, question: String) -> Self {
        Self {
            id,
            question,
            answers: Vec::new(),
        }
    }

    pub fn is_correct(&self, answer: &str) -> bool {
        self.answers.contains(&answer.to_string())
    }
}

pub fn create_exercise_set() -> Vec<Exercise> {
    // Implementation
}
```

### Component Guidelines

1. **Props**: Use `#[prop(into)]` for strings and simple conversions
2. **Signals**: Create signals at the appropriate scope (local vs context)
3. **Side Effects**: Use `create_effect` sparingly, prefer derived signals
4. **Event Handlers**: Keep them simple, extract complex logic to functions
5. **Styling**: Use class strings, consider a utility-first approach

### CSS/Styling Approach

```rust
// Define style constants in styles/theme.rs
pub const COLOR_PRIMARY: &str = "#4F46E5";
pub const COLOR_SUCCESS: &str = "#10B981";
pub const COLOR_ERROR: &str = "#EF4444";
pub const SPACING_SM: &str = "0.5rem";
pub const SPACING_MD: &str = "1rem";
pub const SPACING_LG: &str = "1.5rem";

// Use in components
view! {
    <div style=format!(
        "padding: {}; background: {};",
        SPACING_MD, COLOR_PRIMARY
    )>
        // Content
    </div>
}
```

---

## Development Workflow

### Adding a New Component

1. Create file in appropriate `components/` subdirectory
2. Define component with clear, minimal props
3. Export from parent `mod.rs`
4. Write basic usage example in doc comment
5. Test on mobile viewport (DevTools responsive mode)

### Adding a New Language

1. Add variant to `Language` enum in `core/language.rs`
2. Implement all trait methods for the new variant
3. Create `data/{language}/` directory structure
4. Add vocabulary and grammar content files
5. Update content loader to include new language
6. Test language selection and content display

### Adding a New Exercise Type

1. Define exercise variant in `core/exercise_types.rs`
2. Create corresponding UI component in `components/exercise/`
3. Implement validation logic
4. Add to exercise rotation logic
5. Test thoroughly on mobile

---

## Best Practices

### Performance

- Use `Memo` for expensive computations
- Avoid unnecessary clones (use `move` closures wisely)
- Keep component trees shallow
- Batch state updates when possible

### Accessibility

- Use semantic HTML elements
- Add ARIA labels where needed
- Ensure sufficient color contrast (WCAG AA minimum)
- Support keyboard navigation (even on mobile browsers)

### Content Design

- Keep exercises short (30-60 seconds each)
- Provide immediate feedback
- Use progressive difficulty
- Include variety in exercise types

### Error Handling

- Graceful degradation for missing content
- Clear user-facing error messages
- Use `Result` and `Option` appropriately
- Log errors to console for debugging

---

## Testing Strategy

- **Manual Testing**: Primary method, use mobile DevTools
- **Target Devices**: Test on actual iOS/Android if possible
- **Orientations**: Portrait (primary), landscape (functional)
- **Content Validation**: Ensure all language content is accurate

---

## Build and Deployment

```bash
# Development server with hot reload
trunk serve --open

# Production build
trunk build --release

# Output: dist/ directory ready for static hosting
# Deploy to: Netlify, Vercel, GitHub Pages, Cloudflare Pages, etc.
```

---

## Getting Started Checklist

When starting a new feature:

- [ ] Is this component reusable or page-specific?
- [ ] What props does it need? (Keep them minimal)
- [ ] Does it need local state or context?
- [ ] Is the file in the right directory?
- [ ] Is the file size reasonable (<300 lines)?
- [ ] Does it work on mobile viewport?
- [ ] Is it language-agnostic (when applicable)?

---

## Future Considerations

- **Progressive Web App (PWA)**: Add service worker for offline capability
- **Spaced Repetition**: Implement SRS algorithm (without persistence)
- **Audio Pronunciation**: Add pronunciation guides
- **Animations**: Smooth transitions between exercises
- **Gamification**: Streaks, scores, achievements (session-only)

---

## Questions Before Implementation?

Before implementing each feature, ask:

1. Does this align with the stateless principle?
2. Is this component small and focused?
3. Will this work well on mobile?
4. Is this language-extensible?
5. Can this be reused elsewhere?

---

**Remember**: Start small, iterate quickly, and maintain clean separation of concerns. This is a learning app - keep the codebase as learnable as the content it teaches!
