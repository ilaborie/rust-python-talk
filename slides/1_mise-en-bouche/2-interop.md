+++
title = "Interopérabilité"
classes = ["no_title", "data-table"]
+++

<style>
/* Titre collé en haut : la marge négative le sort en partie du `space-evenly`
   de l'article, ce qui rapproche le titre du bord et rend le reste au corps.
   Trois précautions :
   — dupliqué slide par slide, parce que le thème rend le corps dans un shadow
     root que le CSS de `_head.html` ne traverse pas ;
   — les `:not()` parce que l'export statique (et le PDF) met tous ces `<style>`
     dans un seul document : la règle fuirait sur les mises en page qui n'ont
     aucune marge à récupérer (titre centré, `.step` en `flex: 1`, terminal
     plein cadre, code dense) et le titre sortirait par le haut ;
   — -0.5em et pas plus : au-delà, « Le constat » et « Deux façons d'appeler »
     débordent par le haut. Mesuré sur /run, de 1280×800 à 3840×2160. */
section:not(.center):not(.spread-steps):not(.fourneaux):not(.dense-code) > article > h1 {
	margin-block: -0.5em;
}
/* Rendu de tableau calqué sur Bootstrap 5.3 :
   — Reboot : `th { text-align: inherit }`, pour que l'en-tête suive
     l'alignement des cellules au lieu du `center` du navigateur ;
   — `.table` : filet de 1px sous chaque ligne ;
   — `.table-group-divider` : filet de 2px en `currentColor` entre `thead` et
     `tbody`, seule séparation forte du tableau.
   Padding vertical en `em` du tableau (et pas les `0.5rem` de Bootstrap) :
   proportionnel à la résolution comme le reste du deck, et calé pour que les
   7 lignes de « Ce que PyO3 doit réconcilier » tiennent dans la scène. */
section.data-table table {
	width: 100%;
	border-collapse: collapse;
}

section.data-table th,
section.data-table td {
	padding: 0.3em 0.5em;
	text-align: inherit;
	vertical-align: top;
	border-bottom: 1px solid #dee2e6;
}

section.data-table thead th {
	vertical-align: bottom;
}

section.data-table tbody {
	border-top: 2px solid currentColor;
}
</style>

# Interopérabilité

| Approche | Pour | Contre |
|---|---|---|
| `ctypes` | zéro compilation | lent, aucune sûreté |
| `cffi` | ABI ou API mode | une dépendance de plus |
| C-API | perf maximale | tout à la main, en C |
| **PyO3** | ergonomie Rust + perf C-API | il faut compiler du Rust |

<!-- pause -->

Ce que [PyO3](http://pyo3.rs) écrit à votre place : **conversions de types**, **exceptions**,
**signatures & docstrings**, **build & wheels**.

<!-- pause -->

**dans les deux sens** : Python → Rust, et Rust → Python.

<!-- notes -->

- ctypes/cffi : on décrit la signature à la main, à chaque appel — et si on se trompe, segfault
- C-API : c'est ce que fait CPython lui-même, c'est ce que PyO3 génère sous le capot
- Le point à faire passer : la FFI brute *marche* depuis toujours, le problème c'est la DX
- Python → Rust, c'est le cas courant : on remplace le moteur, on garde l'API Python
- Rust → Python : est-ce que ça a un intérêt ? On y répond en fin de chapitre 2 (scripting, plugins, écosystème ML)
- Transition : on passe à la pratique
