+++
title = "Le piège du GIL"
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

# Le piège du GIL

**Serveur muet → interpréteur figé**, même le watchdog Python ne tourne plus.

```rust
// `block_on` garde le GIL pendant tout l'aller-retour réseau
let notif = self.rt.block_on(self.api.command(cmd))?;
```

<!-- pause -->

```rust
// `detach` le rend le temps du réseau, et le reprend après
let notif = py.detach(|| self.rt.block_on(self.api.command(cmd)))?;
```

> [!NOTE]
> `attach` / `detach` : depuis **0.26** (avant `with_gil` / `allow_threads`)

<!-- pause -->

→ Les **autres threads Python** peuvent avancer. L'appelant attend toujours.

<!-- notes -->

- Le bug réel : `Toboggan(...)` sur un serveur qui ne répond pas gelait TOUT l'interpréteur
- GIL tenu = même le thread watchdog ne tourne plus : pas de timeout, pas de Ctrl-C, on tue le REPL
- Invisible en local : un aller-retour à 2 ms ne se distingue pas d'un GIL relâché. Il faut un serveur lent pour le voir
- Le test qui l'attrape tourne dans un process fils : en in-process il n'échoue pas, il fige la session
- `py.detach(|| ...)` : ne capturer que des données utilisables sans attachement à Python, aucun `Bound<PyAny>`
- `Python::attach` / `py.detach` remplacent `with_gil` / `allow_threads` — vus en partie 2, ici ils servent
- ATTENTION : la moitié des tutos en ligne sont encore en `with_gil`
- `detach` ne transforme pas l'appel en coroutine : sur le thread de l'event loop, l'appel synchrone bloque encore la boucle
- 1 min 30. Transition : si le consommateur attend une API asyncio, il faut aussi adapter le contrat d'appel
