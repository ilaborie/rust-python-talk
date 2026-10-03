+++
title = "Tips"
classes = ["no_title", "spread-steps"]
+++

# Trucs et astuces

<!-- pause -->

- Réduire la surface d'interop
- Batcher, un appel pour 1 M d'éléments, pas 1 M d'appels
- [`tool.uv.cache-keys`](https://docs.astral.sh/uv/concepts/cache/#dynamic-metadata) sur `src/**/*.rs` : `uv run` recompile tout seul
- `py.detach(...)` sur les sections CPU-bound et les I/O bloquantes si on `block_on`
- Tester des deux côtés avec `pytest` et `cargo test`
- [`py-spy`](https://github.com/benfred/py-spy): le flamegraph peut voir les frames Rust
- Faire attention aux mesures de performance: `--release`

<!-- notes -->

- Surface FFI : ~µs par appel, négligeable à l'unité, mortel dans une boucle chaude
- Batch : c'est LA règle qui fait la différence entre « 50× plus rapide » et « plus lent qu'avant »
- OnceLock : un runtime par instance de classe, c'est un runtime de trop
- `py.detach` = l'ancien `allow_threads` : pendant un compress() ou un GET, les autres threads Python respirent
- py-spy : `py-spy record -o profile.svg -- python script.py`, aucune instrumentation à ajouter
- `cache-keys` : par défaut uv ne regarde que `pyproject.toml` (et l'apparition d'un dossier `src`), donc il réinstalle sa wheel en cache et écrase le `maturin develop`. Avec `[tool.uv] cache-keys = [{ file = "pyproject.toml" }, { file = "Cargo.toml" }, { file = "src/**/*.rs" }]`, `uv run` recompile dès qu'un `.rs` bouge
- Plus besoin de `maturin-import-hook` ni de `maturin develop` à la main
- Boucle de dev façon Python pur : on édite le `.rs`, on relance le script, c'est tout
- Le dernier point est celui que je regrette le plus de ne pas avoir fait dès le départ
