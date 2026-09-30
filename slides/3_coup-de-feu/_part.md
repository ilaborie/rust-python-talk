+++
title = "Le coup de feu"
classes = ["no_title", "center"]
+++

# Le coup de feu

## Deux projets, en production

<!-- notes -->

- REX sur deux projets réels : toboggan-py (ces slides !) et un client d'inférence chez wefox
- Le fil rouge : du Rust **async** exposé à du Python qui, lui, est **synchrone**
- 13 min, de la minute 21 à la minute 34 : cohérence de l'état → GIL → asyncio → client partagé → observabilité
- Montrer les symptômes avant le correctif ; les sorties OTel servent à vérifier les ponts, pas à introduire un nouveau live-code
