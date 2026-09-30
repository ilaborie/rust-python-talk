# Live-code — 15 minutes

Objectif : appeler Rust depuis Python, exposer une signature compréhensible,
puis montrer la boucle de développement. Garder un terminal Python et un
éditeur visibles.

## Avant la scène

- Installer Rust, Python ≥ 3.9, `uv` et `maturin`; vérifier leurs versions.
- Répéter dans le même environnement et précompiler les dépendances : Comrak
  et son moteur de coloration peuvent prendre plusieurs minutes au premier build.
- Préparer deux dossiers : le projet de live-code et `demo/solution` comme secours.
- Construire le secours avant la session, puis vérifier le résultat :

```sh
export CARGO_TARGET_DIR=$(mktemp -d)
cd demo/solution
uv venv
source .venv/bin/activate
uv pip install maturin-import-hook
maturin develop
python -c 'import md; print(md.to_html("**Bonjour**"))'
```

Résultat attendu : `<p><strong>Bonjour</strong></p>`.
Garder ce terminal : `CARGO_TARGET_DIR` permet au live-code de réutiliser les
dépendances compilées pendant la préparation.

## 0–2 min : résultat et mise en place

Commencer par cet appel et son résultat pour annoncer ce que l'on construit.
Dans un dossier temporaire, créer le projet :

```sh
live_demo_dir=$(mktemp -d)
cd "$live_demo_dir"
maturin init --bindings pyo3 --name md
uv venv
source .venv/bin/activate
cargo add comrak@0.54
cargo add pyo3@0.29 --features abi3-py39
```

Garder le nom du module Rust et celui de la bibliothèque à `md`, puis fixer
`requires-python = ">=3.9"` dans `pyproject.toml` pour l'aligner sur l'ABI.

## 2–6 min : une fonction utilisable

Utiliser `demo/solution/src/lib.rs` comme référence : ajouter `to_html`, les
options Comrak et l'adaptateur de coloration. Pendant ce premier checkpoint,
la fonction peut demander les deux arguments explicitement.

```sh
maturin develop
python -c 'import md; print(md.to_html("**Bonjour**", False))'
```

Checkpoint à 6 min : Python importe `md` et affiche le HTML produit par Rust.
Expliquer que `abi3-py39` vise Python 3.9 et suivants, avec un wheel distinct
par système et architecture.

## 6–10 min : signature, aide et erreurs

Ajouter la docstring et `#[pyfunction(signature = (md:"str", gfm = false))]`,
comme dans la solution, puis reconstruire avec `maturin develop`.
Ouvrir un **nouveau** processus Python :

```python
import inspect
import md

print(inspect.signature(md.to_html))
help(md.to_html)
print(md.to_html("**Bonjour**"))
print(md.to_html("**Bonjour**", True))
print(md.to_html("**Bonjour**", gfm=True))
md.to_html(42)
```

La signature accepte `md` et un `gfm` optionnel, positionnel ou nommé.
Le dernier appel doit lever `TypeError`; lire le message réel dans le terminal.
Le stub `demo/solution/md.pyi` fournit les types aux outils Python; il ne réalise
pas la validation à l'exécution.

Checkpoint à 10 min : aide lisible, valeur par défaut et erreur de type visible.

## 10–13 min : boucle de développement (optionnelle)

Si le timing le permet, installer le hook :

```sh
uv pip install maturin-import-hook
```

Ajouter avant `import md` dans un script Python placé à la racine du projet :

```python
import maturin_import_hook
maturin_import_hook.install()

import md
print(md.to_html("**Bonjour**"))
```

Lancer le script, ajouter temporairement un `println!("Rust appelé");` dans
`to_html`, puis relancer le script dans un nouveau processus. Montrer la
reconstruction automatique. Le hook demande toujours une compilation; les
builds déjà chauds rendent cette étape prévisible.

Checkpoint à 13 min : une modification Rust apparaît au prochain lancement.
Sauter le hook si le deuxième checkpoint dépasse 10 min.

## 13–15 min : résultat et récapitulatif

Revenir aux slides de résultat : appel Python → conversion PyO3 → rendu Rust,
signature et stub. Consacrer le reste à l'ouverture sur le sens inverse,
appeler Python depuis Rust. Ces deux minutes font partie des quinze minutes.

## Coupure et secours

Si le premier build n'est pas terminé à 6 min, ou si une erreur prend plus
d'une minute à résoudre, passer au dossier `demo/solution` préparé. Activer sa
`.venv`, montrer le même appel et poursuivre avec la signature et `TypeError`.
À 10 min, passer au secours si la fonction n'est toujours pas utilisable et
sauter le hook. À 13 min, conclure le code même si le hook n'a pas été montré;
garder les deux dernières minutes pour le récapitulatif.
