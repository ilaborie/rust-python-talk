import logging
import sys

# Équivalent de `opentelemetry-instrument` ; les variables OTEL_* viennent de .mise.toml
from opentelemetry.instrumentation.auto_instrumentation import initialize

initialize()

from opentelemetry import trace

# Import + init par le module
import rusty_otel

# `otelTraceID` est injecté par l'auto-instrumentation
# (OTEL_PYTHON_LOG_CORRELATION=true)
LOG_FORMAT = "%(levelcolor)s%(levelname)-5s%(reset)s %(name)-22s [trace=%(otelTraceID)s] %(message)s"

MARKDOWN = "# Hello **world**\n\nUn paragraphe, et une ~~rature~~.\n"

LEVEL_COLORS = {
    logging.DEBUG: "\033[36m",  # cyan
    logging.INFO: "\033[32m",  # vert
    logging.WARNING: "\033[33m",  # jaune
    logging.ERROR: "\033[31m",  # rouge
    logging.CRITICAL: "\033[1;31m",  # rouge gras
}
RESET = "\033[0m"


class LevelColorFormatter(logging.Formatter):
    """Colore le niveau (logs Python et Rust via pyo3-log), seulement dans un terminal."""

    def __init__(self, fmt: str, use_color: bool) -> None:
        super().__init__(fmt)
        self.use_color = use_color

    def formatMessage(self, record: logging.LogRecord) -> str:
        record.levelcolor = LEVEL_COLORS.get(record.levelno, "") if self.use_color else ""
        record.reset = RESET if self.use_color else ""
        return super().formatMessage(record)


def main() -> None:
    handler = logging.StreamHandler()
    handler.setFormatter(LevelColorFormatter(LOG_FORMAT, use_color=sys.stderr.isatty()))
    logging.basicConfig(level=logging.INFO, handlers=[handler], force=True)

    logger = logging.getLogger("app")
    tracer = trace.get_tracer("app")

    # Span parent
    with tracer.start_as_current_span("handle-request"):
        logger.info("appel du moteur Rust")
        html = rusty_otel.render(MARKDOWN)
        logger.info("rendu reçu : %d octets", len(html))


if __name__ == "__main__":
    main()
