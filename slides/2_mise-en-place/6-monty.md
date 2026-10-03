+++
title = "Et si on réécrivait Python ?"
classes = ["no_title"]
+++

<style>
/* Titre collé en haut : la marge négative le sort en partie du `space-evenly`
   de l'article, ce qui rapproche le titre du bord et rend le reste au corps.
   Trois précautions :
   - dupliqué slide par slide, parce que le thème rend le corps dans un shadow
     root que le CSS de `_head.html` ne traverse pas ;
   - les `:not()` parce que l'export statique (et le PDF) met tous ces `<style>`
     dans un seul document : la règle fuirait sur les mises en page qui n'ont
     aucune marge à récupérer (titre centré, `.step` en `flex: 1`, terminal
     plein cadre, code dense) et le titre sortirait par le haut ;
   - -0.5em et pas plus : au-delà, « Le constat » et « Deux façons d'appeler »
     débordent par le haut. Mesuré sur /run, de 1280×800 à 3840×2160. */
section:not(.center):not(.spread-steps):not(.fourneaux):not(.dense-code) > article > h1 {
	margin-block: -0.5em;
}
</style>

# Et si on réécrivait Python ?

[Monty](https://pydantic.dev/docs/monty/) : un interpréteur Python écrit en Rust, pour exécuter du code non fiable

- **PyO3** embarque CPython : tout Python, tout l'écosystème
- **Monty** le remplace : un sous-ensemble de Python, isolé de l'hôte

<!-- pause -->

En Python avec `pydantic-monty`… écrit avec PyO3

<!-- notes -->

- Monty, par Pydantic, MIT, v1.0.0 sortie le 2026-09-25 (`gh api repos/pydantic/monty/releases/latest`)
- Une VM bytecode en Rust (crate `monty`), pas de CPython dedans
- Les sandboxes tournent dans des workers en sous-processus, gérés par un pool (crate `monty-pool`)
- Promesse : < 1 ms pour obtenir une sandbox depuis un pool chaud, contre ~1500 ms pour un service de sandbox (chiffres de leur README, pas mesurés par moi)
- Le code n'atteint l'hôte que par les fonctions et les montages qu'on lui passe
- Limite : un sous-ensemble de Python 3.14, donc pas de numpy, pas de stdlib complète
- Utilisé par le « Code Mode » de Pydantic AI
- La boucle est bouclée : l'extension Python `_monty` est un crate PyO3 (`crates/monty-python`)
- Transition : on passe à la pratique avec toboggan-py
