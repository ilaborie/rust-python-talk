+++
title = "Rust dans Python"
classes = ["no_title"]
+++

# 🦀 dans 🐍

Le Rust gagne du terrain comme **moteur natif** :

- 📊 [Polars](https://pola.rs/) — DataFrame
- 🧹 [Ruff](https://github.com/astral-sh/ruff) — linter
- 📦 [uv](https://github.com/astral-sh/uv) — package manager
- ✅ [Pydantic v2](https://github.com/pydantic/pydantic-core) — validation
- 🤗 [Tokenizers](https://github.com/huggingface/tokenizers) — NLP
- ⚡ [orjson](https://github.com/ijl/orjson) — JSON
- 🧮 [DataFusion](https://github.com/apache/datafusion-python) — SQL
- 🌐 [Granian](https://github.com/emmett-framework/granian), [Robyn](https://robyn.tech/) — serveurs HTTP

<!-- notes -->

- Pattern dominant : API Python familière, moteur Rust caché
- L'utilisateur ne sait souvent même pas qu'il y a du Rust derrière
- Astral (Ruff/uv) = pari sur la stack Python entièrement réécrite en Rust
