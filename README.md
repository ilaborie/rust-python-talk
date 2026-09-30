# Rust & Python — Le menu parfait ?

> Qu'est-ce que l'interopérabilité, et comment faire cohabiter Rust et Python en cuisine ? Y a-t-il des pièges ? Au menu : du live-code, du retour d'XP, et l'addition.

**Le deck en ligne** : <https://ilaborie.github.io/rust-python-talk/> ([PDF](https://ilaborie.github.io/rust-python-talk/presentation.pdf) · [toutes les slides d'un coup d'œil](https://ilaborie.github.io/rust-python-talk/overview/overview.html))

Support et démos d'un talk de 45 minutes (dont 15 minutes consacrées au live-code, setup et récapitulatif compris, et 5 minutes de questions) sur [PyO3](https://pyo3.rs/) et [Maturin](https://www.maturin.rs/).

Objectif : créer une fonction Python dont le moteur est écrit en Rust, puis comprendre les pièges de cohérence, de concurrence et d'observabilité à la frontière des deux langages.

| Partie | Durée | Repère |
|---|---:|---:|
| Mise en bouche — motivation et interopérabilité | 6 min | 0–6 min |
| Mise en place — live-code, setup et récapitulatif compris | 15 min | 6–21 min |
| Coup de feu — retours de production | 13 min | 21–34 min |
| Addition — coûts et règles à retenir | 6 min | 34–40 min |
| Questions | 5 min | 40–45 min |

Le conducteur du live-code et son repli sont dans [demo/README.md](demo/README.md). Les estimations automatiques de Toboggan portent sur le texte des slides : elles ne remplacent pas ce minutage, qui inclut les démos et les explications.

Les slides sont écrites en Markdown et rendues avec [Toboggan](https://github.com/ilaborie/toboggan).

## Lancer la présentation

Les tâches sont décrites dans `mise.toml` ([mise](https://mise.jdx.dev/)) :

```bash
mise run build   # génère rust-python.toml et index.html
mise run dev     # sert le deck et recharge à chaque modification
mise run run     # sert le deck sans watch
```

## Le code du live-code

- `demo/` — le point de départ du live-code (`code.rs`, `md2html.py`, `test.md`)
- `demo/solution/` — l'extension `md` terminée : `to_html`, signature, docstring, stub `.pyi`
- `demo/otel/` — observabilité à travers la frontière PyO3 : les logs Rust remontent dans Python via `pyo3-log`, et le contexte de trace OpenTelemetry descend dans Rust (voir `demo/otel/README.md`)

## Liens

- [pyo3.rs](https://pyo3.rs/) · [maturin.rs](https://www.maturin.rs/) · [écosystème PyO3](https://pyo3.rs/v0.29.2/ecosystem.html)
- [pyo3-async-runtimes](https://github.com/PyO3/pyo3-async-runtimes) · [maturin-import-hook](https://github.com/PyO3/maturin-import-hook) · [py-spy](https://github.com/benfred/py-spy)
- Les talks de David Hewitt, mainteneur PyO3 : [PyO3 in depth](https://www.youtube.com/watch?v=UilujdubqVU) · [5 years of Rust in Python](https://www.youtube.com/watch?v=KTQn_PTHNCw)
