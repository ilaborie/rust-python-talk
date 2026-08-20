+++
title = "Rust : exposer du C"
classes = ["no_title"]
+++

# Rust : exposer du C

```rust
#[unsafe(no_mangle)]
pub extern "C" fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[repr(C)]
pub struct Point {
    x: f64,
    y: f64,
}
```

<!-- pause -->

```bash
$ cargo build --release
   → target/release/libmacrate.{so|dylib|dll}
```

<!-- notes -->

- `extern "C"` : convention d'appel C
- `#[no_mangle]` : nom non-mutilé, retrouvable par dlsym
- `repr(C)` : layout C, prévisible
- `crate-type = ["cdylib"]` dans Cargo.toml pour produire la lib partagée
- cbindgen génère automatiquement les .h C correspondants
