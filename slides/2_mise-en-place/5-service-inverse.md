+++
title = "Dans l'autre sens"
classes = ["no_title"]
+++

<style>
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

# Dans l'autre sens

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
