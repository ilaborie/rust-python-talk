//! Les deux ponts entre le monde Python et le monde Rust :
//!
//! - les **logs** : `pyo3-log` route les records `log` vers le module `logging`
//!   de Python (et la feature `log-always` de `tracing` fait que chaque event
//!   `tracing!` produit aussi un record `log`) ;
//! - le **contexte de trace** : Rust appelle Python pour lui demander où il en
//!   est dans sa trace, au format W3C TraceContext.

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

/// Gardé pour pouvoir flusher les spans avant que l'interpréteur ne s'arrête :
/// l'exporter écrit sur stdout, mais rien ne le pousse tout seul à la sortie.
static PROVIDER: OnceLock<SdkTracerProvider> = OnceLock::new();

/// Installe le subscriber `tracing` de l'extension.
///
/// Chaque module d'extension configure SA propre instance de `tracing` : deux
/// modules PyO3 ne partagent pas de subscriber.
pub fn init() {
    // Le propagateur W3C : c'est lui qui sait lire un en-tête `traceparent`.
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

    // `set_global_default`, et surtout PAS `.init()` : ce dernier installerait
    // aussi `tracing-log`, qui prendrait la place de `pyo3-log` comme logger
    // `log` global (panique au démarrage) et rebouclerait avec `log-always`.
    if let Err(err) = tracing::subscriber::set_global_default(subscriber) {
        tracing::warn!("subscriber déjà installé : {err}");
    }

    let _ = PROVIDER.set(provider);
}

/// Vide les spans en attente. À appeler avant la fin du process (`atexit`).
pub fn shutdown() {
    if let Some(provider) = PROVIDER.get() {
        let _ = provider.shutdown();
    }
}

/// Demande à Python où il en est dans sa trace, et le rejoue côté Rust.
///
/// `opentelemetry.propagate.inject` remplit le dictionnaire avec les en-têtes
/// W3C (`traceparent`, et `tracestate`/`baggage` s'il y en a) ; le propagateur
/// Rust les relit et reconstruit un [`Context`].
///
/// Échoue si OTel n'est pas installé côté Python — l'appelant décide quoi en
/// faire, ici on repart d'une racine.
pub fn python_context(py: Python<'_>) -> PyResult<Context> {
    let carrier = PyDict::new(py);
    py.import("opentelemetry.propagate")?
        .call_method1("inject", (&carrier,))?;
    let carrier = carrier.extract::<HashMap<String, String>>()?;
    let context = global::get_text_map_propagator(|prop| prop.extract(&carrier));
    Ok(context)
}
