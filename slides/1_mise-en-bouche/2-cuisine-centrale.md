+++
title = "La cuisine centrale"
classes = ["no_title"]
+++

# La cuisine centrale

Tout le monde parle **C** en sortie de compilation.

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

**Ce qui ne passe pas le comptoir :** noms manglés (`extern "C"`),
layout des structs (`repr(C)`), chaînes (`\0` vs `(ptr, len)`).

<!-- notes -->

- ABI ≠ API : c'est la convention *binaire*, au niveau des registres
- Schéma = System V AMD64 (Linux/macOS) ; Windows x64 est différent (rcx, rdx, r8, r9 + shadow space)
- L'ABI C est le plus petit dénominateur commun de l'industrie depuis 50 ans
- Les 3 frictions : c'est exactement ce que PyO3 va nous éviter d'écrire à la main
- Ne pas s'attarder : 45 s, c'est du contexte, pas le sujet
