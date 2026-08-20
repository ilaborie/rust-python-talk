+++
title = "ABI C, lingua franca"
classes = ["no_title"]
+++

# ABI C, lingua franca

**ABI** ≠ API
- **A**pplication **B**inary **I**nterface
- Convention de bas niveau : registres, pile, layout mémoire

<!-- pause -->

Presque tous les langages **savent appeler C**.

<!-- pause -->

→ Si Rust expose une fonction `extern "C"`,
   Python (ou n'importe quel langage) peut l'appeler.

<!-- notes -->

- API = contrat au niveau code source
- ABI = contrat au niveau binaire (assembleur, exécutable)
- Le C définit l'ABI standard depuis 50 ans → tout le monde s'aligne
- Rust et Python parlent C → ils peuvent se parler via C
