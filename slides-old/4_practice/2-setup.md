+++
title = "Setup minimal"
classes = ["no_title"]
+++

# Setup minimal

```toml
# Cargo.toml
[lib]
crate-type = ["cdylib"]

[dependencies]
pyo3 = { version = "0.28", features = ["abi3-py38"] }
```

<!-- pause -->

```toml
# pyproject.toml
[build-system]
requires = ["maturin>=1.0"]
build-backend = "maturin"
```

<!-- pause -->

→ C'est tout.

<!-- notes -->

- `cdylib` : on produit une lib dynamique, pas un binaire
- `abi3-py38` : ABI stable Python 3.8+ → une seule wheel pour toutes les versions Python
- Sans abi3 : il faut compiler une wheel par version Python (3.10, 3.11, 3.12, …)
- pyproject.toml : 3 lignes, c'est Maturin qui gère tout le reste
