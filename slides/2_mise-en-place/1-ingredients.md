+++
title = "Les ingrédients"
classes = ["no_title"]
+++

# Les ingrédients

```toml
# Cargo.toml
[lib]
crate-type = ["cdylib"]

[dependencies]
pyo3 = { version = "0.29", features = ["abi3-py38"] }
```

<!-- pause -->

```toml
# pyproject.toml
[build-system]
requires = ["maturin>=1.9,<2.0"]
build-backend = "maturin"
```

<!-- pause -->

```bash
$ maturin develop          # compile + installe dans le venv
$ maturin build --release  # produit une wheel
```

<!-- notes -->

- `cdylib` : on produit une bibliothèque dynamique, pas un binaire
- `abi3-py38` : ABI stable → **une seule wheel** pour Python 3.8+
- Sans abi3 : une wheel par version de Python (3.10, 3.11, 3.12, 3.13, 3.14…)
- pyproject.toml : 3 lignes, Maturin gère le reste
- `maturin develop` = la boucle de dev ; c'est la commande qu'on va marteler pendant le live-code
