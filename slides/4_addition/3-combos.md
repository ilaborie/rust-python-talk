+++
title = "Les combos du menu"
classes = ["no_title"]
+++

# Les combos du menu

<!-- pause -->

1. **Le moteur** — garder l'API Python, remplacer le cœur par du Rust
   _[Polars](https://pola.rs/), [pydantic-core](https://github.com/pydantic/pydantic-core), [Ruff](https://github.com/astral-sh/ruff)_

<!-- pause -->

2. **L'API polyglotte** — un cœur Rust, exposé partout
   _[OpenDAL](https://github.com/apache/opendal) · [xberg](https://github.com/xberg-io/xberg) : 1 core, 15 bindings_

<!-- pause -->

3. **Proto Python → Rust** — itérer vite, optimiser quand le besoin apparaît

<!-- pause -->

4. 🤔 **A-t-on encore besoin de Python ?**
   _`polars`, `candle`, [`evcxr`](https://github.com/evcxr/evcxr)… mais l'écosystème data reste là-bas_

<!-- notes -->

- Combo 1 : le plus courant, le moins risqué, le gain est ciblé et mesurable
- Combo 2 : le plus ambitieux — demande une archi en couches dès le début (cf. toboggan)
- xberg : extraction documentaire, un core Rust, 15 bindings générés, pas maintenus à la main
- Combo 3 : stratégie de migration, pas d'architecture — assumer la réécriture
- Combo 4 : provoc de fin. Réponse honnête : pas encore, à cause de numpy/torch/sklearn
- Il n'y a pas de bonne réponse universelle : ça dépend de qui consomme la lib
