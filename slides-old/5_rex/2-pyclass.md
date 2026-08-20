+++
title = "Le #[pyclass]"
classes = ["no_title"]
+++

# Le `#[pyclass]`

```rust
#[pyclass]
pub struct Toboggan {
    config: TobogganConfig,
    rt: Runtime,                       // Tokio embarqué
    _ws: WebSocketClient,
    tx: UnboundedSender<Command>,      // canal vers la tâche WS
    talk: Arc<RwLock<TalkResponse>>,   // état partagé
    slides: Arc<RwLock<SlidesResponse>>,
    state: Arc<RwLock<TState>>,
}
```

<!-- pause -->

Le **pattern clé** :

> **Python sync, Rust async**, Runtime Tokio embarqué.

<!-- notes -->

- `Runtime` : on instancie tokio UNE fois dans la struct
- `Arc<RwLock<...>>` : état partagé entre la tâche WebSocket et les méthodes
- `mpsc` : la méthode `next()` envoie un Command, la tâche WS le traite
- Python ne voit qu'une API sync — toute l'asynchronie est cachée
