+++
title = "Ce qui coince"
classes = ["no_title", "spread-steps"]
+++

# Les difficultés / Limitations

<!-- pause -->

- Pas de génériques dans les classes: `#[pyclass] struct Container<T>`
- Pas de lifetime dans les classes: `#[pyclass] struct Container<'a>`
- Pas d'interop traits vers protocoles, par ex. `Iterator` ne devient pas itérable sans écrire manuellement les `__iter__` & `__next__`

- La génération des `.pyi` n'est pas automatique, [Python typing hints](https://pyo3.rs/v0.29.2/python-typing-hints.html), [#5137](https://github.com/PyO3/pyo3/issues/5137)
- Ne pas bloquer le GIL

<!-- notes -->

- Le point .pyi est le plus coûteux : rien ne détecte la dérive au build - sauf un test qu'on écrit soi-même
- Sur toboggan-py : le crate est hors du workspace cargo → ni `cargo clippy --all-targets` ni `nextest` ne le voient
- C'était aussi hors CI. Corrigé depuis : un job dédié, plus un test qui compare le .pyi au module construit
- Pistes : `pyo3-stub-gen`, ou la feature `experimental-inspect` de PyO3 qui génère les stubs
- Génériques : la solution est d'exposer des types concrets (IntContainer, StrContainer)
- Le GIL : c'est LE piège du pattern « runtime interne ». `Python::detach` autour du block_on le règle
- C'est exactement ce que fait toboggan-py depuis peu : avant, un `clients()` gelait tous les threads Python
- Ne pas culpabiliser : c'est un choix de dette assumé quand la lib est mono-thread côté Python
