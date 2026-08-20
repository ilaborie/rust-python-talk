+++
title = "Suivre la commande"
classes = ["no_title"]
+++

# Suivre la commande

```text
Python (otel-python)             Rust (opentelemetry-rust)
────────────────────             ─────────────────────────
trace_id: ABC                    trace_id: ???
span_id:  001                    span_id:  ???
   │                                 │
   │ ─── appel PyO3 ──▶              │   ❌ contexte perdu
   │ ◄──── retour ────               │      (deux TLS distincts)
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
