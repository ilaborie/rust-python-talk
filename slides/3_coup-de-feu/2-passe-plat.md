+++
title = "Le passe-plat asynchrone"
classes = ["no_title"]
+++

# Le passe-plat asynchrone

Rust est **async**. Python, ici, est **synchrone**.

```rust
#[pyclass]
pub struct Toboggan {
    rt: Runtime,                    // le runtime tokio vit dans la classe
    tx: UnboundedSender<Command>,   // canal vers la tâche de fond
    state: Arc<RwLock<TState>>,     // état tenu à jour en tâche de fond
}

#[pymethods]
impl Toboggan {
    #[getter]
    pub fn state(&self) -> PyResult<State> {
        let state = Arc::clone(&self.state);
        Ok(State(self.rt.block_on(async {
            TState::clone(&*state.read().await)   // on clone, on ne prête pas le guard
        })))
    }

    pub fn next(&self) { self.send(Command::NextSlide); }   // fire-and-forget
}
```

<!-- notes -->

- Choix : PAS de `pyo3-async-runtimes`. Un runtime tokio dans la struct, `block_on` dans les méthodes
- L'utilisateur Python ne fait jamais `await` — pour un public data/notebook, c'est le bon défaut
- Ne JAMAIS rendre un `RwLockReadGuard` à Python : on clone la valeur et on relâche
- `next()` n'est même pas bloquant : c'est un `send` sur un canal
- L'ABSTRACTION FUIT : du coup `tbg.next()` puis `tbg.state` peut renvoyer l'ancien état
- Dans `example.py`, il y a un `sleep(1)` entre les deux. C'est honnête, mais c'est une dette
