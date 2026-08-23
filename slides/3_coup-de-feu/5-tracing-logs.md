+++
title = "Les logs passent"
classes = ["no_title"]
+++

<style>
/* Plusieurs blocs Rust et un avant/après sur la même slide : sans ça, le
   titre sort par le haut en 1920×1080. Même réglage que « Le résultat ». */

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

# Les logs passent

```rust
// tracing = { version = "0.1", features = ["log-always"] }  ── Cargo.toml
#[pymodule]
mod rusty_otel {
    #[pymodule_export]
    use super::render;

    #[pymodule_init]
    fn init(m: &Bound<'_, PyModule>) -> PyResult<()> {
        pyo3_log::init();    // log → logging Python
        telemetry::init();   // subscriber tracing + exporter OTel
        Ok(())
    }
}
```

<!-- pause -->

```python
import rusty_otel   # les ponts sont posés à l'import : rien à appeler
```

<!-- pause -->

```text
INFO app         [trace=6a79…5545] appel du moteur Rust
INFO rusty_otel  [trace=6a79…5545] rendu de 53 octets de markdown
```

→ Logger nommé d'après le **`target` Rust**, `trace_id` du span **Python**.

<!-- notes -->

- `pyo3-log` implémente `log::Log` et route chaque record vers `logging.getLogger(target)`
- `log-always` est indispensable : sans elle, `tracing` n'émet vers `log` que s'il n'y a AUCUN subscriber — or on en installe un pour OTel
- `#[pymodule_init]` est le hook du module déclaratif : il tourne à l'`import`, donc l'appelant Python n'a rien à initialiser
- `#[pymodule_export]` garde `render` à la racine du crate : sinon son `target` devient `rusty_otel::rusty_otel`, et le logger Python aussi
- Le module y enregistre aussi son `shutdown` dans `atexit` — encore un appel Rust → Python — sinon les spans ne sont jamais flushés
- Contrepartie assumée : poser un subscriber `tracing` GLOBAL à l'import, c'est confisquer une ressource du process. Acceptable pour une app, discutable pour une lib
- Piège vécu : `tracing_subscriber().init()` fait paniquer le module. Il installe `tracing-log` comme logger `log` global, place déjà prise par `pyo3-log`
- Le correctif : `tracing::subscriber::set_global_default(subscriber)`, qui ne touche qu'à `tracing`. Bonus : pas de boucle log → tracing → log
- La doc `pyo3-log` demande de configurer `logging` avant le premier record ; mesuré en 0.13, les deux ordres marchent (le cache de niveaux est revalidé)
- Le `trace_id` est là parce que `pyo3-log` passe par `Logger.makeRecord`, donc par la `LogRecordFactory` d'OTel
- Corollaire : un log émis depuis un thread tokio n'aura PAS le trace_id — le contextvar Python n'y est pas
- Côté Python, `logging.getLogger("rusty_otel").setLevel(...)` pilote le niveau du Rust
