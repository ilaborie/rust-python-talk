+++
title = "Propager la trace"
classes = ["no_title"]
+++

<style>
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

# Propager la trace

```mermaid:width=88%,alt=Le contexte de trace ne traverse pas la frontière PyO3
flowchart LR
    PY["Python · otel-python<br/>trace_id ABC · span_id 001"]
    FF["frontière PyO3"]
    RS["Rust · opentelemetry-rust<br/>trace_id ? · span_id ?"]
    PY --> FF
    FF -. "contexte perdu · deux TLS distincts" .-> RS
    style PY fill:#4B7F52,stroke:#36603C,color:#ffffff
    style RS fill:#B13F15,stroke:#8a3010,color:#ffffff
```

<!-- pause -->

Solution : sérialiser le **W3C TraceContext** à la frontière.

```text
Python : propagate.inject(carrier)  →  {"traceparent": "00-ABC-001-01"}
Rust   : Extractor  →  reconstruit le Context côté opentelemetry-rust
```

<!-- notes -->

- Les deux libs OTel ont leur propre Thread-Local Storage : elles ne se voient pas
- Symptôme en prod : deux traces orphelines au lieu d'une, impossible de corréler
- La solution est standard mais **manuelle** : injecter côté Python, extraire côté Rust, à chaque frontière
- Coût réel : c'est du code de plomberie à écrire et à maintenir sur chaque méthode instrumentée
- Même problème pour les logs : `tracing` côté Rust n'atterrit pas dans le `logging` Python sans pont explicite
- TODO ORATEUR : dire si vous avez fini par instrumenter tout ou seulement les entrées/sorties
