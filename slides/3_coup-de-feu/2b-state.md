+++
title = "Appel terminé, état à jour ?"
classes = ["no_title"]
+++

<style>
section:not(.center):not(.spread-steps):not(.fourneaux):not(.dense-code) > article > h1 { margin-block: -0.5em; }
</style>

# Appel terminé, état à jour ?

```python
tbg.next()
tbg.state   # encore la slide précédente : 24 lectures sur 24
```

<!-- pause -->

**Avant** : commande envoyée par WebSocket → retour immédiat → état reçu plus tard.

<!-- pause -->

**Après** : `POST /api/command` → état appliqué → cache mis à jour → retour Python.

→ Une méthode synchrone doit tenir sa **promesse de cohérence**.

<!-- notes -->

- 1 min. Le résultat 24/24 vient des lectures observées sur toboggan-py, pas d'un benchmark général
- `next()` envoyait la commande avec `tx.send(cmd)` : « envoyé » n'était pas « appliqué »
- Le `sleep(1)` dans `example.py` masquait le problème sans garantir la cohérence
- Le serveur avait déjà `POST /api/command`, qui renvoie l'état appliqué. Mettre cet état en cache avant de rendre la main règle le contrat
- Le WebSocket reste utile pour les commandes des autres clients et les rechargements du deck
- Cette attente réseau résout la cohérence, mais révèle un autre problème : le GIL
