# Vamos

Vamos is a mobile-first Spanish learning web app focused on quick vocabulary and grammar practice.

Live website: https://vamos.rf.gd/

## Purpose

- Help users practice Spanish vocabulary and grammar in short sessions
- Provide a simple phone-friendly learning experience
- Keep everything client-side (no backend)

## Tech Stack

- Rust (Edition 2024)
- Leptos (CSR)
- Leptos Router
- Trunk
- Serde / Serde JSON
- wasm-bindgen

## Build & Run

Development server:

```bash
trunk serve
```

Production build:

```bash
trunk build --release
```

## Notes

- Designed mainly for phone usage
- Favorites are session-based and reset on page reload
- Content is loaded from static JSON files under `translations/`
