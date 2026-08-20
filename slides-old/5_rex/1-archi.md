+++
title = "Architecture"
classes = ["no_title"]
+++

# Architecture toboggan-py

```text
toboggan-core/    ← logique métier pure (no_std capable)
toboggan-client/  ← client HTTP/WebSocket Rust
toboggan-py/      ← bindings PyO3 (cdylib, abi3-py38)
   └─ src/
      ├─ lib.rs         #[pymodule]
      ├─ toboggan.rs    #[pyclass] Toboggan
      ├─ talk.rs, slides.rs, state.rs, ...
toboggan_py.pyi   ← type stubs (IDE, mypy)
```

<!-- pause -->

→ **Le même cœur métier**, exposé en CLI, Web, Mobile, Python.

<!-- notes -->

- Architecture en couches : core (pur) → client (réseau) → bindings (langage)
- Le core ne sait rien de Python — c'est le bindings qui adapte
- Pattern réutilisable pour exposer aussi en WASM, Swift (UniFFI), Kotlin…
- toboggan-core : no_std capable = peut tourner sur ESP32 (objet connecté pour piloter les slides)
