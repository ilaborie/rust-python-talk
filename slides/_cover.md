+++
title = "Rust & Python : le menu parfait ?"
date = "2026-09-29"
classes = ["no_title", "center", "wide"]
+++

<style>
/* L'illustration porte la couverture : elle prend toute la hauteur que le
   titre et le sous-titre ne consomment pas, quelle que soit la résolution.
   `min-height: 0` est nécessaire pour qu'un enfant flex accepte de rétrécir. */
section.cover article {
    justify-content: flex-start;
    min-height: 0;
    gap: 0.1em;
}

section.cover h1 {
    flex: 0 0 auto;
    font-size: 400%;
    font-weight: 700;
    color: #2D2B25;
    letter-spacing: -0.02em;
    margin: 0;
}

section.cover h2 {
    flex: 0 0 auto;
    font-size: 200%;
    font-weight: 400;
    font-style: italic;
    color: #728B83;
    margin: 0 0 0.25em;
}

/* Le markdown enveloppe l'image dans un <p>. */
section.cover p {
    /* `flex: 1 1 0` (et pas `auto`) : la base est nulle, donc la hauteur du <p>
       sort entièrement de la répartition flex — elle est définie avant que
       l'image ne soit mesurée, et son `max-height: 100%` peut se résoudre. */
    flex: 1 1 0;
    min-height: 0;
    display: flex;
    justify-content: center;
    align-items: center;   /* sinon le flex étire la boîte et `contain` letterboxe */
    margin: 0;
}

section.cover img {
    /* La boîte reste au ratio de l'image — pas de letterbox, l'ombre et les
       coins arrondis épousent l'illustration. */
    max-height: 100%;
    max-width: 100%;
    width: auto;
    height: auto;
    border-radius: 0.5em;
    box-shadow: 0 0.3em 1.4em rgba(45, 43, 37, 0.35);
}
</style>

# Rust & Python

## Le menu parfait ?

![Un foodtruck de campagne : un crabe en toque, derrière le comptoir, tend un sac en papier à un serpent encapuchonné qui attend devant l'ardoise du menu](/public/img/rust-python.webp)

<!-- notes -->

- Se présenter, 20 s max
- L'illustration : Python appelle, Rust exécute, et le passage entre les deux a un coût
- Format : ~30 min, dont **15 min de live-code** — donc peu de slides, on avance
- Plan : pourquoi c'est possible → on le fait en vrai → ce que ça donne en prod → l'addition
- Questions à la fin
