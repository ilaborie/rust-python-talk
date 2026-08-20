+++
title = "Quand ça vaut le coup"
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

# Quand ça vaut le coup

<!-- pause -->

1. **Le moteur** — garder l'API Python, remplacer le cœur par du Rust
   _[Polars](https://pola.rs/) · [pydantic-core](https://github.com/pydantic/pydantic-core)_
<!-- pause -->

2. **L'API polyglotte** — un cœur Rust, exposé partout
   _[OpenDAL](https://github.com/apache/opendal) · [xberg](https://github.com/xberg-io/xberg) : 1 core, 15 bindings_

<!-- pause -->

3. **Proto Python → Rust** — itérer vite, optimiser quand le besoin apparaît

<!-- notes -->

- Cas 1 : le plus courant, le moins risqué, le gain est ciblé et mesurable
- Cas 2 : le plus ambitieux — demande une archi en couches dès le début (cf. toboggan)
- xberg : extraction documentaire, un core Rust, 15 bindings générés, pas maintenus à la main
- Cas 3 : stratégie de migration, pas d'architecture — assumer la réécriture
- Cas 4 : provoc de fin. Réponse honnête : pas encore, à cause de numpy/torch/sklearn
- Il n'y a pas de bonne réponse universelle : ça dépend de qui consomme la lib
