+++
title = "Tips"
classes = ["no_title", "spread-steps"]
+++

# Tips de terrain

<!-- pause -->

- 🎯 **Réduire la surface FFI** — moins de franchissements = moins d'overhead
- 📦 **Batcher** plutôt que appels unitaires
- ♻️ **Runtime Tokio partagé** — `OnceLock<Runtime>`
- 🔓 **Libérer le GIL** sur le CPU-bound — `py.allow_threads(...)`
- ✅ **Tester des deux côtés** — `pytest` ET `cargo test`
- 🔍 **Profiler avec [`py-spy`](https://github.com/benfred/py-spy)** — voit les frames natives
- 📝 **Type stubs `.pyi`** — IDE et mypy heureux

<!-- notes -->

- Surface FFI : chaque appel coûte ~µs, c'est petit mais ça s'accumule
- Batch : 1 appel pour 1M items >> 1M appels pour 1 item
- OnceLock : un seul Runtime pour toute la lib
- allow_threads : pendant un compress() ou un calcul lourd, libérer le GIL
- pytest pour valider l'API Python, cargo test pour la logique Rust pure
- py-spy : flame graph mixte Python + Rust, magique pour debugger les perfs
- .pyi : sans ça, l'IDE Python ne sait rien des types exposés par Rust
