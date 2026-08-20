+++
title = "4 options stratégiques"
classes = ["no_title"]
+++

# 4 options stratégiques

<!-- pause -->

1. **RIIR** — réécrire (partiellement) une lib Python en Rust pour la perf
   _ex : pydantic-core, polars_

<!-- pause -->

2. **API polyglotte** — un cœur Rust, exposé partout (Python, Go, Java, …)
   _ex : [OpenDAL](https://github.com/apache/opendal), [Kreuzberg](https://github.com/Goldziher/kreuzberg)_

<!-- pause -->

3. **Proto Python → Rust** — itérer vite, optimiser quand le besoin émerge

<!-- pause -->

4. 🤔 **A-t-on encore besoin de Python ?**
   _Rust : `polars`, `axum`, `candle`, [`evcxr`](https://github.com/evcxr/evcxr)_

<!-- notes -->

- Option 1 : la plus courante, low-risk, gain ciblé
- Option 2 : ambitieuse, demande de l'archi en couches (cf. toboggan)
- Option 3 : stratégie de migration progressive
- Option 4 : provoc, mais Rust mûrit côté DS/ML (candle, polars natif Rust)
- Pas de bonne réponse universelle — dépend du contexte
