+++
title = "Le service inversé"
classes = ["no_title"]
+++

# Le service inversé

Un binaire **Rust** qui embarque un interpréteur **Python**.

```rust
fn main() -> PyResult<()> {
    Python::attach(|py| {
        let math = py.import("math")?;
        let pi: f64 = math.getattr("pi")?.extract()?;
        println!("π = {pi}");

        py.run(c"print('Bonjour depuis Python')", None, None)
    })
}
```

<!-- pause -->

```toml
pyo3 = { version = "0.29", features = ["auto-initialize"] }
```

<!-- notes -->

- Même crate, même macros — c'est bidirectionnel par construction
- `Python::attach` : depuis PyO3 0.26, remplace `Python::with_gil` (et `detach` remplace `allow_threads`)
- ATTENTION : la moitié des tutos en ligne sont encore en `with_gil` / `&PyModule`
- Cas d'usage réels : plugins utilisateur, scripting, piloter un écosystème ML (numpy, sklearn) depuis un service Rust
- Aussi : migration progressive d'un projet Python vers Rust, morceau par morceau
- `auto-initialize` : démarre l'interpréteur tout seul — pratique en dev, à éviter en lib
