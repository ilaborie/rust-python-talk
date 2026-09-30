+++
title = "Deux traces séparées"
classes = ["no_title", "data-table"]
+++

<style>
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
/* Rendu de tableau calqué sur Bootstrap 5.3 :
   — Reboot : `th { text-align: inherit }`, pour que l'en-tête suive
     l'alignement des cellules au lieu du `center` du navigateur ;
   — `.table` : filet de 1px sous chaque ligne ;
   — `.table-group-divider` : filet de 2px en `currentColor` entre `thead` et
     `tbody`, seule séparation forte du tableau.
   Padding vertical en `em` du tableau (et pas les `0.5rem` de Bootstrap) :
   proportionnel à la résolution comme le reste du deck, et calé pour que les
   7 lignes de « Ce que PyO3 doit réconcilier » tiennent dans la scène. */
section.data-table table {
	width: 100%;
	border-collapse: collapse;
}

section.data-table th,
section.data-table td {
	padding: 0.3em 0.5em;
	text-align: inherit;
	vertical-align: top;
	border-bottom: 1px solid #dee2e6;
}

section.data-table thead th {
	vertical-align: bottom;
}

section.data-table tbody {
	border-top: 2px solid currentColor;
}
</style>

# Deux traces séparées

```mermaid:width=88%,alt=Le contexte de trace ne traverse pas la frontière PyO3
flowchart LR
    PY["Python · otel-python<br/>trace_id ABC · span_id 001"]
    FF["frontière PyO3"]
    RS["Rust · opentelemetry-rust<br/>trace_id DEF · span_id 002"]
    PY --> FF
    FF -. "contexte perdu" .-> RS
    style PY fill:#4B7F52,stroke:#36603C,color:#ffffff
    style RS fill:#B13F15,stroke:#8a3010,color:#ffffff
```

<!-- pause -->

**Une requête, deux traces** : impossible de suivre l'appel de bout en bout.

<!-- notes -->

- Les deux SDK OTel gèrent leur propre contexte : Python utilise notamment des contextvars, Rust a son propre contexte de tracing ; aucun pont n'est posé automatiquement par PyO3
- Symptôme en prod : deux traces orphelines au lieu d'une, impossible de corréler
- Même problème pour les logs : `tracing` côté Rust n'atterrit dans le `logging` Python qu'avec un pont explicite
- Les deux ponts sont indépendants : on peut avoir les logs sans les traces, et l'inverse
- 1 min. Montrer d'abord le symptôme, puis l'objectif sur la slide suivante, avant le code des ponts
- La démo qui tourne : `demo/otel/`, les deux SDK sur la console, on compare les trace_id à l'œil
