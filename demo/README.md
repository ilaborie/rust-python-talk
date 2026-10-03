# Live-code

1. `maturin new`
2. `uv venv init`
3. code `to_html(str, bool) -> str`
4. `maturin develop`
5. python `import md; md.to_html('**plop**', False)`
6. udpate code `println!("plop")`
7. test -> penible -> `[tool.uv] cache-keys` sur `src/**/*.rs`, puis `uv run md2html.py`
8. docstring + signature -> test dans repl
