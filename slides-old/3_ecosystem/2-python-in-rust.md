+++
title = "Python dans Rust"
classes = ["no_title"]
+++

# 🐍 dans 🦀

L'inverse existe aussi :

<!-- pause -->

- **Scripting / plugins** : embarquer un interpréteur Python pour la config, l'extension
- **ML/IA** : piloter PyTorch / scikit-learn depuis un service Rust
- **Migration progressive** : remplacer brique par brique sans tout réécrire
- **Outillage** : exposer du tooling Python (linters, formatters) à un binaire Rust

<!-- pause -->

→ Moins courant, mais bien supporté par PyO3.

<!-- notes -->

- Cas d'usage moins médiatisé mais réel
- Embarquer Python = `Python::with_gil` dans le binaire Rust
- Migration progressive : pattern "strangler fig" — façade Rust autour du Python existant
- Exemple : un serveur Rust qui appelle un modèle Python pour l'inférence
