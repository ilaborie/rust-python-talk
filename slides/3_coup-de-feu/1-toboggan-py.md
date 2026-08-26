+++
title = "Un cas réel : toboggan-py"
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

# Un cas réel : `toboggan-py`

```mermaid:width=64%,rankSpacing=44,alt=Un cœur Rust exposé en CLI · Web · Mobile · Python
flowchart TD
    CORE["toboggan-core<br/>logique métier · no_std capable"]
    CLIENT["toboggan-client<br/>HTTP + WebSocket · tokio"]
    CORE --> CLIENT
    CLIENT --> CLI["CLI"]
    CLIENT --> WEB["Web"]
    CLIENT --> MOB["Mobile"]
    CLIENT --> BIND["toboggan-py<br/>PyO3 · cdylib abi3-py38"]
    style CORE fill:#B13F15,stroke:#8a3010,color:#ffffff
    style BIND fill:#4B7F52,stroke:#36603C,color:#ffffff
```

<!-- pause -->

→ **Le même cœur métier**, exposé en CLI, Web, Mobile, Python.

<!-- pause -->

> [!IMPORTANT]
> API Rust _async_<br>
> API Python _sync_

<!-- notes -->

- Démo possible : `from toboggan_py import Toboggan; tbg.next()` fait avancer CETTE slide
- Architecture en couches : le core ne sait rien de Python, c'est la couche bindings qui adapte
- Contenu de `toboggan-py/src/` : `lib.rs` (`#[pymodule]`), `toboggan.rs` (`#[pyclass] Toboggan`), puis `talk.rs`, `slides.rs`, `state.rs`
- Les stubs `toboggan_py.pyi` sont à côté, écrits à la main — on y revient dans « Ce qui coince »
- `#[pyclass]` = newtypes sur les types du core → zéro modèle de données dupliqué
- toboggan-core est no_std capable : la même logique tourne sur un ESP32 pour piloter les slides
- Pattern réutilisable pour exposer aussi en WASM, Swift, Kotlin (UniFFI)
