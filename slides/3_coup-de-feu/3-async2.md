+++
title = "Le piège du GIL"
classes = ["no_title", "dense-code"]
+++

<style>
/* Plusieurs blocs Rust et un avant/après sur la même slide : sans ça, le
   titre sort par le haut en 1920×1080. Même réglage que « Le résultat ». */

/* Titre collé en haut : la marge négative le sort en partie du `space-evenly`
   de l'article, ce qui rapproche le titre du bord et rend le reste au corps.
   Trois précautions :
   - dupliqué slide par slide, parce que le thème rend le corps dans un shadow
     root que le CSS de `_head.html` ne traverse pas ;
   - les `:not()` parce que l'export statique (et le PDF) met tous ces `<style>`
     dans un seul document : la règle fuirait sur les mises en page qui n'ont
     aucune marge à récupérer (titre centré, `.step` en `flex: 1`, terminal
     plein cadre, code dense) et le titre sortirait par le haut ;
   - -0.5em et pas plus : au-delà, « Le constat » et « Deux façons d'appeler »
     débordent par le haut. Mesuré sur /run, de 1280×800 à 3840×2160. */
section:not(.center):not(.spread-steps):not(.fourneaux):not(.dense-code) > article > h1 {
	margin-block: -0.5em;
}
</style>

# Attention au GIL

```rust
// `block_on` verrouille le GIL pendant les I/O
let notif = self.rt.block_on(self.api.command(cmd))?;
```

<!-- pause -->

```rust
// `detach` relache le lock pendant les I/O
let notif = py.detach(|| self.rt.block_on(self.api.command(cmd)))?;
```

<!-- pause -->

```rust
// L'autre choix avec pyo3-async-runtimes
// En Python: await fetch(url)
#[pyfunction]
fn fetch(py: Python<'_>, url: String) -> PyResult<Bound<'_, PyAny>> {
    pyo3_async_runtimes::tokio::future_into_py(py, async move {
        let rsp = reqwest::get(&url).await.map_err(to_py)?;
        rsp.text().await.map_err(to_py)
    })
}
```

<!-- pause -->

[pyo3.rs/v0.29.3/async-await](https://pyo3.rs/v0.29.3/async-await)

<!-- notes -->

`attach` / `detach` : en 0.29 (avant `with_gil` / `allow_threads`)
