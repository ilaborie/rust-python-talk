+++
title = "Références"
classes = ["no_title", "center", "merci"]
+++

<style>
section.merci h1 { font-size: 240%; color: #B13F15; margin-bottom: 0.1em; }
section.merci h2 { font-size: 130%; font-style: italic; font-weight: 400; color: #728B83; margin-top: 0; }
section.merci ul { font-size: 78%; text-align: left; display: inline-block; }
</style>

## Merci !

### Des questions ?

--- 

- **Outils** — [pyo3.rs](https://pyo3.rs/) · [maturin.rs](https://www.maturin.rs/) ·
  [écosystème PyO3](https://pyo3.rs/v0.29.2/ecosystem.html) ·
  [pyo3-async-runtimes](https://github.com/PyO3/pyo3-async-runtimes) ·
  [rust-numpy](https://github.com/PyO3/rust-numpy) ·
  [py-spy](https://github.com/benfred/py-spy) ·
  [maturin-import-hook](https://github.com/PyO3/maturin-import-hook)

- **Les talks de David Hewitt**, mainteneur PyO3 —
  [PyO3 in depth](https://www.youtube.com/watch?v=UilujdubqVU) ·
  [Free-threaded Python](https://www.youtube.com/watch?v=J7phN_M4GLM) ·
  [5 years of Rust in Python](https://www.youtube.com/watch?v=KTQn_PTHNCw) ·
  [ITW - From Python to Rust and Back Again](https://www.youtube.com/watch?v=UmL_CA-v3O8)


- **Le code de cette conf** —
  [github.com/ilaborie/rust-python-talk](https://github.com/ilaborie/rust-python-talk)
  [slides](https://ilaborie.github.io/rust-python-talk)

<!-- notes -->

- Laisser cette slide affichée pendant les questions
- Si on demande « par où commencer » : le guide PyO3, puis `maturin new -b pyo3`
- Si on demande le free-threading (PEP 703) : PyO3 le supporte, `Py<T>` reste correct, mais toute la synchro devient critique
