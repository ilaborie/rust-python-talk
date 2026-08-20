+++
title = "Les #[pymethods]"
classes = ["no_title"]
+++

# Les `#[pymethods]`

```rust
#[pymethods]
impl Toboggan {
    #[new]
    #[pyo3(signature = (host = "localhost", port = 8080))]
    pub fn __new__(host: &str, port: u16) -> PyResult<Self> {
        // ... setup tokio runtime, websocket, channels
    }

    #[getter]
    pub fn state(&self) -> PyResult<State> {
        let state = Arc::clone(&self.state);
        let s = self.rt.block_on(async {
            state.read().await.clone()
        });
        Ok(State(s))
    }

    pub fn next(&self) { self.send(Command::NextSlide); }
    pub fn previous(&self) { self.send(Command::PreviousSlide); }
}
```

<!-- notes -->

- `#[new]` : le constructeur Python
- `#[pyo3(signature = ...)]` : args nommés et défauts à la Python
- `#[getter]` : devient un attribut Python (`tbg.state` au lieu de `tbg.state()`)
- `block_on` : passerelle sync→async — c'est ici que Python attend Rust
- `next()`/`previous()` : juste un envoi sur un channel, retour immédiat
