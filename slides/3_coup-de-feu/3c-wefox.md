+++
title = "Chez wefox : un client, deux équipes"
classes = ["no_title"]
+++

<style>
section:not(.center):not(.spread-steps):not(.fourneaux):not(.dense-code) > article > h1 { margin-block: -0.5em; }
</style>

# Un client, deux équipes

**Chez wefox** : stocker les résultats d'inférence et les feedbacks.

```mermaid:width=78%,alt=Le même client Rust est utilisé par les équipes Rust et par les équipes Python via PyO3
flowchart LR
    RS["Équipe Rust"] --> CLIENT["Client Rust async"]
    PY["Équipe Python"] --> BIND["PyO3"] --> CLIENT
    CLIENT --> API["API résultats<br/>et feedbacks"]
    style CLIENT fill:#B13F15,stroke:#8a3010,color:#ffffff
    style PY fill:#4B7F52,stroke:#36603C,color:#ffffff
```

<!-- pause -->

→ **Une implémentation partagée**, un contrat adapté à chaque consommateur.

<!-- notes -->

- 1 min. C'est le deuxième REX annoncé : une même implémentation Rust utilisée par les équipes Rust et Python
- Le bénéfice documenté est la mutualisation du client, pas un chiffre de performance
- Rattacher ce cas au choix précédent : adapter une API async Rust au modèle d'exécution de l'appelant Python
- La frontière est petite, mais les erreurs et le contexte d'observabilité doivent aussi la traverser
