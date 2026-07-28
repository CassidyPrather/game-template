---
name: game-template
description: Cassidy's opinionated template for small Rust web toys — macroquad, wasm, static deploy
---

# game-template

A template for small web toys: Rust + macroquad, compiled to wasm, shipped as a
static page. Native desktop builds also work. Crate `game-template`, lib
`game_template`, bin `game-template`. Code is AGPL-3.0-or-later.

## Repo Map

- `src/sim.rs` — the simulation. Pure, deterministic, no macroquad. Most work
  belongs here.
- `src/main.rs` — thin macroquad frontend. Window, draw calls, input gathering.
- `build.rs` — embeds a `git describe` version string.
- `web/index.html`, `web/gl.js` — the static shell and the vendored miniquad
  loader. Zero external requests; honors `prefers-color-scheme` and
  `prefers-reduced-motion`.
- `scripts/build-web.sh` — wasm build → `dist/web/`.
- `.github/workflows/ci-cd.yml` — lint, test, audit, size-budgeted web bundle,
  Pages deploy, release artifacts.
- `benches/` — criterion bench over the sim. Unit tests live in `src/sim.rs`.

## Commands

```bash
cargo build                                                   # build
cargo run                                                     # run natively
cargo clippy --all-targets --all-features -- -D warnings      # lint
cargo clippy --target wasm32-unknown-unknown -- -D warnings   # lint, wasm
cargo fmt                                                     # format
cargo test                                                    # test
./scripts/build-web.sh                                        # wasm -> dist/web/
python3 -m http.server --directory dist/web 8080              # serve it
cargo bench --bench sim_bench -- --quick                      # bench
cargo audit                                                   # audit
```

The web build needs `rustup target add wasm32-unknown-unknown`, and uses
`wasm-opt` from binaryen when it is on PATH.

## The Determinism Contract

The sim advances on a fixed 60 Hz timestep via an accumulator with a frame-dt
clamp, seeded through `fastrand`, and receives input only as an `InputFrame`
struct — so a given seed plus a given input sequence always produces the same
run, and rendering interpolates between the last two states using an alpha.

Do not read wall-clock time, macroquad state, or randomness from inside
`src/sim.rs`. If the frontend needs to tell the sim something, it goes in
`InputFrame`.

## House Rules

Every asset gets a `CREDITS.md` line at intake — source, author, license, URL.
CC0 first.

CI enforces a hard wasm size budget (`MAX_WASM_BYTES`, ~1.5 MB by default);
if a change blows it, shrink the change or retune the budget deliberately.

Audio is off by default. Enabling the macroquad `audio` feature also requires
adding the quad-snd `audio.js` miniquad plugin to `web/`.

See `docs/GETTING_STARTED.md` for framework and asset-source links.
