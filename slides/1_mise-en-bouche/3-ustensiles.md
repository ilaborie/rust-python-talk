+++
title = "Les ustensiles"
classes = ["no_title"]
+++

# Les ustensiles

| Approche | Pour | Contre |
|---|---|---|
| `ctypes` | zéro compilation | lent, aucune sûreté |
| `cffi` | ABI ou API mode | une dépendance de plus |
| C-API | perf maximale | tout à la main, en C |
| **PyO3** | ergonomie Rust + perf C-API | il faut compiler du Rust |

<!-- pause -->

→ L'apport de PyO3, ce n'est pas l'appel. C'est **tout le reste** :
conversions de types, exceptions, docstrings, signatures — **dans les deux sens**.

<!-- notes -->

- ctypes/cffi : on décrit la signature à la main, à chaque appel — et si on se trompe, segfault
- C-API : c'est ce que fait CPython lui-même, c'est ce que PyO3 génère sous le capot
- Le point à faire passer : la FFI brute *marche* depuis toujours, le problème c'est la DX
- « dans les deux sens » : Rust appelé depuis Python ET Python appelé depuis Rust — on verra les deux
- Transition : assez de théorie, on cuisine
