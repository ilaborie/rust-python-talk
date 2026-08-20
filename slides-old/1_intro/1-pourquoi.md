+++
title = "Le constat"
classes = ["no_title"]
+++

# Le constat

La stack Python qui va vite, aujourd'hui, est **écrite en Rust**.

<!-- pause -->

- 🐼 [Polars](https://pola.rs/) — DataFrame
- 🧹 [Ruff](https://github.com/astral-sh/ruff) — linter
- 📦 [uv](https://github.com/astral-sh/uv) — package manager
- ✅ [Pydantic v2](https://github.com/pydantic/pydantic-core) — validation
- 🤗 [Tokenizers](https://github.com/huggingface/tokenizers) — NLP
- ⚡ [orjson](https://github.com/ijl/orjson) — JSON

<!-- pause -->

> Et ce n'est qu'un début…

<!-- notes -->

- Polars : remplace pandas, 10-100× plus rapide
- Ruff : a tué flake8 + isort + … en 6 mois
- uv : remplace pip + virtualenv + poetry, 10-100× plus rapide
- Pydantic v2 : cœur Rust = `pydantic-core`
- Le pattern : on garde l'API Python, on remplace le moteur par du Rust
- Cette présentation : comment c'est possible et comment le faire
