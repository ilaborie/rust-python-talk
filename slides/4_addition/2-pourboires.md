+++
title = "Pourboires"
classes = ["no_title", "spread-steps"]
+++

# Pourboires

<!-- pause -->

- 🎯 **Réduire la surface FFI** — chaque franchissement se paie
- 📦 **Batcher** — un appel pour 1 M d'éléments, pas 1 M d'appels
- ♻️ **Un seul runtime tokio** — `static RT: OnceLock<Runtime>`
- 🔓 **`py.detach(...)`** sur les sections CPU-bound et les I/O bloquantes
- ✅ **Tester des deux côtés** — `pytest` *et* `cargo test`
- 🔍 **[`py-spy`](https://github.com/benfred/py-spy)** — le flame graph voit les frames Rust
- 🤖 **Générer les stubs** plutôt que les écrire

<!-- notes -->

- Surface FFI : ~µs par appel, négligeable à l'unité, mortel dans une boucle chaude
- Batch : c'est LA règle qui fait la différence entre « 50× plus rapide » et « plus lent qu'avant »
- OnceLock : un runtime par instance de classe, c'est un runtime de trop
- `py.detach` = l'ancien `allow_threads` : pendant un compress() ou un GET, les autres threads Python respirent
- py-spy : `py-spy record -o profile.svg -- python script.py`, aucune instrumentation à ajouter
- Le dernier point est celui que je regrette le plus de ne pas avoir fait dès le départ
