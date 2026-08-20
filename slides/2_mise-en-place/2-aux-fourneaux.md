+++
title = "Aux fourneaux"
classes = ["no_title", "term-50vh"]
hidden_in = ["pdf"]
quake_cwd = "../demo"
+++

# 🔥 Aux fourneaux

<!-- term: ../../demo -->

<!-- notes -->

- ~15 min. Script complet dans `demo/README.md`, filet de sécurité dans `demo/solution/`
- Étape 1 — `maturin new -b pyo3 rusty-md` : montrer les 3 fichiers générés
- Étape 2 — `cargo add comrak --no-default-features` (sinon syntect + clap = 40 s de compil)
- Étape 3 — écrire `#[pyfunction] markdown_to_html` puis `#[pymodule]`
- Étape 4 — `uv venv` puis `maturin develop`
- Étape 5 — `python -c "from rusty_md import markdown_to_html; print(markdown_to_html('# Hello **world**'))"`
- Étape 6 — LE MOMENT CLÉ : `markdown_to_html(42)` lève une `TypeError`. Le type Rust est devenu une exception Python
- Étape 7 — `help(rusty_md.markdown_to_html)` : le `///` Rust est devenu la docstring
- Étape 8 — ajouter `#[pyo3(signature = (input, *, github_flavored = false))]` : keyword-only argument
- Étape 9 — `maturin build --release` : une wheel, pas de Rust chez l'utilisateur
- SI ÇA CASSE : `cd solution && maturin develop && ./.venv/bin/python try.py`
