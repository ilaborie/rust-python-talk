
## Général

On peut réduire l'usage du champ sémantique cuisine/foodtruck dans les slides, mais le conserver dans les titre de chapitre. Il faut que ça soit plus subtile.

## Chapitre 1 (mise-en-bouche)

Il faut revoir les slides avec:

### Le constat

Python sait appeler du C
Rust sait produire du code compatible avec la C ABI

Alors c'est quoi le problème ?

### Interop

C'est principalement de la DX pour le développeur

(conversion de type, build, ...)

Python -> Rust
Rust -> Python est-il intéressant

### Quel intérêt?

Citation de David Hewitt (Mainainter Pyo3):
Rust offers **power and precision** to go beyond Python's limits

=> Reliability, Performance, Security, Concurences

### Challenges

Tableau avec les différences (modèle mémoire, classes héritage, Dict, GIL, Exception, ...)
