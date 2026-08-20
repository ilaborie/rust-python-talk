+++
title = "Ownership"
classes = ["no_title"]
+++

# Ownership : Python ou Rust ?

```text
   Python heap (gc, refcount)
 ┌─────────────────────────────┐
 │  ┌──────────┐               │
 │  │ PyObject │  refcount: 2  │
 │  └─────▲────┘               │
 │        │                    │
 └────────┼────────────────────┘
          │  Py<PyAny>  (incrémente refcount)
 ┌────────┼────────────────────┐
 │  Rust  │                    │
 │        │  struct MyClass {  │
 │        └─ obj: Py<PyAny>,   │
 │        }                    │
 └─────────────────────────────┘
```

<!-- pause -->

- `Py<T>` = smart pointer Rust qui maintient une refcount Python
- GIL nécessaire pour **lire/écrire** un PyObject
- ⚠️ Pas de cycle Rust ↔ Python → fuite garantie

<!-- notes -->

- Le GC Python ne voit pas dans le tas Rust → il ne peut pas casser un cycle
- Bonne pratique : pas de back-reference Python depuis Rust si évitable
- `Bound<'py, T>` : référence avec lifetime, plus moderne que `Py<T>`
- `Python::with_gil(|py| { ... })` : la porte d'entrée du GIL côté Rust
