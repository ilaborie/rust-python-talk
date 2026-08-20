# Live-code — « Aux fourneaux »

Support du live-code de la conf **Rust & Python : le menu parfait ?**
(slide `2_mise-en-place/2-aux-fourneaux.md`, ~15 min).

Le terminal intégré à la slide s'ouvre **dans ce dossier**.

- `solution/` — le projet fini, qui compile et qui tourne. **Filet de sécurité.**
  C'est aussi la source du code affiché sur la slide « Le plat servi » : ne pas
  l'éditer sans revérifier la slide.
- `rusty-md/` — le projet créé en live. Ignoré par le VCS, à supprimer avant
  chaque répétition.

## Avant de monter sur scène

```bash
rm -rf rusty-md                    # repartir d'une page blanche
(cd solution && cargo build)       # préchauffer le cache cargo (~5 s au lieu de 40)
```

Le cache de `solution/` est partagé avec `rusty-md/` : `comrak` et `pyo3` sont
déjà compilés, donc le premier `maturin develop` en live prend quelques secondes.

## Le script

### 1. Le squelette (30 s)

```bash
maturin new -b pyo3 rusty-md
cd rusty-md
```

Montrer les 3 fichiers générés : `Cargo.toml` (`crate-type = ["cdylib"]`),
`pyproject.toml` (`build-backend = "maturin"`), `src/lib.rs`.

### 2. Les ingrédients (45 s)

```bash
cargo add comrak --no-default-features
cargo add pyo3@0.29 --features abi3-py38
```

- `--no-default-features` : sans ça, `comrak` tire `syntect` et `clap`, ~40 s de
  compilation qu'on ne veut pas subir sur scène.
- **Le second `cargo add` n'est pas cosmétique** : `maturin new` scaffolde encore
  PyO3 **0.25**, alors que les slides parlent de 0.29. Sans ce bump, la version
  affichée à l'écran contredit le support.

### 3. La recette (3 min)

Vider `src/lib.rs` et écrire, en direct :

```rust
use pyo3::prelude::*;

#[pyfunction]
fn markdown_to_html(input: &str) -> String {
    comrak::markdown_to_html(input, &comrak::Options::default())
}

#[pymodule]
fn rusty_md(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(markdown_to_html, m)?)?;
    Ok(())
}
```

Insister : `&str` ↔ `str` et `String` ↔ `str`, c'est PyO3 qui convertit. On n'a
écrit aucune ligne de FFI.

### 4. Le service (2 min)

```bash
uv venv          # pas besoin de l'activer : maturin trouve ./.venv tout seul
maturin develop
python -c "from rusty_md import markdown_to_html; print(markdown_to_html('# Hello **world**'))"
```

Attendu : `<h1>Hello <strong>world</strong></h1>`

### 5. Ce que PyO3 fait pour nous (3 min)

```bash
python -c "from rusty_md import markdown_to_html; markdown_to_html(42)"
```

→ ```
TypeError: 'int' object is not an instance of 'str'
while processing 'input'
```

Le typage Rust est devenu une `TypeError` Python. Puis :

```bash
python -c "import rusty_md; help(rusty_md.markdown_to_html)"
```

→ la doc Rust `///` est devenue la docstring Python.

### 6. Un vrai paramètre (4 min)

Ajouter un argument nommé optionnel :

```rust
#[pyfunction]
#[pyo3(signature = (input, *, github_flavored = false))]
fn markdown_to_html(input: &str, github_flavored: bool) -> String {
    let mut options = comrak::Options::default();
    options.extension.strikethrough = github_flavored;
    options.extension.table = github_flavored;
    comrak::markdown_to_html(input, &options)
}
```

```bash
maturin develop
python -c "from rusty_md import markdown_to_html; print(markdown_to_html('~~barré~~', github_flavored=True))"
```

→ `<p><del>barré</del></p>`

Le `*` de la signature Rust produit un **keyword-only argument** Python. C'est ça,
la DX : on décrit l'API Python depuis Rust.

### 7. Sortie (1 min)

```bash
maturin build --release
ls target/wheels/
```

Une wheel, `pip install`-able, sans Rust chez l'utilisateur.

## Vérifier le filet

```bash
cd solution
uv venv && maturin develop && ./.venv/bin/python try.py
```

Attendu :

```
<h1>Hello <strong>world</strong></h1>

<p><del>barré</del></p>
```
