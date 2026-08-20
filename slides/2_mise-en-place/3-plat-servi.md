+++
title = "Le plat servi"
classes = ["no_title", "dense-code"]
+++

<style>
/* Seule exception du deck : 17 lignes de Rust + un REPL sur la même slide.
   Le style vit dans le shadow tree de la slide, il ne fuit pas ailleurs. */
section.dense-code { --slide-code-size: 54%; }
</style>

# Le plat servi

<!-- code:rust:demo/solution/src/lib.rs -->

<!-- pause -->

```python
>>> markdown_to_html(42)
TypeError: 'int' object is not an instance of 'str'
while processing 'input'
```

<!-- notes -->

- Ce code EST celui du live-code : la slide l'embarque depuis `demo/solution/src/lib.rs`, il ne peut pas mentir
- 17 lignes. Aucune ligne de FFI, aucun `unsafe`, aucun header C
- `&str` ← `str`, `String` → `str`, `bool` ↔ `bool` : les conversions sont générées
- Le cas nominal, on vient de le voir en live — ici on montre ce qu'on n'a PAS écrit : la validation
- `signature = (input, *, ...)` : le `*` produit un keyword-only argument Python
- La `TypeError` est offerte : le typage Rust devient une erreur Python idiomatique, avec le nom du paramètre
- Le `///` Rust devient la docstring — `help()` fonctionne
