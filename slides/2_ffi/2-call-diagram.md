+++
title = "Schéma d'un appel C"
classes = ["no_title"]
+++

# Schéma d'un appel C

System V AMD64 (Linux/macOS) :

```text
   Caller                                 Callee
 ┌─────────────────────┐                ┌─────────────────────┐
 │ rdi, rsi, rdx,      │ ─ 6 args int ▶ │                     │
 │ rcx,  r8, r9        │                │  exécute            │
 │ xmm0..xmm7          │ ─ flottants ─▶ │                     │
 │ pile (au-delà)      │                │  rax = résultat     │
 │                     │ ─── call ───▶  │  ret                │
 │ rax ← retour        │ ◄── return ──  │                     │
 └─────────────────────┘                └─────────────────────┘
```

<!-- pause -->

**Frictions :**
- Name mangling (`extern "C"`)
- Layout (`repr(C)`)
- Strings : `\0` vs `(ptr, len)`

<!-- notes -->

- ABI Linux/macOS : System V AMD64
- ABI Windows : différent (rcx, rdx, r8, r9 + shadow space)
- Rust par défaut mutile les noms (modules, generics) → besoin de `#[no_mangle]`
- `repr(C)` fige l'ordre des champs et le padding (sinon Rust optimise)
- Les chaînes : C utilise `\0` final, Rust passe (ptr, len) — incompatible
