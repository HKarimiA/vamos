# Vocabulary Content

Content data for the **Vocabulary** feature: common Spanish words grouped into thematic units, each with 20 word/example pairs translated into English and German.

## File structure

```
translations/vocabulary/
  {unit}/
    es.json   # Source vocabulary (Spanish) — canonical, authored first
    en.json   # English translation of es.json
    de.json   # German translation of es.json
```

- `unit` ranges from `1` to `33`.
- Each `{lang}.json` file is an array of exactly 20 objects: `{ "id": number, "word": string, "example": string }`.
- `id` is a global running index across all units (unit 1 = ids 1-20, unit 2 = ids 21-40, etc.) — unlike the Expressions feature, ids are NOT reset per unit.
- The three files for a given unit are positionally aligned: entry `id: N` in `es.json`, `en.json`, and `de.json` are translations of the same word and example sentence.
- Spanish/German nouns include gender markers where relevant (e.g. `"el/la amigo/a"`, `"der Freund / die Freundin"`); adjectives use the `/a` or `/o` masculine-feminine shorthand (e.g. `"bueno/a"`).

## Translation direction

Spanish (`es.json`) is the source of truth. Each word and its example sentence were authored first in Spanish as common, everyday vocabulary, then translated into English (`en.json`) and German (`de.json`) — never the other way around. Translations favor natural equivalents over literal word-for-word renderings where needed (e.g. regional variants like `"la computadora"` (Latin America) vs. `"el ordenador"` (Spain) are both explicitly labeled in the English/German translations).

## Units and categories

| Unit | Category | CEFR Level |
|------|----------|------------|
| 1 | Basic Essentials (nouns, core verbs, adjectives) | A1 |
| 2 | Family & Core Verbs | A1 |
| 3 | Education & Communication | A1-A2 |
| 4 | Food & Shopping | A1-A2 |
| 5 | Time & Temporal Expressions | A1-A2 |
| 6 | Body & Health | A1-A2 |
| 7 | Colors & Nature/Weather | A1-A2 |
| 8 | Transportation & Travel | A1-A2 |
| 9 | Emotions & Feelings | A1-A2 |
| 10 | Quantifiers & Common Words | A1 |
| 11 | Clothing & Appearance | A1-A2 |
| 12 | House & Home Furniture/Rooms | A1-A2 |
| 13 | Leisure & Entertainment Activities | A1-A2 |
| 14 | Work & Business | A1-A2 |
| 15 | Communication & Learning Verbs | A2-B1 |
| 16 | Weather, Climate & Seasons | A1-A2 |
| 17 | Animals & Nature | A1-A2 |
| 18 | Travel & Transportation | A1-A2 |
| 19 | Shopping & Commerce | A1-A2 |
| 20 | School & Education | A1-A2 |
| 21 | Technology & Computing | A2-B1 |
| 22 | Professions & Jobs | B1-B2 |
| 23 | Personality & Character Traits | B1-B2 |
| 24 | Sports & Fitness | B1-B2 |
| 25 | Music, Arts & Entertainment | B1-B2 |
| 26 | Kitchen & Cooking Utensils | B1-B2 |
| 27 | Money & Finance | B1-B2 |
| 28 | City & Urban Life | B1-B2 |
| 29 | Countries, Nationalities & Languages | B1-B2 |
| 30 | IT, Computer & Internet | B1-B2 |
| 31 | Environment & Ecology | B1-B2 |
| 32 | Days, Months & Calendar | B1-B2 |
| 33 | Prepositions & Connectors | B1-B2 |

Units 1-21 are the original, foundational A1-B1 vocabulary set. Units 22-33 were added later to extend coverage into more advanced, everyday B1-B2 vocabulary (professions, personality, finance, IT, etc.), including some intentional thematic overlap with earlier units (e.g. unit 30 expands on the basic computer terms already introduced in unit 21).

## Verification pass (level & direction)

All 21 original units (420 Spanish word/example pairs and their English/German translations) were reviewed against two criteria:

1. **CEFR level A1-B2**: The Spanish word must be understandable to a beginner-to-intermediate learner, matching its assigned category level.
2. **Translation direction**: Every English/German word and example sentence must be a translation *of* the Spanish entry (same id), not an independently authored equivalent that merely fits the same theme.

No translation-fidelity issues were found in units 1-21 — all word/example pairs are semantically aligned, correctly directional, and use consistent gender articles in German. The same verification approach (direction + level check) was applied when authoring the new units 22-33.
