use pyo3::prelude::*;
use tracing_opentelemetry::OpenTelemetrySpanExt as _;

mod telemetry;

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

/// Appeler dans le `atexit` du module.
#[pyfunction]
fn shutdown() {
    telemetry::shutdown();
}

#[pymodule]
mod rusty_otel {
    #[pymodule_export]
    use super::render;
    #[pymodule_export]
    use super::shutdown;

    use pyo3::prelude::*;

    use crate::telemetry;

    // Appeler au démarrage du module
    #[pymodule_init]
    fn init(m: &Bound<'_, PyModule>) -> PyResult<()> {
        pyo3_log::init(); // log → logging Python
        telemetry::init(); // subscriber tracing + exporter OTel

        // Hook pour la fin du module (nettoyage)
        m.py()
            .import("atexit")?
            .call_method1("register", (m.getattr("shutdown")?,))?;
        Ok(())
    }
}
