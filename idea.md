
## La présentation
Titre: Rust & Python: le menu parfait ?
Durée prévus: 30min dont 15min pour le livecode.

Maximum 20 slides

## Le Plan

- Intro 
 - any language <---> C
 - Interop => ajout de la DX (conversion entre types, bidirectionnel, ...)
- Démo - Pyo3 & Maturin
 - live code maturin, comrak (livecode)
 - Bidirectionnelexample call python in Rust
 - Retour d'experiance
  - toboggan (voir ../toboggan/toboggan-py)
    - Async vers appel bloquant
  - wefox - API client pour le stockage des résultats d'inférence & feedback
    - avec tokio et interop future
    - challenge OpenTelemetry instrumentation
- Takeaway
 - Bilan de tout ce qu'on peut faire 
 - Difficultés (pyi, need wrapper)
 - Tips
 - Use cases
  - API multilanguage (ex: OpenDAL, xberg)
  - performance/efficacité (ex: polars, ...)
 - links
- Merci + questions

## Direction artistique

Voir ./public/img/rust-python.png
Foodtruck, repas, gout, ...

Note: qu'il faut revoir les titres des chapitres et slides en fonction du champs sématique associée à la cuisine, foodtruck, cours de cuisines, ...

## Liens utils 

à utiliser pour construire les slides et pour mettre dans le slide de fin

Pyo3 & maturin
[py03 repository](https://github.com/pyo3/pyo3)
[py03 guide](https://pyo3.rs/v0.29.2/)
[py03 API](https://docs.rs/pyo3/latest/pyo3/)
[maturin repository](https://github.com/PyO3/maturin)
[maturin guide](https://www.maturin.rs/)

+ [ecosystem](https://pyo3.rs/v0.29.2/ecosystem.html)
 
[Talks - David Hewitt: How Python Harnesses Rust through PyO3](https://www.youtube.com/watch?v=UilujdubqVU)
[Using Rust in Free-Threaded vs Regular Python 3.13 - David Hewitt](https://www.youtube.com/watch?v=J7phN_M4GLM)
[ITW - PyO3 : De Python à Rust et retour (avec David Hewitt)](https://www.youtube.com/watch?v=UmL_CA-v3O8)
[Techniques apprises au cours de cinq années passées à trouver la voie vers Rust en Python - David Hewitt](https://www.youtube.com/watch?v=KTQn_PTHNCw)


## TODO

1. on renomer le répertoire ./slides/ en ./slides-old/
   car le contenu de doit être complèmetement réécrit
2. penser à utiliser les éléments de la DA, l'illustration doit être utiliser pour la page de titre. Il faut peut-être commencer par faire un fichier DA.md avec tout ce qui peut être explorer dans le domain de la cuisine ou du foodruck. (il faut convertir l'image en webp
3. réécrir le contenu à partir du plan, s'appuyer sur les guides officiels.
   on peut utiliser du contenu des anciens slides si besoin, ou le fichier ./notes.md
