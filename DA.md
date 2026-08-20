# Direction artistique — « Rust & Python : le menu parfait ? »

Document de référence pour la rédaction du deck. **Ce n'est pas une slide.**

Source unique : `public/img/rust-python.png` (1536×1024), déclinée en
`public/img/rust-python.webp` (249 Ko) pour la couverture.

---

## 1. Décodage de l'illustration

Un **foodtruck** garé dans la campagne, portière ouverte sur le comptoir.

| Élément | Ce que ça raconte pour la conf |
|---|---|
| Le **crabe en toque**, derrière le comptoir | Rust 🦀 : c'est lui qui **cuisine**, en coulisses |
| Le **serpent en cape**, devant le comptoir | Python 🐍 : c'est lui le **client**, il commande et consomme |
| Le **comptoir** qui les sépare | La frontière FFI / l'ABI C — le seul endroit où les deux se parlent |
| Le **sac en papier** qui passe le comptoir | Un appel : on passe des données, pas la casserole |
| L'**ardoise de menu** | Ce que la lib expose : l'API publique |
| Le **camion**, mobile, autonome | La wheel : un artefact qu'on emporte et qu'on déploie |

C'est la métaphore centrale, et elle est fidèle : **Python commande, Rust
cuisine, et le comptoir coûte quelque chose à franchir.**

> L'illustration contient aussi une couche de clins d'œil Monty Python
> (« Unladen Swallow », SPAM, « Ni! »). **On ne l'exploite pas dans le corps du
> deck** — décision assumée : l'image porte le gag, les slides restent sobres.

## 2. Palette

Échantillonnée directement dans l'illustration.

| Rôle | Hex | Origine |
|---|---|---|
| Accent primaire / Rust | `#B13F15` | carapace du crabe |
| Accent secondaire / Python | `#4B7F52` | cape du serpent, repoussée vers le vert |
| Fond clair / crème | `#E3C184` | enseigne du camion |
| Bleu-vert de marque | `#728B83` | carrosserie |
| Texte fort / ardoise | `#2D2B25` | tableau du menu |
| Surlignage | `#D9B758` | lettrage jaune du menu |
| Respiration | `#71B4EC` | ciel |

Le vert de la cape (`#546051`) est franchement gris une fois projeté sur fond
blanc : on garde la teinte mais on la sature, d'où `#4B7F52` — contraste 4,8:1
avec du texte blanc, au-dessus du seuil AA.

Usage : accent primaire pour les mots-clés Rust, secondaire pour Python, ardoise
pour le texte. Le reste des slides garde le rendu Toboggan par défaut — la
palette ne sert que dans les `<style>` inline (couverture, slide de fin).

## 3. Lexique

La table qui pilote les **titres de chapitre** (les `_part.md`) et la
couverture. Au niveau des slides, elle n'est plus qu'une réserve d'idées :
voir la règle 1 ci-dessous.

| Concept technique | Terme cuisine |
|---|---|
| Intro, contexte | Mise en bouche |
| ABI C, convention d'appel | La cuisine centrale, le passe-plat |
| `ctypes` / `cffi` / C-API vs PyO3 | Les ustensiles |
| Setup PyO3 + Maturin | La mise en place, les ingrédients |
| Live-code | Aux fourneaux |
| Le code final qui tourne | Le plat servi |
| Python appelé depuis Rust | Le service inversé |
| Retour d'expérience, production | Le coup de feu |
| Le projet de référence | Le plat signature |
| Async Rust → appel bloquant Python | Le passe-plat asynchrone |
| Traçabilité, OpenTelemetry | Suivre la commande |
| Difficultés, pièges | Allergènes & intolérances |
| Tips de terrain | Pourboires |
| Cas d'usage | Les combos du menu |
| Liens, références | La carte |
| Conclusion | L'addition |

## 4. Règles de rédaction

1. **La métaphore vit au niveau chapitre.** Les quatre `_part.md` et la
   couverture portent tout le lexique ; les titres de slides sont
   descriptifs et techniques, le corps aussi (du code, un diagramme, un
   tableau). Un clin d'œil qui se lit aussi bien au premier degré passe
   (« Tips », « Ce que ça coûte, ce que ça rapporte ») ; un titre qu'il faut
   décoder, non.
2. **Jamais de jeu de mots au prix de la clarté.** Si le public doit décoder le
   titre pour comprendre le sujet, le titre est raté.
3. **Pas de vocabulaire cuisine dans le code, les identifiants, les notes.** Les
   notes du présentateur sont des aide-mémoire, pas du spectacle.
4. **Émojis : 🦀 et 🐍 uniquement**, plus 🔥 pour la slide live-code. Pas de
   guirlande d'émojis alimentaires.
5. **Densité faible.** 30 min dont 15 de live-code : il reste ~45 s par slide.
   Une idée, trois lignes, on avance.
