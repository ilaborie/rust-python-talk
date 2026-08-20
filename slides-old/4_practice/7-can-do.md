+++
title = "Ce qu'on peut faire"
classes = ["no_title", "spread-steps"]
+++

# Ce qu'on peut faire

<!-- pause -->

- ✅ **Modules** : `#[pymodule]`
- ✅ **Fonctions** : `#[pyfunction]`
- ✅ **Classes** : `#[pyclass]` + `#[pymethods]`
- ✅ **Exceptions custom** : `create_exception!`
- ✅ **Sous-modules**, conversions auto
- ✅ **NumPy** : crate [`rust-numpy`](https://github.com/PyO3/rust-numpy)
- ✅ **Async** : [`pyo3-async-runtimes`](https://github.com/PyO3/pyo3-async-runtimes)
- ✅ **Type stubs** (`.pyi`) pour l'IDE / mypy

<!-- notes -->

- Tout ce qu'un dev Python attend, on peut l'exposer
- Les conversions de types sont automatiques pour les types primitifs et collections
- NumPy : interop zero-copy via `PyArray<T>`
- Async : on peut exposer une fonction Rust `async` comme coroutine Python
