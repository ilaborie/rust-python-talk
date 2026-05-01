# Rust & Python

TODO: trouver un titre catchy, on peut par exemple jouer sur la notion d'animaux (crabe et serpent)

Propositions :
- **« Crabe & Serpent : un mariage de raison »**
- **« Ferris rencontre Python : interop Rust ↔ Python en pratique »**
- **« Du venin et de la rouille : booster Python avec Rust »**

## Interop langage de programmation

Presque tous les langages peuvent s'interopérer avec du C, via des appels compatibles avec l'ABI C.

**ABI** (Application Binary Interface) vs API :
- Convention d'appel : passage des paramètres par registres ou pile selon la plateforme (System V AMD64, Windows x64)
- Layout mémoire des structures, name mangling
- Ownership des pointeurs : qui alloue, qui libère ?

Côté Python :
- `ctypes` : appel direct de bibliothèques partagées (`.so`, `.dll`)
- `cffi` : interface plus expressive, supporte l'ABI et l'API mode
- **API CPython** (`PyObject*`) : le vrai mécanisme d'extension natif

Côté Rust :
- `extern "C"`, `#[no_mangle]`, `repr(C)`
- Crate [`libc`](https://crates.io/crates/libc) pour les types compatibles C

Concrètement : Python ↔ Rust, c'est **FFI** (Foreign Function Interface) via l'ABI C.

## Pourquoi interop Python - Rust

**Rust apporte :**
- Performances et sécurité mémoire (sans GC)
- Système de types expressif (enums, traits, lifetimes)
- Proche du système tout en offrant des abstractions haut niveau
- Fearless concurrency

**Python apporte :**
- Souplesse et itération rapide (prototypage, exploration)
- Accessibilité (scientifique, scolaire, data science)
- Écosystème riche : big-data, ML (NumPy, scikit-learn, PyTorch, pandas…)
- Python est lent → des libs natives pour la perf (ex : PyTorch, NumPy, TensorFlow)

Le succès de Python repose en grande partie sur des libs natives exposées en Python — certaines sont déjà écrites en Rust :
- [Polars](https://pola.rs/) — DataFrame ultra-rapide
- [Ruff](https://github.com/astral-sh/ruff) — linter Python
- [uv](https://github.com/astral-sh/uv) — gestionnaire de packages Python
- [Pydantic v2](https://github.com/pydantic/pydantic-core) (pydantic-core) — validation de données
- [HuggingFace Tokenizers](https://github.com/huggingface/tokenizers) — tokenisation NLP
- [orjson](https://github.com/ijl/orjson) — sérialisation JSON
- [DataFusion](https://github.com/apache/datafusion-python) — moteur SQL analytique
- [Granian](https://github.com/emmett-framework/granian), [Robyn](https://robyn.tech/) — serveurs HTTP

**Est-ce qu'il y a des cas intéressants pour appeler du Python depuis Rust ?**
- Oui : embarquer un interpréteur Python pour du scripting utilisateur ou des plugins ML
- Réutiliser un écosystème scientifique mature (NumPy, scikit-learn) depuis un service Rust
- Transition progressive d'un projet Python vers Rust

## Du code

Stack : **[PyO3](https://pyo3.rs/)** (bindings Rust ↔ Python) + **[Maturin](https://www.maturin.rs/)** (build & packaging wheels)

### Exemple 1 : utiliser comrak pour faire du Markdown (live-code)

- Exposer `fn markdown_to_html(s: &str) -> String`
- Cas pédagogique idéal : zéro état partagé, juste une transformation pure
- `#[pyfunction]` + `#[pymodule]` — quelques lignes de code

### Exemple 2 : perf — analyser des logs structurés

- Log serveur, calculer avg / min / max / écart-type des temps de réponse
- Comparer :
  - (a) pur Python
  - (b) pur Rust (benchmark de référence)
  - (c) Python qui appelle Rust via PyO3
- Mesurer aussi le coût de conversion `PyList` ↔ `Vec` (overhead FFI)

### Exemple 3 : exécuter du code Python depuis Rust

- `Python::with_gil(|py| { ... })` — acquérir le GIL et exécuter du code Python
- Utile pour du scripting utilisateur, des plugins, piloter un écosystème ML
- Crate [`pyo3`](https://pyo3.rs/) des deux côtés

### Récap : ce qu'on peut faire

- Modules (`#[pymodule]`), fonctions (`#[pyfunction]`), classes (`#[pyclass]`)
- Exceptions custom (`#[pyclass(extends=PyException)]`)
- Sous-modules, conversions de types (via `IntoPy`, `FromPyObject`)
- Interop NumPy via [`rust-numpy`](https://github.com/PyO3/rust-numpy)

### Récap : ce qu'on ne peut pas (facilement) faire

- Héritage multi-niveau Python depuis Rust
- Exposer des génériques Rust tels quels
- Mapping automatique traits Rust → protocoles Python

## Retour XP, challenges

### REX

**Wefox AI — Inference storage** : création d'une API client pour un microservice, pour les équipes Rust et pour les équipes Python.

**[Toboggan-py](https://github.com/ilaborie/toboggan)** — projet pédagogique, support de présentation.

### Challenges

**Asynchronisme**
- Possible : [`pyo3-async-runtimes`](https://github.com/PyO3/pyo3-async-runtimes) (ex `pyo3-asyncio`) fait le pont entre `tokio` et `asyncio`
- Piège : le GIL doit être relâché pendant les await Rust, sinon deadlock
- Alternative : runtime Tokio interne (bloquant côté Python, async côté Rust)

**Intégration OpenTelemetry**
- Complexe : les spans Rust ne voient pas le contexte Python OTel et inversement
- Propagation de contexte cross-language non triviale
- Écosystème en cours d'évolution

**Ownership des données : Python ou Rust ?**
- Côté Rust : `Py<T>` — référence comptée vers un objet Python
- Côté Python : refcount géré par CPython
- Le GIL comme « lock global » — attention aux fuites circulaires
- **GIL lui-même** : challenge pour la concurrence (voir [PEP 703](https://peps.python.org/pep-0703/) — free-threaded Python 3.13+)

**Packaging multi-plateforme**
- Wheels : manylinux, musllinux, macOS universal2, Windows
- Maturin gère ça + CI avec `cibuildwheel` ou GitHub Actions

### Tips

- **Réduire la surface d'échange** (cloisonner) : moins de franchissements FFI = moins d'overhead
- **Passer des batchs** plutôt que des appels unitaires (amortir le coût de passage FFI)
- **Python sync + Rust async** avec un Runtime Tokio partagé — créer le `Runtime` une fois, le réutiliser
- **Libérer le GIL** (`py.allow_threads(...)`) sur les sections Rust CPU-bound → le code Python concurrent peut respirer
- **Tester des deux côtés** : `pytest` côté Python, `cargo test` côté Rust — chaque côté a ses propres angles morts
- **Profiler avec [`py-spy`](https://github.com/benfred/py-spy)** — il voit aussi les frames natives Rust

## Conclusion

Rust 🤝 Python

- **Option 1 : RIIR** — Réécriture (partielle) de libs Python en Rust pour la performance
- **Option 2 : API polyglotte** — une implémentation Rust, exposée en Python (et Go, Java…) via PyO3 — voir [OpenDAL](https://github.com/apache/opendal), [Kreuzberg](https://github.com/Goldziher/kreuzberg)
- **Option 3 : Proto en Python, réécriture (partielle ou totale) en Rust** — itérer vite, puis optimiser
- **Option 4 : 🤔 A-t-on besoin de Python ?** — aujourd'hui Rust a `polars`, `axum`, `candle` (ML), des notebooks ([evcxr](https://github.com/evcxr/evcxr)), donc le proto rapide est possible… mais Python garde l'avantage de l'écosystème data/ML

---

## Références

### Outils / crates

- [PyO3](https://pyo3.rs/) — bindings Rust ↔ Python
- [Maturin](https://www.maturin.rs/) — build & packaging de modules Python en Rust
- [pyo3-async-runtimes](https://github.com/PyO3/pyo3-async-runtimes) — pont tokio/asyncio
- [rust-numpy](https://github.com/PyO3/rust-numpy) — interop NumPy
- [libc](https://crates.io/crates/libc) — types C pour FFI Rust

### Projets emblématiques (Rust exposé en Python)

- [Polars](https://pola.rs/) — DataFrame
- [Ruff](https://github.com/astral-sh/ruff) — linter Python
- [uv](https://github.com/astral-sh/uv) — gestionnaire de packages
- [pydantic-core](https://github.com/pydantic/pydantic-core) — cœur de Pydantic v2
- [HuggingFace Tokenizers](https://github.com/huggingface/tokenizers) — NLP
- [orjson](https://github.com/ijl/orjson) — JSON rapide
- [DataFusion Python](https://github.com/apache/datafusion-python) — SQL analytique
- [Granian](https://github.com/emmett-framework/granian), [Robyn](https://robyn.tech/) — serveurs HTTP

### Articles

- [« Making Python 100x faster with less than 100 lines of Rust »](https://ohadravid.github.io/posts/2023-03-rusty-python/) — Ohad Ravid
- [« How Pydantic V2 leverages Rust's Superpowers »](https://samuel.colvin.me/posts/2023-01-09-annotated-types-and-pydantic-v2/) — Samuel Colvin
- [Astral blog](https://astral.sh/blog) — Ruff, uv, la stack Rust pour Python
- [The Rustonomicon — FFI](https://doc.rust-lang.org/nomicon/ffi.html) — FFI Rust bas niveau
- [PEP 703 — Making the GIL Optional](https://peps.python.org/pep-0703/) — free-threaded Python 3.13+
- [PyO3 User Guide](https://pyo3.rs/latest/) — référence complète

### Talks vidéos

- David Hewitt — « PyO3: From Python to Rust and Back Again » — mainteneur PyO3, EuroRust (chercher sur YouTube)
- Ritchie Vink — talks Polars (PyCon, EuroRust) — créateur de Polars
- Samuel Colvin — « How Pydantic V2 Was Made » — PyCon US
- « Calling Rust from Python » — talks PyO3 sur YouTube / conference.scipy.org
