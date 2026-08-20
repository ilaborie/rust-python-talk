+++
title = "Ce qu'il y a déjà dans votre assiette"
classes = ["no_title"]
+++

# Déjà dans votre assiette

<!-- pause -->

- 🐼 [Polars](https://pola.rs/) — DataFrame
- 🧹 [Ruff](https://github.com/astral-sh/ruff) — linter
- 📦 [uv](https://github.com/astral-sh/uv) — packaging
- ✅ [pydantic-core](https://github.com/pydantic/pydantic-core) — validation
- 🔤 [tokenizers](https://github.com/huggingface/tokenizers) — NLP
- ⚡ [orjson](https://github.com/ijl/orjson) — JSON

<!-- pause -->

→ La stack Python qui va vite est **écrite en Rust**.

<!-- notes -->

- Question au public : qui utilise au moins un de ces outils ? (souvent : tout le monde, sans le savoir)
- Le pattern est toujours le même : on garde l'API Python, on remplace le moteur
- Ruff et uv : c'est de l'outillage, l'utilisateur ne voit même pas que c'est natif
- pydantic v2 : la v1 était en Python pur, la v2 a un cœur Rust — même API
- Donc : ce n'est pas une expérimentation, c'est déjà la norme
