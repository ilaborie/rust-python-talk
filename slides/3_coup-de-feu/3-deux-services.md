+++
title = "Deux façons de servir"
classes = ["no_title"]
+++

# Deux façons de servir

```rust
// 1 — Runtime interne, dans #[pymethods]. Python : client.fetch(url)
fn fetch(&self, url: &str) -> PyResult<String> {
    self.rt.block_on(async { reqwest::get(url).await?.text().await })
        .map_err(|e| PyRuntimeError::new_err(e.to_string()))
}
```

<!-- pause -->

```rust
// 2 — pyo3-async-runtimes. Python : await fetch(url)
#[pyfunction]
fn fetch(py: Python<'_>, url: String) -> PyResult<Bound<'_, PyAny>> {
    pyo3_async_runtimes::tokio::future_into_py(py, async move {
        let rsp = reqwest::get(&url).await.map_err(to_py)?;
        rsp.text().await.map_err(to_py)
    })
}
```

<!-- pause -->

→ Choisir selon **qui** consomme : un notebook, ou un service `asyncio`.

<!-- notes -->

- REX wefox : client d'API pour stocker les résultats d'inférence et les feedbacks
- Une seule implémentation Rust, consommée par les équipes Rust ET les équipes Python
- Côté service Python (FastAPI, asyncio) → pattern 2 obligatoire, sinon on bloque l'event loop
- Côté notebook ou batch → pattern 1, plus simple, moins de surface d'API
- `to_py` = un petit helper maison `fn(impl Display) -> PyErr` ; il n'y a pas de `From<reqwest::Error>` gratuit
- `pyo3-async-runtimes` suit les versions de PyO3 : 0.29 pour 0.29
- Piège du pattern 2 : le GIL doit être relâché pendant les await Rust, sinon deadlock
- TODO ORATEUR : ajouter ici 1 chiffre concret (volumétrie ou latence) pour ancrer le REX
