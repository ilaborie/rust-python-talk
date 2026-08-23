+++
title = "Rust & Python : le menu parfait ?"
date = "2026-09-29"
classes = ["no_title", "center", "wide"]
+++

<style>
/* Le titre est dans l'illustration : la couverture n'a rien d'autre à afficher,
   donc l'image prend toute la scène. */
section.cover article {
	/* `align-self` : la section est en `align-items: center`, sans ça l'article
	   ne fait que la largeur de son contenu. `flex: 1` + `min-height: 0` pour
	   qu'il prenne toute la hauteur et que le `flex-basis: 0` du <p> ait une
	   référence — sinon la hauteur reste celle du contenu et l'image ne
	   remplit pas la scène. */
	flex: 1 1 0;
	align-self: stretch;
	min-height: 0;
	justify-content: center;
	padding: 0;
}

/* Le markdown enveloppe l'image dans un <p>.
   `flex: 1 1 0` (et pas `auto`) : la base est nulle, donc la hauteur du <p>
   sort entièrement de la répartition flex — elle est connue avant que l'image
   ne soit mesurée, et le `height: 100%` de l'image peut s'y résoudre. */
section.cover p {
	flex: 1 1 0;
	min-height: 0;
	display: flex;
	justify-content: center;
	margin: 0;
}

section.cover img {
	/* `height: 100%` et pas `max-height` : `max-*` ne fait que rétrécir, jamais
	   agrandir. L'illustration fait 1536×1024, elle serait restée à cette
	   taille au milieu d'une scène 1920 ou 3840. Mesuré : sans ça, 1536 px de
	   large sur un projecteur 4K.
	   `width: auto` garde le ratio ; `max-width` + `object-fit` ne servent que
	   sur une scène plus étroite que 3:2, où l'image se recentre au lieu de
	   déborder. Pas d'ombre ni de coins arrondis : l'image touche les bords
	   hauts et bas, ils seraient rognés par l'`overflow: hidden` de la scène. */
	height: 100%;
	width: auto;
	max-width: 100%;
	object-fit: contain;
}
</style>

![Rust & Python — Le menu parfait ? Un foodtruck de campagne baptisé « Unladen Swallow » : un crabe en toque, derrière le comptoir, tend un sac en papier à un serpent encapuchonné qui attend devant l'ardoise du menu](/public/img/rust-python.webp)

<!-- notes -->

- Se présenter, 20 s max
- L'illustration : Python appelle, Rust exécute, et le passage entre les deux a un coût
- Format : ~30 min, dont **15 min de live-code** — donc peu de slides, on avance
- Plan : pourquoi c'est possible → on le fait en vrai → ce que ça donne en prod → l'addition
- Questions à la fin
