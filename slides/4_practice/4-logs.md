+++
title = "Exemple 2 — perf logs"
classes = ["no_title"]
+++

# Exemple 2 — Stats sur logs

```rust
#[pyfunction]
fn stats(durations_ms: Vec<f64>) -> (f64, f64, f64, f64) {
    let n = durations_ms.len() as f64;
    let sum: f64 = durations_ms.iter().sum();
    let avg = sum / n;
    let min = durations_ms.iter().copied()
        .fold(f64::INFINITY, f64::min);
    let max = durations_ms.iter().copied()
        .fold(f64::NEG_INFINITY, f64::max);
    let var = durations_ms.iter()
        .map(|x| (x - avg).powi(2))
        .sum::<f64>() / n;
    (avg, min, max, var.sqrt())
}
```

<!-- notes -->

- Cas perf : avg, min, max, écart-type sur des temps de réponse
- `Vec<f64>` : PyO3 convertit auto une `list[float]` Python en `Vec<f64>` Rust
- Coût : la conversion fait une copie — à mesurer
- Variantes : `&PyList` zero-copy, lecture du log directement en Rust
