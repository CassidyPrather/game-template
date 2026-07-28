# Game Template

Cassidy's opinionated template for small Rust web toys. Enter at your own peril.

Rust + [macroquad](https://macroquad.rs/), compiled to wasm, deployed as a
static page. The narrower sibling of
[rust-template](https://github.com/CassidyPrather/rust-template), which handles
the CLI end of things. Native desktop builds work too.

## What You Get

- **Split architecture** — `src/sim.rs` is a pure, macroquad-free simulation
  library; `src/main.rs` is a thin macroquad frontend
- **Fixed 60 Hz timestep** — accumulator, frame-dt clamp, render interpolation
  alpha. The boring part, already done
- **Seedable determinism** — `fastrand` plus a single `InputFrame` struct as the
  only input channel. Same seed, same inputs, same run
- **Web pipeline** — `scripts/build-web.sh` → `dist/web/`, `wasm-opt` if you
  have it, zero external requests at runtime
- **Size budget in CI** — the build fails if the optimized wasm gets fat
- **GitHub Pages deploy** on every push to `main`
- **Strict CI** — fmt, clippy (native *and* wasm), tests, `cargo audit`
- **CC0-first assets** with [CREDITS.md](CREDITS.md) bookkeeping
- **Version stamping** — `build.rs` embeds `git describe` output

## The Demo

A particle fidget. Delete it and write your own.

| Input   | Effect            |
| ------- | ----------------- |
| Mouse   | Attract particles |
| Click   | Burst             |
| `Space` | Pause             |
| `R`     | Reseed            |

Seed and build version are drawn on-screen, which is how you file a useful bug
report against a random universe.

Audio is off. The macroquad `audio` feature is disabled by default — smaller
wasm, and browsers block autoplay anyway. Turning it on later means adding the
`quad-snd` `audio.js` miniquad plugin to `web/`.

## Development

Requires [Rust](https://rustup.rs/)

Build: `cargo build`

Run (native): `cargo run`

Lint: `cargo clippy --all-targets --all-features -- -D warnings`

Lint (wasm): `cargo clippy --target wasm32-unknown-unknown -- -D warnings`

Format: `cargo fmt`

Test: `cargo test`

Web build: `./scripts/build-web.sh` (needs
`rustup target add wasm32-unknown-unknown`; uses `wasm-opt` from
[binaryen](https://github.com/WebAssembly/binaryen) if installed)

Serve the result: `python3 -m http.server --directory dist/web 8080`

### Advanced

Benchmark: `cargo bench --bench sim_bench -- --quick`

Security audit: `cargo audit` (requires `cargo install cargo-audit`)

Pre-commit hook: `git config core.hooksPath .githooks` (runs `cargo fmt`)

## Deploying

CI builds `dist/web/` and publishes it to GitHub Pages on every push to `main`.
One-time setup: repo **Settings → Pages → Source: "GitHub Actions"**. Releases
also carry native binaries and a zipped web bundle.

Or skip Pages entirely and copy `dist/web/` into a static site — for example a
`toys/<name>/` directory on `cassidyprather.github.io`, which is what ends up
on wirenook.net. It is a folder of files with no external requests; it will run
anywhere you can put a folder of files.

## Template Setup

1. **Create a new repository** from this template on GitHub (click "Use this
   template")

2. **Clone your new repository** and navigate to it

3. **Update project metadata** in `Cargo.toml`:
   - Change `name` to your toy's name (kebab-case, e.g. `orbit-fidget`)
   - Update the `[lib]` name to match (snake_case, e.g. `orbit_fidget`)
   - Update the `[[bin]]` name to match (kebab-case)
   - Update `description`, `repository`, and `keywords`
   - Update dependencies as required

4. **Update the binary name everywhere it is hardcoded**:
   - `web/index.html`: the `.wasm` filename it loads
   - `scripts/build-web.sh`: `BIN=`
   - `.github/workflows/ci-cd.yml`: `BIN=` in the package step

5. **Update `.vscode/launch.json`** (if using VS Code):
   - Replace the old crate name in the `cargo` sections

6. **Tune `MAX_WASM_BYTES`** in `.github/workflows/ci-cd.yml` — the default
   budget is sized for a toy, not for your sprite atlas

7. **Update the README**:
   - Replace the title and description
   - Remove or customize this Template Setup section

8. **Verify everything works**:
   ```bash
   cargo fmt --check
   cargo clippy --all-targets --all-features -- -D warnings
   cargo test
   ./scripts/build-web.sh
   ```

## House Rules

**Every asset gets a [CREDITS.md](CREDITS.md) line at intake.** Source, author,
license, URL. Thirty seconds now versus archaeology later.

**CC0 first.** Prefer it and the bookkeeping mostly disappears.

**Accessibility floor.** Keyboard-operable where it makes sense. Respect
`prefers-reduced-motion`. Do not rely on color alone. Audio muted by default
with a visible toggle.

**Code is AGPL-3.0-or-later** (see [LICENSE](LICENSE)). Assets carry their own
licenses; that is what CREDITS.md is for.

## Starting a New Toy

See [docs/GETTING_STARTED.md](docs/GETTING_STARTED.md) — frameworks, asset
sources, shader references, and the rest of the link pile.
