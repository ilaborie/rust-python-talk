+++
title = "Le contexte aussi"
classes = ["no_title"]
+++

<style>
/* Plusieurs blocs Rust et un avant/après sur la même slide : sans ça, le
   titre sort par le haut en 1920×1080. Même réglage que « Le résultat ». */

/* Titre collé en haut : la marge négative le sort en partie du `space-evenly`
   de l'article, ce qui rapproche le titre du bord et rend le reste au corps.
   Trois précautions :
   — dupliqué slide par slide, parce que le thème rend le corps dans un shadow
     root que le CSS de `_head.html` ne traverse pas ;
   — les `:not()` parce que l'export statique (et le PDF) met tous ces `<style>`
     dans un seul document : la règle fuirait sur les mises en page qui n'ont
     aucune marge à récupérer (titre centré, `.step` en `flex: 1`, terminal
     plein cadre, code dense) et le titre sortirait par le haut ;
   — -0.5em et pas plus : au-delà, « Le constat » et « Deux façons d'appeler »
     débordent par le haut. Mesuré sur /run, de 1280×800 à 3840×2160. */
section:not(.center):not(.spread-steps):not(.fourneaux):not(.dense-code) > article > h1 {
	margin-block: -0.5em;
}
</style>

# Le contexte OTel

```rust
// C'est Rust qui appelle Python pour savoir où il en est
fn python_context(py: Python<'_>) -> PyResult<Context> {
    let carrier = PyDict::new(py);
    py.import("opentelemetry.propagate")?
        .call_method1("inject", (&carrier,))?;  // {"traceparent": "00-6a79…-01"}

    let carrier = carrier.extract::<HashMap<String, String>>()?;
    let context = global::get_text_map_propagator(|prop| prop.extract(&carrier));
    Ok(context)
}
```

<!-- pause -->

```rust
let span = tracing::info_span!("render", bytes = input.len());
let parent = python_context(py).unwrap_or_default();  // pas d'OTel → racine
if let Err(err) = span.set_parent(parent) {           // OpenTelemetrySpanExt
    tracing::warn!("contexte Python non rattaché : {err}");
}
```

<!-- pause -->

```text
Rust    render          TraceId  6a79…5545  ParentSpanId 86c1602bdc1c1aca
Python  handle-request  trace_id 6a79…5545  span_id      86c1602bdc1c1aca
```

→ Une seule trace, et **rien à changer côté Python**.
· [pyo3.rs/v0.29.2/ecosystem/tracing](https://pyo3.rs/v0.29.2/ecosystem/tracing.html)

<!-- notes -->

- L'idée : plutôt que de demander à l'appelant Python de passer un carrier, c'est Rust qui va le chercher
- `propagate.inject(dict)` est l'API standard d'OTel Python — celle qui sert à poser un en-tête HTTP
- `HashMap<String, String>` implémente déjà `Extractor` côté opentelemetry-rust : rien à écrire
- Si OTel est absent côté Python, l'import échoue → `unwrap_or_default()`, on repart d'une racine : le module reste utilisable sans OTel
- `set_parent` vient de `tracing_opentelemetry::OpenTelemetrySpanExt` ; il renvoie un `Result` (erreur si la layer OTel n'est pas installée)
- Coût : 3 lignes par méthode instrumentée. Sur une API large, ça se sent — on a instrumenté les entrées/sorties, pas tout
- L'alternative : `pyo3-python-tracing-subscriber` — Rust délègue à une `Layer` écrite en Python, donc zéro propagation manuelle, mais tout passe par le GIL
- Tout le code de ces 2 slides tourne dans `demo/otel/` : `maturin develop` puis `opentelemetry-instrument … python app.py`
