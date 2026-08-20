+++
title = "Python : appeler du natif"
classes = ["no_title"]
+++

# Python : appeler du natif

```python
import ctypes
lib = ctypes.CDLL("./libmath.so")
lib.add.argtypes = [ctypes.c_int, ctypes.c_int]
lib.add.restype = ctypes.c_int
print(lib.add(2, 3))  # 5
```

<!-- pause -->

| Approche | Pour | Contre |
|---|---|---|
| `ctypes` | Pas de compilation | Lent, peu sûr |
| `cffi` | Plus propre | Dépendance |
| **C-API CPython** | Perf max, intégration native | C, tout à la main |
| **PyO3** | Ergonomie + perf C-API | Compile Rust |

<!-- notes -->

- ctypes : standard library, marche partout, mais parsing à chaque appel
- cffi : plus structuré, supporte ABI mode (= ctypes) ou API mode (= compilation)
- C-API : `PyObject*`, refcount manuel, le vrai mécanisme natif
- PyO3 utilise la C-API en arrière-plan, mais cache toute la complexité
