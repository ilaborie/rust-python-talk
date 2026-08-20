+++
title = "Exemple 3 — Python dans Rust"
classes = ["no_title"]
+++

# Exemple 3 — Python dans Rust

```rust
use pyo3::prelude::*;

fn main() -> PyResult<()> {
    Python::with_gil(|py| {
        let math = py.import("math")?;
        let pi: f64 = math.getattr("pi")?.extract()?;
        println!("π = {pi}");

        py.run(c"print('Hello from embedded Python')",
               None, None)?;
        Ok(())
    })
}
```

<!-- pause -->

```toml
pyo3 = { version = "0.28", features = ["auto-initialize"] }
```

<!-- notes -->

- L'inverse : un binaire Rust qui embarque un interpréteur Python
- `Python::with_gil` : on acquiert le GIL pour parler à Python
- `py.import` / `getattr` / `extract` : on navigue dans l'écosystème Python
- `py.run` : on exécute du code Python arbitraire
- Cas d'usage : plugins, scripting utilisateur, piloter du ML
- `auto-initialize` : démarre l'interpréteur Python automatiquement
