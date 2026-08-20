+++
title = "Le pont"
classes = ["no_title"]
+++

# Le pont

```text
   Python                  ABI C                   Rust
 ┌──────────┐         ┌────────────┐         ┌─────────────┐
 │ PyObject │ ──────▶ │ extern "C" │ ──────▶ │ #[pyclass]  │
 │   .py    │ ◄────── │  no_mangle │ ◄────── │  Rust code  │
 └──────────┘         └────────────┘         └─────────────┘
                       PyO3 / Maturin
                       génèrent ce pont
```

<!-- pause -->

→ **Vous écrivez du Rust idiomatique.**
→ **PyO3 + Maturin font tout le sale boulot.**

<!-- notes -->

- Le développeur n'a (presque) jamais à toucher au C-API
- PyO3 = macros + helpers, Maturin = build + packaging
- Le code Rust reste idiomatique (pas de #[no_mangle] partout)
- Transition vers la section pratique
