# Copilot Instructions for Vamos

Vamos is a **client-side, mobile-first Spanish learning app**: Rust + Leptos 0.8 (CSR), bundled with Trunk, no backend/auth/database. All content is static JSON `include_str!()`'d at compile time; runtime state lives only in Leptos signals/context (cleared on refresh).

## Architecture map

```
src/
  main.rs            # Router + <App/> root, provide_context() for FavoritesContext/LanguageContext
  core/               # FavoritesContext, Language enum + LanguageContext (UI language, default German)
  data/               # Content loaders + models, all pub-re-exported from data/mod.rs
    vocabulary.rs      # stages 1..=33, macro-generated include_str! per stage/lang (es/en/de)
    expressions.rs     # units 1..=27, same macro pattern, ExpressionCard{id, phrase} (no example field)
    grammar.rs         # topics 1..=27, GrammarContent (explanation, hint, questions+answers)
    quiz.rs            # dynamic 20-question quiz session generator (see below)
    ui_translations.rs # UiStrings struct, get_ui_strings(Language) loads translations/ui/{en,de}.json
  pages/              # One file per route, #[component] fn matching route name
  components/         # Shared widgets, e.g. vocabulary_card.rs (swipe/reveal card used by 3+ pages)
translations/
  vocabulary/{1..33}/{es,en,de}.json
  expressions/{1..27}/{es,en,de}.json
  grammar/{1..27}/en.json          # NOTE: grammar content is English-only currently
  ui/{en,de}.json                  # UI chrome strings (buttons, labels), keyed by UiStrings field names
```

Routes are defined in `src/main.rs`. **Static routes must be listed before parameterized ones** (e.g. `/vocabulary/favorites` before `/vocabulary/:stage`).

## Adding content (follow existing patterns exactly)

- **New vocabulary stage N**: add `translations/vocabulary/N/{es,en,de}.json`, then add `N` to the stage list literal in `src/data/vocabulary.rs`'s `include_vocabulary_stages!` macro call. Card IDs are global across stages in blocks of 20.
- **New expression unit N**: same pattern in `src/data/expressions.rs` / `translations/expressions/`.
- **New grammar topic**: add `translations/grammar/N/en.json`, register in `get_all_topics()` and the include macro in `src/data/grammar.rs`.
- **New UI string**: add the field to `UiStrings` in `src/data/ui_translations.rs`, then add the matching key to **both** `translations/ui/en.json` and `translations/ui/de.json` — the struct is `Deserialize`d directly from these files, so a missing key breaks JSON parsing for that language at runtime.
- **Quiz feature**: `src/data/quiz.rs` samples from the existing vocabulary/expression/grammar pools (no new content needed) to build a fresh shuffled 20-question session (`build_quiz_session`). Vocabulary/Grammar questions are `MultipleChoice`; Expression questions are `FillInTheBlank`.

## Leptos gotchas learned the hard way in this codebase

1. **`on:click`/`on:input` closures must be `Fn(...) + 'static`, not `FnOnce`.** If a closure needs to own non-`Copy` data (e.g. `Vec<String>`) and is used as an event handler (especially inside `<Show>`/reactive re-render cycles), wrap the data in `StoredValue::new(...)` at the top of the component and access it via `.with_value(|v| ...)` / `.get_value()`. Do **not** just `move` an owned `Vec`/`String` directly into the handler closure. See `grammar_topic.rs`'s `StoredValue::new(content)` and `quiz.rs`'s `FillInBlankCard` for the established pattern.
2. **Deeply nested `view! {}` macros combined with `.into_any()` across multiple conditional branches can produce misleading `E0310` "may not live long enough" errors** that are really an architecture problem, not a lifetime bug. Fix: extract each conditional UI branch into its own separate `#[component] fn` (mirrors `grammar_topic.rs`: `GrammarTopic` → `QuizInterface` → `ResultsScreen`), so Leptos gets concrete, nameable types at each `<Component/>` boundary instead of one giant inferred type.
3. Use `prop:value=` (not plain `value=`) for reactive/controlled `<input>` elements, or typing will only ever show the initial value. Use `event_target_value(&ev)` inside `on:input` to read the new value.
4. `rand = "0.8"`'s `Rng::gen()` collides with the `gen` reserved keyword in **Rust edition 2024** (this crate's edition) — call it as `rng.r#gen::<T>()`.
5. `rand`/`getrandom` in WASM: keep `getrandom = { version = "0.2", features = ["js"] }` alongside `rand = { version = "0.8", features = ["small_rng"] }`; `SmallRng::from_entropy()` is the proven-working RNG source for CSR/WASM here.

## Build & verify

```bash
trunk serve                                 # dev server, http://127.0.0.1:8087 (see Trunk.toml)
cargo check --target wasm32-unknown-unknown # fast type-check, matches actual build target
trunk build --release                       # full production build
```

**PowerShell quirk**: piping `cargo`/`trunk` output through `Select-Object` (or similar) in this environment often prints a spurious `NativeCommandError`/`FullyQualifiedErrorId` wrapper and a nonzero exit code even on success. Don't trust the exit code alone — look for `Finished \`dev\`/\`release\` profile` (cargo) or `INFO success` (trunk) in the output to confirm success.

## Mobile-first UI conventions (this app is phone-only in practice)

- Touch targets ≥44px height, `touch-action: manipulation` on interactive elements.
- Reuse existing CSS classes where possible: `.answer-button`, `.results-container`/`.results-card`, `.page-header`, `.back-button`, `.hint-button-question`.
- Respect existing responsive breakpoints in `styles.css`: `@media (max-width: 375px)` (narrow phones) and `@media (max-height: 600px)` (short screens) — extend these rather than adding new ad-hoc breakpoints.
- `:active` scale/press feedback is the standard pattern instead of `:hover`.

## i18n notes

- `Language` enum: `Spanish` (content being learned, always Spanish — see `LEARNING_LANGUAGE` const), `English`, `German` (UI languages). Default UI language is German (`LanguageContext::new()`).
- Content language codes used across loaders are `"es"`, `"en"`, `"de"` string literals (not the `Language` enum) — grammar content currently only has `"en"` files, so German UI users still see English grammar explanations.
