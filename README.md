# Game Template

Cassidy's opinionated template for small Rust web toys. Enter at your own peril.

Rust + [macroquad](https://macroquad.rs/), compiled to wasm, deployed as a
static page. Based on [rust-template](https://github.com/CassidyPrather/rust-template)
Native desktop builds work too.

Sound is wired up and works in the browser, which is fiddlier than it sounds:
it needs macroquad's `audio` feature, quad-snd's `audio.js` plugin in `web/`,
and something to do about browsers refusing to make noise before the user
clicks. The effects themselves are synthesised in `src/synth.rs` rather than
loaded, so there are no audio assets to ship or credit. `M` mutes.

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

See [docs/GETTING_STARTED.md](docs/GETTING_STARTED.md) for generated tips: Frameworks, asset
sources, shader references, and so on and so forth.
