+++
title = "Ce que PyO3 doit réconcilier"
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

# Ce que PyO3 doit réconcilier

| | Python | Rust |
|---|---|---|
| Mémoire | refcount + GC | ownership, pas de GC |
| Objets | héritage, monkey-patching | structs + traits |
| Collections | `dict` hétérogène | `HashMap<K, V>` homogène |
| Erreurs | exceptions | `Result<T, E>` |
| Threads | GIL | pas de data race |
| Typage | dynamique, à l'exécution | statique, à la compilation |

<!-- pause -->

→ Deux modèles qui ne se recouvrent pas. **PyO3 fait la traduction** —

<!-- notes -->

là où il ne peut pas, on le verra en fin de talk.

- Ligne mémoire : c'est LA divergence de fond. `Py<T>` côté Rust garde une refcount Python ; le GC Python ne voit pas dans le tas Rust → pas de cycle Rust ↔ Python
- Objets : pas d'héritage en Rust, donc pas de mapping direct — PyO3 expose `#[pyclass]` + `#[pymethods]`
- Collections : le `dict` Python accepte n'importe quoi ; côté Rust il faut choisir un type, ou passer par `PyDict`/`PyAny`
- Erreurs : `PyResult<T>` = `Result<T, PyErr>` — un `Err` remonte en exception Python, gratuitement
- GIL : nécessaire pour lire/écrire un `PyObject`. `py.detach(...)` le relâche sur les sections Rust pures
- Typage : le `.pyi` doit être maintenu à la main — on y revient dans « Ce qui coince »
- Ne pas détailler ligne à ligne : on lit le tableau en diagonale, la punchline porte le message
