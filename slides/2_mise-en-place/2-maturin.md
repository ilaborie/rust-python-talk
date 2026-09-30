+++
title = "De la source à la wheel"
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

# De la source à la wheel

```mermaid:width=90%,nodeSpacing=34,alt=maturin compile les sources et produit soit un venv prêt à l'emploi soit une wheel
flowchart LR
    SRC["src/lib.rs<br/>Cargo.toml · pyproject.toml"]
    MAT["maturin"]
    VENV["le venv<br/>import direct, boucle de dev"]
    WHL["wheel abi3 par plateforme / architecture<br/>CPython 3.9+ avec GIL"]
    SRC --> MAT
    MAT --> DEV(["develop"]) --> VENV
    MAT --> BLD(["build --release"]) --> WHL
    style MAT fill:#B13F15,stroke:#8a3010,color:#ffffff
    style VENV fill:#4B7F52,stroke:#36603C,color:#ffffff
    style WHL fill:#4B7F52,stroke:#36603C,color:#ffffff
```

<!-- pause -->

```bash
$ maturin develop          # compile + installe dans le venv
$ maturin build --release  # produit une wheel
```

<!-- notes -->

- `maturin develop` = la boucle de dev ; c'est la commande qu'on va marteler pendant le live-code
- Elle compile ET installe dans le venv actif : pas de `pip install` à faire derrière
- `maturin build --release` produit la wheel : c'est elle qu'on publie, l'utilisateur n'a pas besoin de Rust
- `abi3-py39` est une option activée dans notre Cargo.toml, pas le comportement automatique de Maturin
- Pour CPython avec GIL : une wheel par plateforme / architecture peut couvrir les versions compatibles à partir de 3.9 ; sans `abi3`, les wheels dépendent aussi de la version de Python
- Les builds CPython sans GIL ont besoin de leurs propres wheels compatibles ; la promesse affichée ne les couvre pas
- Le CI type, c'est `maturin build` sur chaque plateforme + `maturin publish`
