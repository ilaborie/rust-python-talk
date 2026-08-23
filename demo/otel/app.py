"""Une « requête » Python qui traverse Rust, sans perdre ni ses logs ni sa trace.

À lancer via `opentelemetry-instrument` — voir le README.
"""

import logging

from opentelemetry import trace

import rusty_otel

# `otelTraceID` est injecté dans chaque record par l'auto-instrumentation
# (OTEL_PYTHON_LOG_CORRELATION=true). On le met dans le format pour voir, à
# l'œil, que les lignes Python et les lignes Rust portent la même trace.
LOG_FORMAT = "%(levelname)-5s %(name)-22s [trace=%(otelTraceID)s] %(message)s"

MARKDOWN = "# Hello **world**\n\nUn paragraphe, et une ~~rature~~.\n"


def main() -> None:
    # `import rusty_otel` a suffi : le `#[pymodule_init]` du module a posé les
    # deux ponts et enregistré son flush dans `atexit`. Rien à initialiser ici.
    logging.basicConfig(level=logging.INFO, format=LOG_FORMAT, force=True)

    logger = logging.getLogger("app")
    tracer = trace.get_tracer("app")

    # Le span Python qui servira de parent au span Rust.
    with tracer.start_as_current_span("handle-request"):
        logger.info("appel du moteur Rust")
        html = rusty_otel.render(MARKDOWN)
        logger.info("rendu reçu : %d octets", len(html))


if __name__ == "__main__":
    main()
