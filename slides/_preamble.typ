// Préambule Typst du deck — repris tel quel par toboggan (build -o *.typ,
// toboggan pdf, /download.pdf du serveur) à la place de celui qu'il génère.
//
// Il remplace, il ne complète pas : ce fichier doit porter *tous* les imports
// dont les slides ont besoin (touying, codly, codly-languages, gentle-clues,
// mitex), le `subslide-preamble: none` sans lequel chaque titre s'affiche deux
// fois, et l'initialisation de codly.
//
// Les tailles ne sont pas devinées, elles reprennent les ratios du deck web
// (slides/_head.html) : --slide-h1-size: 230% → titre 2.3em, --slide-code-size:
// 70% → raw 0.7em, --slide-table-size: 90% → table 0.9em.

#import "@preview/touying:0.7.3": *
#import themes.simple: *
#import "@preview/codly:1.3.0": *
#import "@preview/codly-languages:0.1.1": *
#import "@preview/gentle-clues:1.3.1": *
#import "@preview/mitex:0.2.7": mi, mitex

// Le web utilise `system-ui` / `ui-monospace` ; en PDF ça donnerait le serif
// par défaut de Typst (Libertinus). Inter et JetBrains Mono en sont les plus
// proches, et sont les deux polices du poste. Les replis servent aux machines
// qui ne les ont pas : sans eux Typst tombe sur le serif sans rien dire.
#let sans-stack = ("Inter", "Helvetica Neue")
#let mono-stack = ("JetBrainsMono NFM", "Menlo")

#show: simple-theme.with(
  aspect-ratio: "16-9",
  // Le thème réaffiche `display-current-heading(level: 2)` en tête de page,
  // alors que toboggan a déjà écrit `== {titre}` dans le corps du slide.
  subslide-preamble: none,
  // 2em de marge sur 25pt de base, ça faisait 50pt de blanc de chaque côté.
  config-page(margin: 1.4em),
  // `..args` est transmis en dernier à `touying-slides` par `simple-theme`,
  // donc cet `init` remplace celui du thème (qui posait `text(size: 25pt)`,
  // soit ~11 lignes utiles par slide, d'où les débordements).
  config-methods(init: (self: none, body) => {
    set text(font: sans-stack, size: 16pt)
    show raw: set text(font: mono-stack, size: 0.7em)
    show heading: set text(font: sans-stack)
    show heading.where(level: 2): set text(size: 2.3em)
    show table: set text(size: 0.9em)
    set table(stroke: (_, y) => (bottom: 0.5pt + luma(200)))
    show table.cell.where(y: 0): strong
    // Ici et pas au premier niveau : au premier niveau, cette mise à jour
    // d'état est du contenu, et touying en fait une page blanche en tête.
    codly(languages: codly-languages)
    body
  }),
)
#show: codly-init.with()

// La couverture web n'affiche ni titre ni date : ils sont dans l'illustration
// (`classes = ["no_title"]` dans _cover.md, que l'export Typst ignore). toboggan
// émet quand même `#title-slide[titre, date, corps de _cover.md]` — d'où le
// titre en double, et une illustration sans hauteur qui poussait la couverture
// sur une deuxième page.
//
// On ne garde donc de ce corps que l'illustration et les marqueurs
// `<toboggan-slide>` (que `toboggan pdf` interroge pour son rapport de
// débordement) : `space`, `styled` (titre, date) et `v` passent à la trappe.
#let generated-title-slide = title-slide
#let title-slide(body) = generated-title-slide[
  #set image(height: 100%)
  #body.children.filter(child => child.func() in (metadata, image)).join()
]
