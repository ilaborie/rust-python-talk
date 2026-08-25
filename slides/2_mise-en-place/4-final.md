+++
title = "Le résultat"
classes = ["no_title", "dense-code"]
+++

<style>
/* 16 lignes de Rust + un REPL sur la même slide. Deux slides de « Le coup
   de feu » utilisent le même réglage, pour la même raison.
   Le style vit dans le shadow tree de la slide, il ne fuit pas ailleurs. */

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

# Le résultat

<!-- code:rust:slides/2_mise-en-place/result.rs -->

<!-- notes -->

- C'est le résultat du live-code, resserré pour tenir sur la slide : la version qui compile est `demo/solution/src/lib.rs`, et ce que la slide embarque est `slides/2_mise-en-place/result.rs`, une copie tenue à la main — les `use`, les options `gfm` et la coloration syntaxique passent à la trappe (les deux `// ...`). À resynchroniser si le live-code change
- 16 lignes. Aucune ligne de FFI, aucun `unsafe`, aucun header C
- `&str` ← `str`, `String` → `str`, `bool` ↔ `bool` : les conversions sont générées
- Le cas nominal, on vient de le voir en live — ici on montre ce qu'on n'a PAS écrit : la validation
- `signature = (input, *, ...)` : le `*` produit un keyword-only argument Python
- La `TypeError` est offerte : le typage Rust devient une erreur Python idiomatique, avec le nom du paramètre
- Le `///` Rust devient la docstring — `help()` fonctionne
