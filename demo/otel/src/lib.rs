//! Un module d'extension minimal, mais observable : ses logs atterrissent dans
//! le `logging` de Python, et ses spans dans la trace ouverte par Python.

use pyo3::prelude::*;
use tracing_opentelemetry::OpenTelemetrySpanExt as _;

mod telemetry;

/// Convertit du Markdown en HTML.
#[pyfunction]
fn render(py: Python<'_>, input: &str) -> String {
    // LES lignes : le parent du span Rust est le span courant… côté Python.
    let span = tracing::info_span!("render", bytes = input.len());
    let parent = telemetry::python_context(py).unwrap_or_default();
    if let Err(err) = span.set_parent(parent) {
        tracing::warn!("contexte Python non rattaché : {err}");
    }
    let _entered = span.enter();

    tracing::info!("rendu de {} octets de markdown", input.len());
    // Le GIL n'est pas nécessaire pour convertir : on le rend le temps du calcul.
    let html = py.detach(|| comrak::markdown_to_html(input, &comrak::Options::default()));
    tracing::info!(octets = html.len(), "rendu terminé");
    html
}

/// Vide les spans en attente. Enregistré dans `atexit` par le module.
#[pyfunction]
fn shutdown() {
    telemetry::shutdown();
}

/// Module déclaratif : les deux ponts sont posés à l'import, côté Python il n'y
/// a rien à appeler.
///
/// `#[pymodule_export]` garde les fonctions à la racine du crate : leur `target`
/// `tracing` reste `rusty_otel`, donc le logger Python aussi.
#[pymodule]
mod rusty_otel {
    #[pymodule_export]
    use super::render;
    #[pymodule_export]
    use super::shutdown;

    use pyo3::prelude::*;

    use crate::telemetry;

    #[pymodule_init]
    fn init(m: &Bound<'_, PyModule>) -> PyResult<()> {
        pyo3_log::init(); // log → logging Python
        telemetry::init(); // subscriber tracing + exporter OTel

        // L'exporter n'est poussé par personne : on programme le flush ici
        // plutôt que de le demander à l'appelant Python.
        m.py()
            .import("atexit")?
            .call_method1("register", (m.getattr("shutdown")?,))?;
        Ok(())
    }
}
