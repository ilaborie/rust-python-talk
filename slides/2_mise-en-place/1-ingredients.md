+++
title = "Deux fichiers, et c'est tout"
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

# Deux fichiers, et c'est tout

```toml
# Cargo.toml
[lib]
crate-type = ["cdylib"]

[dependencies]
pyo3 = { version = "0.29", features = ["abi3-py38"] }
```

<!-- pause -->

```toml
# pyproject.toml
[build-system]
requires = ["maturin>=1.9,<2.0"]
build-backend = "maturin"
```

<!-- notes -->

- `cdylib` : on produit une bibliothèque dynamique, pas un binaire
- `abi3-py38` : ABI stable → **une seule wheel** pour Python 3.8+
- Sans abi3 : une wheel par version de Python (3.10, 3.11, 3.12, 3.13, 3.14…)
- pyproject.toml : 3 lignes, Maturin gère le reste
- Ces deux fichiers, c'est tout ce que `maturin new -b pyo3` génère en plus du `src/lib.rs`
