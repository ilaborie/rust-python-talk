import logging

from opentelemetry import trace

# Import + init par le module
import rusty_otel

# `otelTraceID` est injecté par l'auto-instrumentation
# (OTEL_PYTHON_LOG_CORRELATION=true)
LOG_FORMAT = "%(levelname)-5s %(name)-22s [trace=%(otelTraceID)s] %(message)s"

MARKDOWN = "# Hello **world**\n\nUn paragraphe, et une ~~rature~~.\n"


def main() -> None:
    logging.basicConfig(level=logging.INFO, format=LOG_FORMAT, force=True)

    logger = logging.getLogger("app")
    tracer = trace.get_tracer("app")

    # Span parent
    with tracer.start_as_current_span("handle-request"):
        logger.info("appel du moteur Rust")
        html = rusty_otel.render(MARKDOWN)
        logger.info("rendu reçu : %d octets", len(html))


if __name__ == "__main__":
    main()
