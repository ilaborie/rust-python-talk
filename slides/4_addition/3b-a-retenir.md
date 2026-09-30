+++
title = "Le menu parfait ?"
classes = ["no_title", "spread-steps"]
+++

# Le menu parfait ?

<!-- pause -->

1. **Profiler** — choisir un coût mesuré, comparer en `--release`.

<!-- pause -->

2. **Batcher** — une petite API, du travail utile à chaque appel.

<!-- pause -->

3. **Détacher** — laisser Python avancer pendant les longs traitements Rust.

<!-- pause -->

→ **Python garde l'ergonomie ; Rust prend le travail lourd.**

<!-- notes -->

- 1 min. Réponse au titre : c'est un bon menu quand le gain justifie les coûts de build, de packaging et de maintenance de l'interface
- Profiler avant de réécrire, comparer le même travail et la même sortie
- Batcher pour amortir conversions et passages de frontière
- `detach` pour les longs traitements Rust sans accès à Python ; si l'appelant est asyncio, offrir aussi un awaitable ou déporter le travail sur un thread
- Prochain pas concret : `maturin new -b pyo3`, exposer une fonction ciblée, tester le contrat des deux côtés
- Passer aux questions à la minute 40
