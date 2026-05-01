# Rust & Python

TODO: trouver un titre catchy, on peut par exemple jouer sur la notion d'animaux (crabe et serpent)

Propositions :

- **« Crabe & Serpent : un mariage de raison »**
- **« Ferris rencontre Python : interop Rust ↔ Python en pratique »**
- **« Du venin et de la rouille : booster Python avec Rust »**

## Interop langage de programmation

### C ABI

Presque tous les langages peuvent s'interopérer avec du C, via des appels compatibles avec l'ABI C.

**ABI** (Application Binary Interface) vs API :

- Convention d'appel : passage des paramètres par registres ou pile selon la plateforme (System V AMD64, Windows x64)
- Layout mémoire des structures, name mangling
- Ownership des pointeurs : qui alloue, qui libère ?

**Schéma d'un appel C (System V AMD64) :**

```text
   Caller                                   Callee
 ┌─────────────────────┐                  ┌─────────────────────┐
 │  prépare les args   │                  │                     │
 │  rdi, rsi, rdx,     │ ── 6 premiers ─▶ │  exécute            │
 │  rcx,  r8, r9       │   args entiers   │                     │
 │  xmm0..xmm7         │ ── flottants  ─▶ │  rax = résultat     │
 │  pile (au-delà)     │                  │                     │
 │                     │ ──── call ────▶  │  ret                │
 │  rax ← retour       │ ◄─── return ───  │                     │
 └─────────────────────┘                  └─────────────────────┘
```

Points de friction quand on quitte C :

- **Name mangling** : C++/Rust mutilent les noms (overloading, modules) → besoin de `extern "C"` + `#[no_mangle]`
- **Layout** : `repr(C)` pour figer l'ordre/padding des champs, sinon Rust se réserve le droit de réordonner
- **Strings** : C utilise `\0` final, Rust utilise (ptr, len) — pas le même contrat

Références :

- [System V AMD64 ABI (psABI)](https://gitlab.com/x86-psABIs/x86-64-ABI)
- [Microsoft x64 calling convention](https://learn.microsoft.com/en-us/cpp/build/x64-calling-convention)
- [Rust Reference — External blocks & ABI](https://doc.rust-lang.org/reference/items/external-blocks.html)

### Python natif

- `ctypes` : appel direct de bibliothèques partagées (`.so`, `.dll`)
- `cffi` : interface plus expressive, supporte l'ABI et l'API mode
- **API CPython** (`PyObject*`) : le vrai mécanisme d'extension natif

**Exemple `ctypes`** (le minimum vital) :

```python
import ctypes
lib = ctypes.CDLL("./libmath.so")
lib.add.argtypes = [ctypes.c_int, ctypes.c_int]
lib.add.restype = ctypes.c_int
print(lib.add(2, 3))  # 5
```

**Exemple API CPython** (extension native, ce que PyO3 génère sous le capot) :

```c
static PyObject* hello(PyObject* self, PyObject* args) {
    return PyUnicode_FromString("Hello from C");
}

static PyMethodDef Methods[] = {
    {"hello", hello, METH_NOARGS, NULL},
    {NULL, NULL, 0, NULL}
};

PyMODINIT_FUNC PyInit_mymod(void) { /* ... */ }
```

Trade-offs :

| Approche | Pour | Contre |
|---|---|---|
| `ctypes` | Pas de compilation | Lent (parsing à chaque appel), pas safe |
| `cffi` | API plus propre, ABI ou API mode | Encore une dépendance |
| C-API | Performance maximale, intégration native | C, tout à la main |
| **PyO3** | Ergonomie Rust + perf C-API | Compilation Rust requise |

Références :

- [Python C API — Extension types](https://docs.python.org/3/extending/extending.html)
- [`ctypes`](https://docs.python.org/3/library/ctypes.html)
- [`cffi`](https://cffi.readthedocs.io/)

### Rust FFI

- `extern "C"`, `#[no_mangle]`, `repr(C)`
- Crate [`libc`](https://crates.io/crates/libc) pour les types compatibles C

**Exemple minimal** :

```rust
#[unsafe(no_mangle)]
pub extern "C" fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[repr(C)]
pub struct Point {
    x: f64,
    y: f64,
}
```

Build : `cargo build --release` → `target/release/libmacrate.{so|dylib|dll}` selon la plateforme.

Concrètement : Python ↔ Rust, c'est **FFI** (Foreign Function Interface) via l'ABI C.

```text
   Python                  ABI C                   Rust
 ┌──────────┐         ┌────────────┐         ┌─────────────┐
 │ PyObject │ ──────▶ │ extern "C" │ ──────▶ │ #[pyclass]  │
 │   .py    │ ◄────── │  no_mangle │ ◄────── │  Rust code  │
 └──────────┘         └────────────┘         └─────────────┘
                       PyO3 / Maturin
                       génèrent ce pont
```

Références :

- [The Rustonomicon — FFI](https://doc.rust-lang.org/nomicon/ffi.html)
- [cbindgen](https://github.com/mozilla/cbindgen) — génère des headers C depuis du Rust
- [Rust Reference — External blocks](https://doc.rust-lang.org/reference/items/external-blocks.html)

## Pourquoi interop Python - Rust

### Pro/Cons des langages

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

### Python écosystème natif

Le succès de Python repose en grande partie sur des libs natives exposées en Python — certaines sont déjà écrites en Rust :

- [Polars](https://pola.rs/) — DataFrame ultra-rapide
- [Ruff](https://github.com/astral-sh/ruff) — linter Python
- [uv](https://github.com/astral-sh/uv) — gestionnaire de packages Python
- [Pydantic v2](https://github.com/pydantic/pydantic-core) (pydantic-core) — validation de données
- [HuggingFace Tokenizers](https://github.com/huggingface/tokenizers) — tokenisation NLP
- [orjson](https://github.com/ijl/orjson) — sérialisation JSON
- [DataFusion](https://github.com/apache/datafusion-python) — moteur SQL analytique
- [Granian](https://github.com/emmett-framework/granian), [Robyn](https://robyn.tech/) — serveurs HTTP

### Et du Python dans du Rust ?

**Est-ce qu'il y a des cas intéressants pour appeler du Python depuis Rust ?**

- Oui : embarquer un interpréteur Python pour du scripting utilisateur ou des plugins ML
- Réutiliser un écosystème scientifique mature (NumPy, scikit-learn) depuis un service Rust
- Transition progressive d'un projet Python vers Rust

## Dans la pratique

Stack : **[PyO3](https://pyo3.rs/)** (bindings Rust ↔ Python) + **[Maturin](https://www.maturin.rs/)** (build & packaging wheels)

Setup minimal :

```toml
# Cargo.toml
[lib]
crate-type = ["cdylib"]

[dependencies]
pyo3 = { version = "0.28", features = ["abi3-py38"] }
```

```toml
# pyproject.toml
[build-system]
requires = ["maturin>=1.0"]
build-backend = "maturin"
```

Workflow : `maturin develop` (dev) ou `maturin build --release` (wheel).

### Exemple 1 : utiliser comrak pour faire du Markdown (live-code)

- Exposer `fn markdown_to_html(s: &str) -> String`
- Cas pédagogique idéal : zéro état partagé, juste une transformation pure
- `#[pyfunction]` + `#[pymodule]` — quelques lignes de code

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

Côté Python (après `maturin develop`) :

```python
from rusty_md import markdown_to_html
print(markdown_to_html("# Hello **world**"))
# <h1>Hello <strong>world</strong></h1>
```

### Exemple 2 : perf — analyser des logs structurés

- Log serveur, calculer avg / min / max / écart-type des temps de réponse
- Comparer :
  - (a) pur Python
  - (b) pur Rust (benchmark de référence)
  - (c) Python qui appelle Rust via PyO3
- Mesurer aussi le coût de conversion `PyList` ↔ `Vec` (overhead FFI)

Structure proposée : `ex_log_analyze/{src/lib.rs, bench.py, data/access.log}`

```rust
use pyo3::prelude::*;

#[pyfunction]
fn stats(durations_ms: Vec<f64>) -> (f64, f64, f64, f64) {
    let n = durations_ms.len() as f64;
    let sum: f64 = durations_ms.iter().sum();
    let avg = sum / n;
    let min = durations_ms.iter().copied().fold(f64::INFINITY, f64::min);
    let max = durations_ms.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    let var = durations_ms.iter().map(|x| (x - avg).powi(2)).sum::<f64>() / n;
    (avg, min, max, var.sqrt())
}
```

Bench Python (avec `pytest-benchmark` ou `timeit`) :

```python
import statistics, timeit
from log_analyze import stats as rust_stats

def py_stats(xs):
    return (statistics.mean(xs), min(xs), max(xs), statistics.stdev(xs))

xs = [...]  # 1M de mesures
print("py:  ", timeit.timeit(lambda: py_stats(xs), number=10))
print("rust:", timeit.timeit(lambda: rust_stats(xs), number=10))
```

Variantes à benchmarker :

- Passer un `Vec<f64>` (copie) vs un `&PyList` (zero-copy)
- Lire le log en Rust (`std::fs`) vs lire en Python et passer la liste
- Le coût FFI s'amortit sur le volume → un appel pour 1M de points >> 1M d'appels

### Exemple 3 : exécuter du code Python depuis Rust

- `Python::with_gil(|py| { ... })` — acquérir le GIL et exécuter du code Python
- Utile pour du scripting utilisateur, des plugins, piloter un écosystème ML
- Crate [`pyo3`](https://pyo3.rs/) des deux côtés

```rust
use pyo3::prelude::*;

fn main() -> PyResult<()> {
    Python::with_gil(|py| {
        // Importer un module Python
        let math = py.import("math")?;
        let pi: f64 = math.getattr("pi")?.extract()?;
        println!("π = {pi}");

        // Exécuter du code Python arbitraire
        py.run(c"print('Hello from embedded Python')", None, None)?;

        Ok(())
    })
}
```

Cargo.toml :

```toml
[dependencies]
pyo3 = { version = "0.28", features = ["auto-initialize"] }
```

### Récap : ce qu'on peut faire

- Modules (`#[pymodule]`), fonctions (`#[pyfunction]`), classes (`#[pyclass]`)
- Exceptions custom (`create_exception!`)
- Sous-modules, conversions de types (via `IntoPy`, `FromPyObject`)
- Interop NumPy via [`rust-numpy`](https://github.com/PyO3/rust-numpy)

### Récap : ce qu'on ne peut pas (facilement) faire

- Héritage multi-niveau Python depuis Rust
- Exposer des génériques Rust tels quels
- Mapping automatique traits Rust → protocoles Python

```rust
// ❌ Génériques exposés tels quels
#[pyclass]
struct Container<T> { /* ... */ }   // ne compile pas : pyclass ne supporte pas les génériques

// ✅ Solution : monomorphiser et exposer des types concrets
#[pyclass] struct IntContainer { /* ... */ }
#[pyclass] struct StrContainer { /* ... */ }
```

```rust
// ❌ Iterator Rust ne devient pas itérable Python tout seul
impl Iterator for MyType { /* ... */ }

// ✅ Solution : implémenter __iter__ et __next__ explicitement
#[pymethods]
impl MyType {
    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> { slf }
    fn __next__(&mut self) -> Option<i32> { /* ... */ }
}
```

```rust
// ❌ Héritage Python → Rust → Python (diamant) : impossible
// ✅ Composer plutôt qu'hériter, ou n'hériter que d'une classe Python depuis Rust
#[pyclass(extends = PyDict)]
struct MyDict { /* ... */ }
```

## Retour XP, challenges

### REX

**Wefox AI — Inference storage** : création d'une API client pour un microservice, pour les équipes Rust et pour les équipes Python.

**[Toboggan-py](https://github.com/ilaborie/toboggan/tree/master/toboggan-py)** — projet pédagogique, support de présentation.

Architecture (cf. `../toboggan/`) :

```text
toboggan-core/    ← logique métier pure (no_std capable)
toboggan-client/  ← client HTTP/WebSocket Rust
toboggan-py/      ← bindings PyO3 (cdylib, abi3-py38)
   └─ src/
      ├─ lib.rs        — déclaration du #[pymodule]
      ├─ toboggan.rs   — #[pyclass] Toboggan + #[pymethods]
      ├─ talk.rs, slides.rs, state.rs, client_info.rs
toboggan_py.pyi   ← type stubs pour IDE/mypy
```

Pattern utilisé : **Python sync, Rust async, Runtime Tokio partagé**

```rust
#[pyclass]
pub struct Toboggan {
    rt: tokio::runtime::Runtime,         // Runtime créé une fois
    talk: Arc<RwLock<TalkResponse>>,     // état partagé
    tx: UnboundedSender<Command>,        // canal vers la tâche WS
}

#[pymethods]
impl Toboggan {
    #[new]
    #[pyo3(signature = (host = "localhost", port = 8080))]
    pub fn __new__(host: &str, port: u16) -> PyResult<Self> { /* ... */ }

    #[getter]
    pub fn state(&self) -> PyResult<State> {
        let state = Arc::clone(&self.state);
        let s = self.rt.block_on(async { state.read().await.clone() });
        Ok(State(s))
    }

    pub fn next(&self) { self.send(Command::NextSlide); }
}
```

API publique côté Python :

```python
from toboggan_py import Toboggan
tbg = Toboggan("localhost", 8080)
tbg.next()
print(tbg.state)
```

### Challenges

**Asynchronisme**

- Possible : [`pyo3-async-runtimes`](https://github.com/PyO3/pyo3-async-runtimes) (ex `pyo3-asyncio`) fait le pont entre `tokio` et `asyncio`
- Piège : le GIL doit être relâché pendant les await Rust, sinon deadlock
- Alternative : runtime Tokio interne (bloquant côté Python, async côté Rust)

```text
   Python (asyncio)              Rust (tokio)
 ┌─────────────────┐          ┌──────────────────┐
 │  await client   │          │  tokio::Runtime  │
 │      .fetch()   │ ─────▶   │   ┌──────────┐   │
 │                 │          │   │ reqwest  │   │
 │  ◄───── Future ─┤          │   └──────────┘   │
 │  (résolue)      │          │                  │
 └─────────────────┘          └──────────────────┘
        ↑                              ↑
        │  pyo3-async-runtimes::tokio  │
        └─────── future_into_py ───────┘
```

```rust
// Pattern 1 : Runtime interne, API Python sync (toboggan-py)
#[pymethods]
impl Client {
    fn fetch(&self, url: &str) -> PyResult<String> {
        self.rt
            .block_on(async { reqwest::get(url).await?.text().await })
            .map_err(|e| PyRuntimeError::new_err(e.to_string()))
    }
}

// Pattern 2 : pyo3-async-runtimes, API Python async
#[pyfunction]
fn fetch(py: Python<'_>, url: String) -> PyResult<Bound<'_, PyAny>> {
    pyo3_async_runtimes::tokio::future_into_py(py, async move {
        reqwest::get(&url)
            .await
            .map_err(|e| PyRuntimeError::new_err(e.to_string()))?
            .text()
            .await
            .map_err(|e| PyRuntimeError::new_err(e.to_string()))
    })
}
```

```python
# Pattern 1 (sync)
client.fetch("https://example.com")

# Pattern 2 (async)
import asyncio
asyncio.run(fetch("https://example.com"))
```

**Intégration OpenTelemetry**

- Complexe : les spans Rust ne voient pas le contexte Python OTel et inversement
- Propagation de contexte cross-language non triviale
- Écosystème en cours d'évolution

```text
Python (otel-python)             Rust (opentelemetry-rust)
────────────────────             ─────────────────────────
trace_id: ABC                    trace_id: ???
span_id:  001                    span_id:  ???
   │                                 │
   │ ─── PyO3 call ──▶               │
   │                                 │   ❌ contexte perdu :
   │                                 │      les deux libs ont
   │                                 │      leur propre TLS
   │ ◄── return ───                  │

Solution : injecter/extraire W3C TraceContext aux frontières

   ┌──────────────────────┐
   │ Python : propagate.  │       ┌─────────────────────┐
   │   inject(carrier)    │ ────▶ │ Rust : Extractor    │
   │   → {"traceparent":  │       │  reconstruit le     │
   │      "00-ABC-001-01"}│       │  Context Rust       │
   └──────────────────────┘       └─────────────────────┘
```

Références :

- [W3C TraceContext](https://www.w3.org/TR/trace-context/)
- [opentelemetry-rust — propagator](https://docs.rs/opentelemetry/latest/opentelemetry/propagation/)

**Ownership des données : Python ou Rust ?**

- Côté Rust : `Py<T>` — référence comptée vers un objet Python
- Côté Python : refcount géré par CPython
- Le GIL comme « lock global » — attention aux fuites circulaires
- **GIL lui-même** : challenge pour la concurrence (voir [PEP 703](https://peps.python.org/pep-0703/) — free-threaded Python 3.13+)

```text
   Python heap (gc, refcount)
 ┌─────────────────────────────┐
 │  ┌──────────┐               │
 │  │ PyObject │  refcount: 2  │
 │  └─────▲────┘               │
 │        │                    │
 └────────┼────────────────────┘
          │  Py<PyAny>  (incrémente la refcount)
 ┌────────┼────────────────────┐
 │  Rust  │                    │
 │        │  struct MyClass {  │
 │        └─ obj: Py<PyAny>,   │
 │        }                    │
 └─────────────────────────────┘
```

Règles d'or :

- `Py<T>` = smart pointer côté Rust qui garde une refcount Python
- Le GIL est nécessaire pour **lire/écrire** un `PyObject` (`Python::with_gil` ou `Bound<'py, T>`)
- Pas de cycle Rust ↔ Python : le GC Python ne voit pas dans le tas Rust → fuite garantie
- `py.allow_threads(|| { ... })` : relâcher le GIL pendant du Rust pur (CPU-bound) — d'autres threads Python peuvent avancer

**Packaging multi-plateforme**

- Wheels : manylinux, musllinux, macOS universal2, Windows
- Maturin gère ça + CI avec `cibuildwheel` ou GitHub Actions

GitHub Actions minimal :

```yaml
# .github/workflows/release.yml
- uses: PyO3/maturin-action@v1
  with:
    target: ${{ matrix.target }}
    args: --release --out dist
    manylinux: auto
- run: maturin upload --skip-existing dist/*
```

### Tips

- **Réduire la surface d'échange** (cloisonner) : moins de franchissements FFI = moins d'overhead
- **Passer des batchs** plutôt que des appels unitaires (amortir le coût de passage FFI)
- **Python sync + Rust async** avec un Runtime Tokio partagé — créer le `Runtime` une fois, le réutiliser
- **Libérer le GIL** (`py.allow_threads(...)`) sur les sections Rust CPU-bound → le code Python concurrent peut respirer
- **Tester des deux côtés** : `pytest` côté Python, `cargo test` côté Rust — chaque côté a ses propres angles morts
- **Profiler avec [`py-spy`](https://github.com/benfred/py-spy)** — il voit aussi les frames natives Rust

```rust
// Tip 1+2 : batch (1 traversée FFI pour N éléments)
#[pyfunction]
fn process_batch(items: Vec<String>) -> Vec<usize> {
    items.iter().map(|s| s.len()).collect()
}

// Tip 3 : Runtime Tokio partagé (lazy, global)
use std::sync::OnceLock;
use tokio::runtime::Runtime;

static RT: OnceLock<Runtime> = OnceLock::new();
fn rt() -> &'static Runtime {
    RT.get_or_init(|| Runtime::new().expect("tokio runtime"))
}

// Tip 4 : libérer le GIL pour CPU-bound
#[pyfunction]
fn heavy(py: Python<'_>, data: Vec<u8>) -> Vec<u8> {
    py.allow_threads(|| compress(&data))
}

// Tip 5 : type stubs pour l'IDE (toboggan_py.pyi)
//   class Toboggan:
//       def __init__(self, host: str = "localhost", port: int = 8080) -> None: ...
//       @property
//       def state(self) -> State: ...
//       def next(self) -> None: ...
```

```bash
# Tip 6 : profiler en production
py-spy record -o profile.svg -- python my_script.py
# le flame graph contient les frames Python ET les frames Rust
```

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
- [cbindgen](https://github.com/mozilla/cbindgen) — génération de headers C depuis Rust
- [py-spy](https://github.com/benfred/py-spy) — profiler Python qui voit les frames natives

### Projets emblématiques (Rust exposé en Python)

- [Polars](https://pola.rs/) — DataFrame
- [Ruff](https://github.com/astral-sh/ruff) — linter Python
- [uv](https://github.com/astral-sh/uv) — gestionnaire de packages
- [pydantic-core](https://github.com/pydantic/pydantic-core) — cœur de Pydantic v2
- [HuggingFace Tokenizers](https://github.com/huggingface/tokenizers) — NLP
- [orjson](https://github.com/ijl/orjson) — JSON rapide
- [DataFusion Python](https://github.com/apache/datafusion-python) — SQL analytique
- [Granian](https://github.com/emmett-framework/granian), [Robyn](https://robyn.tech/) — serveurs HTTP
- [OpenDAL](https://github.com/apache/opendal), [Kreuzberg](https://github.com/Goldziher/kreuzberg) — API polyglottes

### Articles

- [« Making Python 100x faster with less than 100 lines of Rust »](https://ohadravid.github.io/posts/2023-03-rusty-python/) — Ohad Ravid
- [« How Pydantic V2 leverages Rust's Superpowers »](https://samuel.colvin.me/posts/2023-01-09-annotated-types-and-pydantic-v2/) — Samuel Colvin
- [Astral blog](https://astral.sh/blog) — Ruff, uv, la stack Rust pour Python
- [The Rustonomicon — FFI](https://doc.rust-lang.org/nomicon/ffi.html) — FFI Rust bas niveau
- [PEP 703 — Making the GIL Optional](https://peps.python.org/pep-0703/) — free-threaded Python 3.13+
- [PyO3 User Guide](https://pyo3.rs/latest/) — référence complète
- [W3C TraceContext](https://www.w3.org/TR/trace-context/) — propagation OTel cross-language

### Talks vidéos

- David Hewitt — « PyO3: From Python to Rust and Back Again » — mainteneur PyO3, EuroRust (chercher sur YouTube)
- Ritchie Vink — talks Polars (PyCon, EuroRust) — créateur de Polars
- Samuel Colvin — « How Pydantic V2 Was Made » — PyCon US
- « Calling Rust from Python » — talks PyO3 sur YouTube / conference.scipy.org
