+++
title = "Asynchronisme"
classes = ["no_title"]
+++

# Asynchronisme

```text
   Python (asyncio)              Rust (tokio)
 ┌─────────────────┐          ┌──────────────────┐
 │  await client   │          │  tokio::Runtime  │
 │      .fetch()   │ ─────▶   │   ┌──────────┐   │
 │                 │          │   │ reqwest  │   │
 │  ◄───── Future ─┤          │   └──────────┘   │
 │  (résolue)      │          │                  │
 └─────────────────┘          └──────────────────┘
        ↑                              ↑
        │  pyo3-async-runtimes::tokio  │
        └─────── future_into_py ───────┘
```

<!-- pause -->

Deux mondes async **indépendants** → il faut un pont.

<!-- notes -->

- Tokio et asyncio sont 2 runtimes async qui ne se parlent pas nativement
- `pyo3-async-runtimes` (ex `pyo3-asyncio`) fait le pont
- Piège : il FAUT relâcher le GIL pendant les awaits Rust, sinon deadlock
- Alternative simple : runtime Tokio interne + API Python sync (toboggan-py)
