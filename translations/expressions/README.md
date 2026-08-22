# Expressions Content

Content data for the **Expressions** feature: everyday Spanish expressions and phrases, grouped into 21 thematic units, each with 20 phrases translated into English and German.

## File structure

```
translations/expressions/
  {unit}/
    es.json   # Source phrases (Spanish) — canonical, authored first
    en.json   # English translation of es.json
    de.json   # German translation of es.json
```

- `unit` ranges from `1` to `21`.
- Each `{lang}.json` file is an array of exactly 20 objects: `{ "id": number, "phrase": string }`.
- `id` is a 1-20 index local to the unit (not globally unique across units).
- The three files for a given unit are positionally aligned: entry `id: N` in `es.json`, `en.json`, and `de.json` are translations of the same expression.

## Translation direction

Spanish (`es.json`) is the source of truth. Each phrase was authored first in Spanish as a common, everyday expression, then translated into English (`en.json`) and German (`de.json`) — never the other way around. Translations favor natural, idiomatic equivalents in the target language over literal word-for-word renderings (important for units 18 and 20, which contain idioms/exclamations that rarely translate literally).

## Units and categories

| Unit | Category | Example phrase (es) |
|------|----------|----------------------|
| 1 | Greetings & Introductions | "Hola, ¿qué tal?" |
| 2 | Politeness & Small Talk | "Por favor" / "Gracias" |
| 3 | Asking Questions | "¿Cómo se dice esto en español?" |
| 4 | Café & Restaurant | "La cuenta, por favor" |
| 5 | Shopping | "¿Cuánto cuesta esto?" |
| 6 | Directions | "Gire a la derecha" |
| 7 | Time & Scheduling | "¿A qué hora nos vemos?" |
| 8 | Weather | "Está lloviendo" |
| 9 | Making Plans | "¿Quieres salir esta noche?" |
| 10 | Opinions & Agreement/Disagreement | "Estoy de acuerdo" |
| 11 | Emotions & Feelings | "Estoy feliz" |
| 12 | Family & Relationships | "Esta es mi familia" |
| 13 | Work & School | "Trabajo en una oficina" |
| 14 | Travel | "Voy a viajar a España" |
| 15 | At the Hotel | "Tengo una reserva" |
| 16 | Phone & Technology | "Se me acabó la batería" |
| 17 | Numbers & Money | "¿Cuánto es en total?" |
| 18 | Common Idioms & Sayings | "Está lloviendo a cántaros" |
| 19 | Farewells | "Adiós" / "Hasta luego" |
| 20 | Reactions & Exclamations | "¡Qué sorpresa!" |
| 21 | Everyday Life (Miscellaneous) | "Estoy en camino" |
| 22 | Health & Body | "Me duele la cabeza" |
| 23 | At Home & Household Chores | "Tengo que lavar los platos" |
| 24 | Sports & Hobbies | "Me gusta jugar al fútbol" |
| 25 | Food & Cooking | "Voy a cocinar la cena" |
| 26 | Social Media & Online Life | "Voy a publicar una foto" |
| 27 | Celebrations & Special Occasions | "Feliz cumpleaños" |

## Verification pass (level & direction)

All 21 units (420 Spanish phrases + their English/German translations) were reviewed against two criteria:

1. **CEFR level A1-B2**: The Spanish source phrase must be understandable to a beginner-to-upper-intermediate learner. Units 18 (Common Idioms) and 20 (Reactions & Exclamations) are inherently idiomatic/fixed expressions that don't map cleanly onto CEFR grammar levels; these were given leeway rather than held to a strict cap, since idioms of this kind are commonly taught even to intermediate learners.
2. **Translation direction**: Every English/German phrase must be a translation *of* the Spanish phrase (same id), not an independently authored expression that merely fits the same theme.

### Issues found and corrected

- **Unit 16 (`de.json`)**: id `3` ("Se me acabó la batería") and id `20` ("Estoy sin batería") are distinct Spanish phrases, but both had been translated to the identical German "Mein Akku ist leer". Id `20` was corrected to "Ich habe keinen Akku mehr" to preserve the distinction (mirrored in the English file, which already differentiated "My battery died" vs. "I'm out of battery").
- **Unit 20 (`en.json`)**: id `13` ("¡Claro que sí!") and id `14` ("¡Por supuesto!") are distinct Spanish exclamations, but both had been translated to the identical English "Of course!". Id `13` was corrected to "Definitely!" to preserve the distinction (mirrored in the German file, which already differentiated "Klar doch!" vs. "Natürlich!").

No other level or direction violations were found; all remaining phrases fall within A1-B2 (or are common idioms in units 18/20) and are faithful translations of their Spanish source.
