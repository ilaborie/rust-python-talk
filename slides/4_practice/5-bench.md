+++
title = "Bench"
classes = ["no_title"]
+++

# Bench Python vs Rust vs PyO3

```python
import statistics, timeit
from log_analyze import stats as rust_stats

def py_stats(xs):
    return (statistics.mean(xs), min(xs),
            max(xs), statistics.stdev(xs))

xs = [...]  # 1M de mesures
print("py:  ", timeit.timeit(lambda: py_stats(xs), number=10))
print("rust:", timeit.timeit(lambda: rust_stats(xs), number=10))
```

<!-- pause -->

À mesurer :
- `Vec<f64>` (copie) vs `&PyList` (zero-copy)
- Lire le log en Rust vs en Python
- Coût FFI s'amortit sur le volume

<!-- notes -->

- ORDRE DE GRANDEUR ATTENDU : ~10-50× plus rapide en Rust sur 1M points
- Le coût d'un appel FFI est non-nul (~µs) — donc batcher les appels
- Insister sur "1 appel pour 1M points >> 1M appels pour 1 point chacun"
- Si on fait la lecture du log en Rust aussi, gain encore plus important
