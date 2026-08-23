+++
title = "Rust async, Python sync"
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

# Rust async, Python sync

```rust
#[pyclass]
pub struct Toboggan {
    rt: Runtime,                 // le runtime tokio
    api: TobogganApi,
    state: Arc<RwLock<TState>>,  // l'état
}
```

> [!WARNING]
> `block_on` dans chaque `#[pymethods]` — **Python n'écrit jamais `await`.**

<!-- notes -->

- Choix : PAS de `pyo3-async-runtimes`. Un runtime tokio dans la struct, `block_on` dans les méthodes
- Le runtime est construit une fois dans `__new__` et vit aussi longtemps que l'objet Python
- L'utilisateur Python ne fait jamais `await` — pour un public data/notebook, c'est le bon défaut
- Le getter : `Ok(State(self.rt.block_on(async { TState::clone(&*state.read().await) })))`
- Ne JAMAIS rendre un `RwLockReadGuard` à Python : on clone la valeur et on relâche le lock
- LA fuite : `next()` = `tx.send(cmd)`, non bloquant. L'état n'arrive qu'un aller-retour socket plus tard
- Mesuré : `next()` puis `state`, 24 lectures sur 24 renvoyaient l'état d'avant. Pas « parfois », toujours
- Le pansement : un `sleep(1)` dans `example.py`. Honnête, mais c'est une dette
- Le vrai correctif : le serveur avait déjà `POST /api/command`, qui répond avec l'état appliqué — zéro appelant
- Le socket garde le job que lui seul peut faire : ce que font les AUTRES clients, et les rechargements de deck
- `cache(notif)` : l'état est écrit AVANT que la méthode rende la main — c'est toute la différence
- Ce `block_on` a encore un défaut, et c'est la slide suivante
