+++
title = "Tips"
classes = ["no_title", "spread-steps"]
+++

# Tips

<!-- pause -->

- **Réduire la surface d'interop.**
- **Batcher** — un appel pour 1 M d'éléments, pas 1 M d'appels
- **[`maturin-import-hook`](https://github.com/PyO3/maturin-import-hook)** ou un watcher comme [bacon](https://dystroy.org/bacon/)
- **`py.detach(...)`** sur les sections CPU-bound et les I/O bloquantes si on `block_on`
- **Tester des deux côtés** — `pytest` *et* `cargo test`
- **[`py-spy --native`](https://github.com/benfred/py-spy#can-py-spy-profile-native-extensions)** — frames Rust sur les plateformes prises en charge
- Faire attention aux mesures de performance : `--release`

<!-- notes -->

- Surface FFI : ~µs par appel, négligeable à l'unité, mortel dans une boucle chaude
- Batch : c'est LA règle qui fait la différence entre « 50× plus rapide » et « plus lent qu'avant »
- OnceLock : un runtime par instance de classe, c'est un runtime de trop
- `py.detach` = l'ancien `allow_threads` : pendant un compress() ou un GET, les autres threads Python respirent
- Profiling natif sur Linux pris en charge : `py-spy record --native -o profile.svg -- python script.py`. Garder les symboles dans l'extension pour identifier les fonctions Rust ; `--release` choisit les optimisations, pas la conservation des symboles
- Le mode natif n'est pas pris en charge sur macOS : le profiling Python reste disponible, mais il ne montre pas les frames Rust
- `maturin-import-hook` : `import maturin_import_hook; maturin_import_hook.install()` dans le `sitecustomize.py` du venv, et l'`import` recompile si les sources ont bougé
- Boucle de dev façon Python pur : on édite le `.rs`, on relance le script, c'est tout
- Le dernier point est celui que je regrette le plus de ne pas avoir fait dès le départ
