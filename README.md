# Rust & Python — Le menu parfait ?

> Qu'est-ce que l'interopérabilité, et comment faire cohabiter Rust et Python en cuisine ? Y a-t-il des pièges ? Au menu : du live-code, du retour d'XP, et l'addition.

Support et démos d'un talk d'environ 30 minutes (dont ~15 minutes de live-code) sur [PyO3](https://pyo3.rs/) et [Maturin](https://www.maturin.rs/).

Les slides sont écrites en Markdown et rendues avec [Toboggan](https://github.com/ilaborie/toboggan).

## Lancer la présentation

Les tâches sont décrites dans `mise.toml` ([mise](https://mise.jdx.dev/)) :

```bash
mise run build   # génère rust-python.toml et index.html
mise run dev     # sert le deck et recharge à chaque modification
mise run run     # sert le deck sans watch
```

`bacon` (voir `bacon.toml`) rebuild le deck en continu pendant l'écriture.

## Le code du live-code

- `demo/` — le point de départ du live-code (`code.rs`, `md2html.py`, `test.md`)
- `demo/solution/` — l'extension `md` terminée : `to_html`, signature, docstring, stub `.pyi`
- `demo/otel/` — observabilité à travers la frontière PyO3 : les logs Rust remontent dans Python via `pyo3-log`, et le contexte de trace OpenTelemetry descend dans Rust (voir `demo/otel/README.md`)

## Structure du dépôt

```
slides/       le deck, un dossier par chapitre, un fichier Markdown par slide
public/       les images servies par toboggan-server
demo/         le code du live-code et les démos
```

## Références

- [pyo3.rs](https://pyo3.rs/) · [maturin.rs](https://www.maturin.rs/) · [écosystème PyO3](https://pyo3.rs/v0.29.2/ecosystem.html)
- [pyo3-async-runtimes](https://github.com/PyO3/pyo3-async-runtimes) · [maturin-import-hook](https://github.com/PyO3/maturin-import-hook) · [py-spy](https://github.com/benfred/py-spy)
- Les talks de David Hewitt, mainteneur PyO3 : [PyO3 in depth](https://www.youtube.com/watch?v=UilujdubqVU) · [5 years of Rust in Python](https://www.youtube.com/watch?v=KTQn_PTHNCw)
