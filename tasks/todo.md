# Présentation Rust & Python — révision 45 minutes

## Plan validé

- [x] Créer un workspace jj isolé depuis `main` et le bookmark `improve-presentation-45min`.
- [x] Annoncer le résultat attendu avant les détails d'ABI et de configuration.
- [x] Caler le déroulé sur 45 minutes : introduction 6, live-code 15, production 13, conclusion 6, questions 5.
- [x] Séparer cohérence de l'état, GIL et intégration asyncio ; rendre les deux REX visibles.
- [x] Introduire l'observabilité par deux traces séparées puis une trace corrélée.
- [x] Ajouter des checkpoints et un repli au live-code ; corriger ABI, signature et profiling.
- [x] Terminer par trois règles : profiler, batcher, détacher les longs traitements Rust.
- [x] Vérifier lint, rendu, format Rust, compilation, wheel et contrat Python.
- [x] Relire le diff, décrire le changement jj et pousser uniquement le bookmark dédié.

## Revue

- `toboggan lint` et le build HTML passent sans diagnostic.
- 30 vignettes photographiées avec Chrome ont été relues ; les nouveaux écrans tiennent sans rognage.
- Le PDF tient sur 30 pages après avoir compacté le contrat async/sync de la slide `toboggan-py`.
- `cargo fmt --check`, `cargo check --locked`, Clippy avec `-D warnings` et `cargo test --locked` passent sur `demo/solution`.
- Wheels debug et release `cp39-abi3` construites ; le contrat Python 3.9 a été vérifié pour le rendu, GFM, la signature, la docstring et la `TypeError`.
- Le workspace original et ses modifications locales sont restés intacts.
