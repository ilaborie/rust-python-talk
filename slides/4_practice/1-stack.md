+++
title = "La stack"
classes = ["no_title", "center"]
+++

# La stack

## [PyO3](https://pyo3.rs/) + [Maturin](https://www.maturin.rs/)

<!-- pause -->

- **PyO3** : bindings Rust ↔ Python (les macros)
- **Maturin** : build & packaging des wheels

<!-- pause -->

```bash
$ maturin develop          # dev local
$ maturin build --release  # produit une wheel
```

<!-- notes -->

- PyO3 fait les bindings, Maturin fait le packaging
- Les deux sont les standards de fait depuis ~2020
- `maturin develop` : compile + installe dans le venv courant
- `maturin build` : produit une wheel distribuable (manylinux, musllinux, …)
