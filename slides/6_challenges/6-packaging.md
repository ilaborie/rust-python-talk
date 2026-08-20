+++
title = "Packaging"
classes = ["no_title"]
+++

# Packaging multi-plateforme

Wheels nécessaires :
- **manylinux** (glibc), **musllinux** (Alpine)
- **macOS** universal2 (arm64 + x86_64)
- **Windows** (x86_64, arm64)

<!-- pause -->

```yaml
# .github/workflows/release.yml
- uses: PyO3/maturin-action@v1
  with:
    target: ${{ matrix.target }}
    args: --release --out dist
    manylinux: auto
- run: maturin upload --skip-existing dist/*
```

<!-- pause -->

→ `maturin-action` orchestre le cross-compile + upload PyPI.

<!-- notes -->

- Avant : galère sans nom (cross-compile, gcc multilib, …)
- Maintenant : maturin-action fait le boulot
- abi3-py38 : 1 wheel par plateforme/arch, pas par version Python
- Sans abi3 : nx9 wheels (n plateformes × 9 versions Python supportées)
- Penser à publier les sdist aussi (source distribution)
