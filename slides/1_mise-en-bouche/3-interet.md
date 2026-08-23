+++
title = "Quel intérêt ?"
classes = ["no_title", "citation"]
+++

<style>
/* Le thème rend les blockquotes comme du texte courant : sans marque visuelle,
   la citation se confond avec la punchline juste en dessous. Filet à la couleur
   Rust + italique, et l'attribution en petit, alignée à droite. */
section.citation blockquote {
	border-left: 0.12em solid #B13F15;
	padding-left: 0.6em;
	font-style: italic;
	font-size: 115%;
	color: #2D2B25;
}

section.citation blockquote p:last-child {
	font-size: 75%;
	font-style: normal;
	text-align: right;
	color: #728B83;
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

# Quel intérêt ?

> Rust offers **power and precision** to go beyond Python's limits.
>
> — David Hewitt, mainteneur de [PyO3](https://pyo3.rs/)

<!-- pause -->

→ **Fiabilité** · **Performance** · **Sûreté mémoire** · **Concurrence**

<!-- pause -->

[Polars](https://pola.rs/) ·
[datafusion](https://datafusion.apache.org/) ·
[pydantic-core](https://github.com/pydantic/pydantic-core) ·
[tokenizers](https://github.com/huggingface/tokenizers) ·
[orjson](https://github.com/ijl/orjson)

→ La stack Python qui va vite est **déjà écrite en Rust**.

<!-- notes -->

- Les 4 mots : fiabilité (le compilo attrape ce que les tests ratent), performance, sûreté mémoire sans GC, concurrence sans data race
- Question au public : qui utilise au moins un de ces outils ? (souvent : tout le monde, sans le savoir)
- Le pattern est toujours le même : on garde l'API Python, on remplace le moteur
- Ruff et uv : c'est de l'outillage, l'utilisateur ne voit même pas que c'est natif
- pydantic v2 : la v1 était en Python pur, la v2 a un cœur Rust — même API
- Donc : ce n'est pas une expérimentation, c'est déjà la norme
