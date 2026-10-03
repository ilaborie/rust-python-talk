+++
title = "Chez Wefox AI: une API client, deux équipes"
classes = ["no_title"]
+++

<style>
section:not(.center):not(.spread-steps):not(.fourneaux):not(.dense-code) > article > h1 { margin-block: -0.5em; }
</style>

# Un client, deux équipes

Chez Wefox AI: micro service pour stocker les résultats d'inférence et leurs feedback

```mermaid:width=78%,alt=Le même client Rust est utilisé par les équipes Rust et par les équipes Python via PyO3
flowchart LR
    RS["Team API (Rust)"] --> CLIENT_RUST["Crate Rust"]
    PY["Team ML (Python)"] --> CLIENT_PY["Lib Python"] --> BIND["PyO3"] --> CLIENT_RUST
    CLIENT_RUST --> API["API résultats inférence<br/>et feedbacks"]
    style CLIENT_RUST fill:#B13F15,stroke:#8a3010,color:#ffffff
    style CLIENT_PY fill:#4B7F52,stroke:#36603C,color:#ffffff
```

<!-- pause -->

Implémentation partagée, un contrat adapté à chaque équipe, on masque le détail d'implémentation

<!-- notes -->

- 1 min. C'est le deuxième REX annoncé : une même implémentation Rust utilisée par les équipes Rust et Python
- Le bénéfice documenté est la mutualisation du client, pas un chiffre de performance
- Rattacher ce cas au choix précédent : adapter une API async Rust au modèle d'exécution de l'appelant Python
- La frontière est petite, mais les erreurs et le contexte d'observabilité doivent aussi la traverser
