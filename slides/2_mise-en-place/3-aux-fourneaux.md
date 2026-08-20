+++
title = "Live-code"
classes = ["no_title", "wide", "fourneaux"]
hidden_in = ["pdf"]
quake_cwd = "../demo"
+++

<style>
/* Le terminal EST la slide : le titre se réduit à un bandeau, tout le reste
   est pour le shell.
   `.toboggan-terminals` est un FRÈRE de <article>, pas un descendant — d'où
   les sélecteurs sur `section`. On n'utilise pas la classe `term-50vh` du
   thème : elle fige le terminal à `flex: 0 0 auto; height: 50vh`, ce qui
   laissait la moitié haute vide. */
section.fourneaux > article {
	flex: 0 0 auto;
}

section.fourneaux h1 {
	font-size: 130%;
	margin: 0.1em 0 0.25em;
}

section.fourneaux .toboggan-terminals {
	flex: 1 1 auto;
	min-height: 0;
	/* Le rendu du terminal quantifie sa hauteur en lignes : le résidu est
	   réparti haut/bas plutôt que rejeté sous le terminal. */
	align-items: center;
}

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
</style>

# 🔥 Live-code

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
