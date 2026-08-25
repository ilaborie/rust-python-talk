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
   — dupliqué slide par slide, parce que le thème rend le corps dans un shadow
     root que le CSS de `_head.html` ne traverse pas ;
   — les `:not()` parce que l'export statique (et le PDF) met tous ces `<style>`
     dans un seul document : la règle fuirait sur les mises en page qui n'ont
     aucune marge à récupérer (titre centré, `.step` en `flex: 1`, terminal
     plein cadre, code dense) et le titre sortirait par le haut ;
   — -0.5em et pas plus : au-delà, « Le constat » et « Deux façons d'appeler »
     débordent par le haut. Mesuré sur /run, de 1280×800 à 3840×2160. */
section:not(.center):not(.spread-steps):not(.fourneaux):not(.dense-code) > article > h1 {
	margin-block: -0.5em;
}
</style>

# Le piège du GIL

```rust
// `block_on` garde le GIL pendant tout l'aller-retour réseau
let notif = self.rt.block_on(self.api.command(cmd))?;
```

<!-- pause -->

```rust
// `detach` le rend le temps du réseau, et le reprend après
let notif = py.detach(|| self.rt.block_on(self.api.command(cmd)))?;
```

> [!NOTE]
> `attach` / `detach` : en 0.29 (avant `with_gil` / `allow_threads`)

<!-- pause -->

```rust
// L'autre voie : pyo3-async-runtimes. Côté Python : await fetch(url)
#[pyfunction]
fn fetch(py: Python<'_>, url: String) -> PyResult<Bound<'_, PyAny>> {
    pyo3_async_runtimes::tokio::future_into_py(py, async move {
        let rsp = reqwest::get(&url).await.map_err(to_py)?;
        rsp.text().await.map_err(to_py)
    })
}
```

<!-- pause -->

Le guide : [pyo3.rs/v0.29.2/async-await](https://pyo3.rs/v0.29.2/async-await)

<!-- notes -->

- Le bug réel : `Toboggan(...)` sur un serveur qui ne répond pas gelait TOUT l'interpréteur
- GIL tenu = même le thread watchdog ne tourne plus : pas de timeout, pas de Ctrl-C, on tue le REPL
- Invisible en local : un aller-retour à 2 ms ne se distingue pas d'un GIL relâché. Il faut un serveur lent pour le voir
- Le test qui l'attrape tourne dans un process fils : en in-process il n'échoue pas, il fige la session
- `py.detach(|| ...)` : la closure doit être `Send` — d'où le `handle` cloné plutôt que `&self`
- `Python::attach` / `py.detach` remplacent `with_gil` / `allow_threads` — vus en partie 2, ici ils servent
- ATTENTION : la moitié des tutos en ligne sont encore en `with_gil`
- REX wefox : client d'API pour stocker les résultats d'inférence et les feedbacks
- Une seule implémentation Rust, consommée par les équipes Rust ET les équipes Python
- Côté service Python (FastAPI, asyncio) → pattern 2 obligatoire, sinon on bloque l'event loop
- Côté notebook ou batch → pattern 1, plus simple, moins de surface d'API
- `to_py` = un petit helper maison `fn(impl Display) -> PyErr` ; il n'y a pas de `From<reqwest::Error>` gratuit
- `pyo3-async-runtimes` suit les versions de PyO3 : 0.29 pour 0.29
- TODO ORATEUR : ajouter ici 1 chiffre concret (volumétrie ou latence) pour ancrer le REX
