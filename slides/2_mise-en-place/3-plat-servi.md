+++
title = "Le plat servi"
classes = ["no_title"]
+++

# Le plat servi

<!-- code:rust:demo/solution/src/lib.rs -->

<!-- pause -->

```python
>>> from rusty_md import markdown_to_html
>>> markdown_to_html("# Hello **world**")
'<h1>Hello <strong>world</strong></h1>'
>>> markdown_to_html(42)
TypeError: 'int' object is not an instance of 'str'
while processing 'input'
```

<!-- notes -->

- Ce code EST celui du live-code : la slide l'embarque depuis `demo/solution/src/lib.rs`, il ne peut pas mentir
- 17 lignes. Aucune ligne de FFI, aucun `unsafe`, aucun header C
- `&str` ← `str`, `String` → `str`, `bool` ↔ `bool` : les conversions sont générées
- `signature = (input, *, ...)` : le `*` produit un keyword-only argument Python
- La `TypeError` est offerte : le typage Rust devient une erreur Python idiomatique, avec le nom du paramètre
- Le `///` Rust devient la docstring — `help()` fonctionne
