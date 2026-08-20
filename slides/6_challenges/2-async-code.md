+++
title = "Patterns async"
classes = ["no_title"]
+++

# Deux patterns

```rust
// Pattern 1 : Runtime interne, API Python sync
#[pymethods]
impl Client {
    fn fetch(&self, url: &str) -> PyResult<String> {
        self.rt.block_on(async {
            reqwest::get(url).await?.text().await
        }).map_err(|e| PyRuntimeError::new_err(e.to_string()))
    }
}
```

<!-- pause -->

```rust
// Pattern 2 : pyo3-async-runtimes, API Python async
#[pyfunction]
fn fetch(py: Python<'_>, url: String) -> PyResult<Bound<'_, PyAny>> {
    pyo3_async_runtimes::tokio::future_into_py(py, async move {
        reqwest::get(&url).await
            .map_err(|e| PyRuntimeError::new_err(e.to_string()))?
            .text().await
            .map_err(|e| PyRuntimeError::new_err(e.to_string()))
    })
}
```

<!-- notes -->

- Pattern 1 : utilisateur Python ne fait pas d'`await`, c'est sync — toboggan-py
- Pattern 2 : utilisateur Python fait `await fetch(...)`, c'est async
- Pattern 1 plus simple, Pattern 2 plus pythonique pour les libs async
- Choisir selon le public : data scientist → sync, dev async → pattern 2
