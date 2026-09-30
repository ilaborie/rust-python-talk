+++
title = "Et l'event loop Python ?"
classes = ["no_title", "dense-code"]
+++

# Et l'event loop Python ?

`detach` libère les autres threads ; **`await` laisse avancer la boucle asyncio**.

```rust
#[pyfunction]
fn fetch(py: Python<'_>, url: String) -> PyResult<Bound<'_, PyAny>> {
    pyo3_async_runtimes::tokio::future_into_py(py, async move {
        let response = reqwest::get(&url).await.map_err(to_py)?;
        response.text().await.map_err(to_py)
    })
}
```

<!-- pause -->

```python
body = await fetch(url)
```

→ Choisir le contrat Python : **appel synchrone** ou **awaitable**.

<!-- notes -->

- 1 min 30. Notebook ou batch synchrone : runtime Rust interne + `detach` peut suffire
- Consommateur asyncio : exposer un awaitable avec `pyo3-async-runtimes`, ou déporter une API synchrone sur un thread avec `asyncio.to_thread`. Ne pas appeler `block_on` directement sur le thread de l'event loop
- `future_into_py` adapte le future tokio en awaitable Python ; la requête réseau laisse la boucle Python continuer
- `to_py` est un helper maison qui convertit l'erreur en `PyErr`, pas une conversion automatique de reqwest
- Cet extrait illustre l'adaptateur, il ne fait pas partie du module markdown de la démo
- Les versions de `pyo3-async-runtimes` et de PyO3 doivent être compatibles ; ici 0.29
- Guide : https://pyo3.rs/v0.29.2/async-await
