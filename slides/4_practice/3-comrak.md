+++
title = "Exemple 1 — comrak"
classes = ["no_title"]
+++

# Exemple 1 — Markdown via comrak

```rust
use pyo3::prelude::*;

#[pyfunction]
fn markdown_to_html(input: &str) -> String {
    comrak::markdown_to_html(input, &comrak::Options::default())
}

#[pymodule]
fn rusty_md(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(markdown_to_html, m)?)?;
    Ok(())
}
```

<!-- pause -->

```python
from rusty_md import markdown_to_html
print(markdown_to_html("# Hello **world**"))
# <h1>Hello <strong>world</strong></h1>
```

<!-- notes -->

- Cas idéal pour démarrer : zéro état partagé, fonction pure
- `#[pyfunction]` : convertit la signature Rust en Python (auto-conversion `&str` ↔ `str`)
- `#[pymodule]` : déclare le module, on ajoute les fonctions/classes
- 8 lignes pour transformer une crate Rust en module Python utilisable
- LIVE-CODE possible si à l'aise — sinon laisser le code à l'écran
