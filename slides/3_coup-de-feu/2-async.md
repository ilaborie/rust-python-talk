+++
title = "Rust async, Python sync"
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

# Rust async, Python sync

```mermaid:width=78%,rankSpacing=34,alt=next() envoie POST /api/command, le serveur applique et répond avec l'état ; le socket pousse en plus ce que font les autres clients
flowchart LR
    PY["Python<br/>next()"]
    SRV["serveur<br/>applique · diffuse"]
    ST["l'état appliqué<br/>state ✓"]
    WS["socket · tokio<br/>tâche de fond"]
    PY -- "POST /api/command" --> SRV
    SRV -- "réponse" --> ST
    SRV -.-> WS
    WS -. "ce que font les autres" .-> ST
    style PY fill:#4B7F52,stroke:#36603C,color:#ffffff
    style SRV fill:#B13F15,stroke:#8a3010,color:#ffffff
```

```rust
#[pyclass]
pub struct Toboggan {
    rt: Runtime,                 // le runtime tokio vit dans la classe
    api: TobogganApi,            // la commande : un aller-retour REST
    state: Arc<RwLock<TState>>,  // l'état, poussé par le socket
}
```

<!-- pause -->

→ Premier jet : un `send` sur un canal. `next()` rendait la main **avant** que
l'état ait bougé. **L'abstraction fuit.**

<!-- pause -->

→ Le correctif : `POST /api/command` **renvoie** l'état qu'il a appliqué.

<!-- notes -->

- Choix : PAS de `pyo3-async-runtimes`. Un runtime tokio dans la struct, `block_on` dans les méthodes
- L'utilisateur Python ne fait jamais `await` — pour un public data/notebook, c'est le bon défaut
- Le getter : `Ok(State(self.rt.block_on(async { TState::clone(&*state.read().await) })))`
- Ne JAMAIS rendre un `RwLockReadGuard` à Python : on clone la valeur et on relâche le lock
- LA fuite : `next()` = `tx.send(cmd)`, non bloquant. L'état n'arrive qu'un aller-retour socket plus tard
- Mesuré : `next()` puis `state`, 24 lectures sur 24 renvoyaient l'état d'avant. Pas « parfois », toujours
- Le pansement : un `sleep(1)` dans `example.py`. Honnête, mais c'est une dette
- Le vrai correctif : le serveur avait déjà `POST /api/command`, qui répond avec l'état appliqué — zéro appelant
- Bonus : une commande refusée lève enfin (`PermissionError` pour l'audience) au lieu de ne rien faire
- Et `py.detach()` autour du `block_on`, sinon on tient le GIL pendant tout l'aller-retour
