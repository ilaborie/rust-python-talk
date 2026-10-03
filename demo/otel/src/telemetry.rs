use std::collections::HashMap;
use std::sync::OnceLock;

use opentelemetry::Context;
use opentelemetry::global;
use opentelemetry::trace::TracerProvider as _;
use opentelemetry_sdk::propagation::TraceContextPropagator;
use opentelemetry_sdk::trace::SdkTracerProvider;
use pyo3::prelude::*;
use pyo3::types::PyDict;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::layer::SubscriberExt as _;

static PROVIDER: OnceLock<SdkTracerProvider> = OnceLock::new();

pub fn init() {
    global::set_text_map_propagator(TraceContextPropagator::new());

    let exporter = opentelemetry_stdout::SpanExporter::default();
    let provider = SdkTracerProvider::builder()
        .with_simple_exporter(exporter)
        .build();
    let tracer = provider.tracer("rusty-otel");
    global::set_tracer_provider(provider.clone());

    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    let subscriber = tracing_subscriber::registry()
        .with(filter)
        .with(tracing_opentelemetry::layer().with_tracer(tracer));

    // Evite d'installer le tracing-log et utiliser py03-log à la place
    if let Err(err) = tracing::subscriber::set_global_default(subscriber) {
        tracing::warn!("subscriber déjà installé : {err}");
    }

    let _ = PROVIDER.set(provider);
}

pub fn shutdown() {
    if let Some(provider) = PROVIDER.get() {
        let _ = provider.shutdown();
    }
}

/// Récupère le context Otel de Python, et le construit pour Rust
pub fn python_context(py: Python<'_>) -> PyResult<Context> {
    let carrier = PyDict::new(py);
    py.import("opentelemetry.propagate")?
        .call_method1("inject", (&carrier,))?;
    let carrier = carrier.extract::<HashMap<String, String>>()?;
    let context = global::get_text_map_propagator(|prop| prop.extract(&carrier));
    Ok(context)
}
