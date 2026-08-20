+++
title = "Ce qu'on ne peut pas (facilement)"
classes = ["no_title"]
+++

# Ce qu'on ne peut pas (facilement)

```rust
// ❌ Génériques exposés tels quels
#[pyclass]
struct Container<T> { /* ... */ }   // ne compile pas
```

<!-- pause -->

```rust
// ✅ Solution : monomorphiser
#[pyclass] struct IntContainer { /* ... */ }
#[pyclass] struct StrContainer { /* ... */ }
```

<!-- pause -->

```rust
// ❌ Iterator Rust → itérable Python (pas auto)
impl Iterator for MyType { /* ... */ }

// ✅ Implémenter __iter__ et __next__
#[pymethods]
impl MyType {
    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> { slf }
    fn __next__(&mut self) -> Option<i32> { /* ... */ }
}
```

<!-- notes -->

- Génériques : Python ne connaît pas, il faut monomorphiser
- Traits Rust ≠ protocoles Python — il faut implémenter explicitement
- Héritage Python ↔ Rust : limité (un seul niveau)
- Lifetimes : pas exposables, mais PyO3 gère pour vous via `Bound<'py, T>`
