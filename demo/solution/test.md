# Demo pyo3

Exemple de code

```rust
#[pyo3::pymodule]
mod pyo3_example {
    use pyo3::prelude::*;

    /// Formats the sum of two numbers as string.
    #[pyfunction]
    fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
        Ok((a + b).to_string())
    }
}
```

See https://pyo3.rs
