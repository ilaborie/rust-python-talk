# Rust & Python

TODO: trouver un titre catchy, on peut par example joué sur la notion d'animaux (crabe et serpent)

## Interop langage de programation

En gros presque tous les langages peuvent s'interop avec du C, donc avec des appels compatible avec l'ABI C

définir ABI, préciser comment se passe un appel de fonction C (stack param, resultat)

Concretement en Python, Et en Rust -> FFI

## Pourquoi interop Python - Rust

Rust: Perf et + de sécurité, et super système de typages Rust: on peu être proche du système et avoir des features de hauts niveau Rust: fearless concurrency

Python est souple (laxiste), iteration rapide (prototypage, exploration, ...) Python est très accessible (scientific, scolaire) Python a un riche écosystème, par example big-data, ML. Python lent -> lib native pour perf (donné des examples comme pytorch, ...)

Le succès de Python est aussi basé sur des lib native exposée dans Python, certain sont déjà écrit en Rust

Slide sur les examples existant (pydantic, databusion, polars, ...)

Est-ce qu'il y a des cas intéressants pour appeler du phython depuis Rust ?

## Du code

Avec Maturin, Pyo3, trouver un example fun mais simple

Exemple 1 utiliser comrak pour faire du markdown? (live-code)

Exemple 2 perf test pour analyser des logs structurés (exemple log server, faire un compute des avg, min, max, ecart type des temps de réponse) => comparaison des perf en pure Rust

Exemple 3: executer du code python en Rust.

Recap sur ce qu'on peut faire (module, class, ...) et pas faire

## Retour XP, challenges

### REX

Wefox AI: Inference storage: création d'une API client pour un microservice, pour les équipes qui faisait du Rust, et pour celle qui faisait du Python.

Toboggan-py

### Challenges

Asynchonisme (possible, tokio et pyo3, ou bloquant (Runtime tokio interne))

Intergration OpenTelemetry complex pour récupérer les éléments?

Ownership des data: Python ou Rust, comment ça se comporte?

### Tips

Réduire la surface d'échange (cloisoné)

Python sync + Rust async avec un Runtime tokio shared

## Conclusion

Rust 🤝 Python

Option 1: RIIR des libs (partie de lib) pour de la perf Option 2: API Polyglote, voir OpenDAL, Kreuzberg Option 3: proto en python, et réécriture (partiel/total) en Rust Option 4: 🤔 a-t'on besoin de Python, aujourd'hui on a des outils qui permets un proto rapide en Rust
