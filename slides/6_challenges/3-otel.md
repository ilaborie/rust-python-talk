+++
title = "OpenTelemetry"
classes = ["no_title"]
+++

# OpenTelemetry

```text
Python (otel-python)             Rust (opentelemetry-rust)
────────────────────             ─────────────────────────
trace_id: ABC                    trace_id: ???
span_id:  001                    span_id:  ???
   │                                 │
   │ ─── PyO3 call ──▶               │
   │                                 │   ❌ contexte perdu
   │ ◄── return ───                  │
```

<!-- pause -->

Solution : injecter / extraire **W3C TraceContext** aux frontières.

```text
Python : propagate.inject(carrier)
       → {"traceparent": "00-ABC-001-01"}
Rust   : Extractor reconstruit le Context
```

<!-- notes -->

- Les 2 libs OTel ont leur propre Thread-Local Storage → contexte non partagé
- Solution standard : sérialiser/désérialiser le contexte au passage FFI
- W3C TraceContext = standard interop (header `traceparent`)
- C'est manuel, c'est pénible — mais ça marche
- Écosystème en évolution, des helpers émergent
