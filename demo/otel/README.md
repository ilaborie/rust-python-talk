# Observabilité à travers la frontière PyO3

Support des slides `3_coup-de-feu/5-tracing-logs.md` et `6-tracing-context.md`.

Une app Python auto-instrumentée OpenTelemetry appelle une extension Rust qui
utilise `tracing`. Sans rien, on obtient **deux traces orphelines** et les logs
Rust n'apparaissent nulle part. Ici, les deux ponts sont en place :

| Pont | Sens | Comment |
|---|---|---|
| Les logs | Rust → Python | `pyo3-log` + la feature `log-always` de `tracing` |
| Le contexte de trace | Python → Rust | Rust appelle `opentelemetry.propagate.inject` |

## Lancer

```bash
uv venv --python 3.12
uv pip install maturin opentelemetry-distro opentelemetry-instrumentation-logging
./.venv/bin/maturin develop

env OTEL_SERVICE_NAME=demo-otel OTEL_PYTHON_LOG_CORRELATION=true \
  ./.venv/bin/opentelemetry-instrument \
    --traces_exporter console --metrics_exporter none --logs_exporter none \
    ./.venv/bin/python app.py
```

Aucune infra : les deux SDK exportent sur la console, et on compare à l'œil.

## Ce qu'on doit voir

```text
INFO  app         [trace=6a79e3fe28c4f2e8f5f71c3db9875545] appel du moteur Rust
INFO  rusty_otel  [trace=6a79e3fe28c4f2e8f5f71c3db9875545] rendu de 53 octets de markdown
INFO  rusty_otel  [trace=6a79e3fe28c4f2e8f5f71c3db9875545] rendu terminé octets=79
INFO  app         [trace=6a79e3fe28c4f2e8f5f71c3db9875545] rendu reçu : 79 octets
```

1. **Les logs Rust sont des logs Python** : le logger s'appelle `rusty_otel`
   (le `target` Rust), le format est celui de `logging.basicConfig`.
2. **Ils portent le `trace_id` du span Python** : `otelTraceID` est injecté par
   l'auto-instrumentation, et `pyo3-log` passe par `Logger.makeRecord`, donc par
   la `LogRecordFactory` d'OTel.
3. **Une seule trace** : le span Rust `render` et le span Python
   `handle-request` partagent le `TraceId`, et le `ParentSpanId` du Rust est le
   `SpanId` du Python.

```text
Rust    Name : render            TraceId : 6a79…5545   ParentSpanId : 86c1602bdc1c1aca
Python  name : handle-request    trace_id: 0x6a79…5545  span_id      : 0x86c1602bdc1c1aca
```

Contrôle automatisable — il ne doit sortir **qu'un seul** identifiant :

```bash
… python app.py 2>&1 | rg -o '[0-9a-f]{32}' | sort -u
```

## Ce qu'on a appris en l'écrivant

- **`.init()` de `tracing-subscriber` fait paniquer le module.** Il installe
  aussi `tracing-log` comme logger `log` global, place déjà prise par
  `pyo3-log` → `SetLoggerError`. On utilise
  `tracing::subscriber::set_global_default`, qui ne touche qu'à `tracing`.
  Bonus : ça évite la boucle `log → tracing → log` avec `log-always`.
- **`#[pymodule_init]` suffit pour tout initialiser.** Les deux ponts sont posés
  à l'`import`, et le module enregistre lui-même son `shutdown` dans `atexit` :
  côté Python il n'y a rien à appeler. Contrepartie assumée : poser un
  subscriber `tracing` *global* à l'import, c'est confisquer une ressource du
  process — acceptable pour une app, discutable pour une bibliothèque.
- **`#[pymodule_export]` n'est pas cosmétique.** Déclarer `render` *dans* le
  `mod rusty_otel` donnerait le `target` `rusty_otel::rusty_otel`, donc un
  logger Python `rusty_otel.rusty_otel`. En le gardant à la racine du crate et
  en l'exportant, le logger s'appelle `rusty_otel`.
- **L'ordre d'initialisation, finalement pas un piège.** La doc `pyo3-log`
  demande de configurer `logging` avant le premier record Rust ; mesuré en
  0.13, les deux ordres fonctionnent — le cache de niveaux est revalidé.

## Détail

- `src/telemetry.rs` — les deux ponts et l'init du subscriber.
- `src/lib.rs` — `render()`, et le `#[pymodule_init]` qui câble le tout.
- `app.py` — l'app : `logging`, un span, un appel Rust.

La ligne `++ render; bytes=53 span=1` dans la sortie est le record de *création*
de span produit par `log-always` : `log` n'a pas la notion de span, il la rend
comme un événement. Bruit connu, prix à payer pour ce pont-là.
