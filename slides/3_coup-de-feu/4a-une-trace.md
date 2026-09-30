+++
title = "Une requête, une trace"
classes = ["no_title", "data-table"]
+++

<style>
section:not(.center):not(.spread-steps):not(.fourneaux):not(.dense-code) > article > h1 { margin-block: -0.5em; }
section.data-table table { width: 100%; border-collapse: collapse; }
section.data-table th, section.data-table td { padding: 0.3em 0.5em; text-align: inherit; border-bottom: 1px solid #dee2e6; }
section.data-table tbody { border-top: 2px solid currentColor; }
</style>

# Une requête, une trace

```mermaid:width=78%,alt=Le span Rust partage la trace ABC du span Python et indique le span Python comme parent
flowchart LR
    PY["Python · handle-request<br/>trace_id ABC · span_id 001"]
    RS["Rust · render<br/>trace_id ABC · parent_span_id 001"]
    PY -->|"contexte propagé"| RS
    style PY fill:#4B7F52,stroke:#36603C,color:#ffffff
    style RS fill:#B13F15,stroke:#8a3010,color:#ffffff
```

<!-- pause -->

| Pont | Sens | Mécanisme |
|---|---|---|
| Logs | Rust → Python | `pyo3-log` |
| Contexte de trace | Python → Rust | W3C TraceContext |

<!-- notes -->

- 30 s. Une même trace_id et un parent_span_id Rust égal au span_id Python : voilà le résultat à vérifier
- Les deux ponts sont indépendants : avoir les logs ne prouve pas que les spans sont rattachés
- On garde maintenant cet objectif en tête pour lire les deux extraits de code
