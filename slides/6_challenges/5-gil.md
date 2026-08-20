+++
title = "Le GIL et la PEP 703"
classes = ["no_title"]
+++

# Le GIL et la PEP 703

Le **G**lobal **I**nterpreter **L**ock :

- 1 thread Python à la fois exécute du bytecode
- Empêche le vrai parallélisme CPU-bound en pur Python
- Workaround : multiprocessing, ou du natif qui le relâche

<!-- pause -->

**[PEP 703](https://peps.python.org/pep-0703/)** — Free-threaded Python (3.13+)

- Le GIL devient **optionnel**
- Compile flag `--disable-gil`
- Vrai parallélisme CPU-bound

<!-- pause -->

→ Les libs natives (PyO3 inclus) doivent s'adapter.

<!-- notes -->

- Le GIL est LE principal frein à la perf en pur Python
- PEP 703 acceptée en 2023, en cours de déploiement
- Rust est bien placé : `Send + Sync` natif, on sait gérer le parallélisme
- À surveiller : compatibilité free-threaded en cours pour les grosses libs
- En attendant : `py.allow_threads(...)` pour relâcher le GIL pendant le Rust pur
