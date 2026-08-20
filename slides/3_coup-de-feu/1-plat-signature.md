+++
title = "Le plat signature"
classes = ["no_title"]
+++

# Le plat signature

Cette présentation est pilotée par `toboggan-py`.

```text
toboggan-core/    ← logique métier pure (no_std capable)
toboggan-client/  ← client HTTP + WebSocket, async (tokio)
toboggan-py/      ← bindings PyO3 (cdylib, abi3-py38)
   └─ src/
      ├─ lib.rs        #[pymodule]
      ├─ toboggan.rs   #[pyclass] Toboggan
      └─ talk.rs, slides.rs, state.rs, …
toboggan_py.pyi   ← type stubs (IDE, mypy)
```

<!-- pause -->

→ **Le même cœur métier**, exposé en CLI, Web, Mobile, Python.

<!-- notes -->

- Démo possible : `from toboggan_py import Toboggan; tbg.next()` fait avancer CETTE slide
- Architecture en couches : le core ne sait rien de Python, c'est la couche bindings qui adapte
- `#[pyclass]` = newtypes sur les types du core → zéro modèle de données dupliqué
- toboggan-core est no_std capable : la même logique tourne sur un ESP32 pour piloter les slides
- Pattern réutilisable pour exposer aussi en WASM, Swift, Kotlin (UniFFI)
