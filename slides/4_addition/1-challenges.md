+++
title = "Ce qui coince"
classes = ["no_title", "spread-steps"]
+++

# Ce qui coince

<!-- pause -->

-  **Les `.pyi` sont écrits à la main** 
-  **Pas de génériques** — `#[pyclass] struct Container<T>` ne compile pas :
  il faut monomorphiser
-  **Pas de traits → protocoles** — `Iterator` ne devient pas itérable :
  il faut écrire `__iter__` / `__next__`
-  **Pas d'héritage en diamant** Python → Rust → Python
-  **Le GIL reste pris** pendant un `block_on` : un aller-retour réseau gèle
  les autres threads Python

<!-- pause -->

→ Presque tout se contourne. Mais ça se **paie en wrapper**.

<!-- notes -->

- Le point .pyi est le plus coûteux : rien ne détecte la dérive au build
- Sur toboggan-py : le crate est hors du workspace cargo ET hors CI → dérive invisible
- Pistes : `pyo3-stub-gen`, ou la feature `experimental-inspect` de PyO3 qui génère les stubs
- Génériques : la solution est d'exposer des types concrets (IntContainer, StrContainer)
- Le GIL : c'est LE piège du pattern « runtime interne ». `Python::detach` autour du block_on le règle
- C'est exactement ce que fait toboggan-py depuis peu : avant, un `clients()` gelait tous les threads Python
- Ne pas culpabiliser : c'est un choix de dette assumé quand la lib est mono-thread côté Python
