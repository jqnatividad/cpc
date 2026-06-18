# Suggested Commands

## Rust crate (repo root)
- `cargo run -- '100ms to s'` — run CLI.
- `cargo run -- '<expr>' --verbose` (or `-v`) — print lexed tokens, parsed AST, per-stage ms timings.
- `cargo test` — all tests (most live in `mod tests` inside each src file, plus doctests).
- `cargo test <substring>` — run a subset by name.
- `cargo test --doc` — doctests only (lib.rs module example + `eval` + `Number`).
- `cargo build [--release]`.
- `cargo fmt`.

## Web (`web/`, run from that dir)
- `npm install`
- `npm run dev` — builds WASM (wasm-pack --target bundler → `/pkg`) then `vite dev`.
- `npm run build` — builds WASM then SvelteKit build.
- `npm run build-wasm` — WASM only.
- `npm run check` — `svelte-check` type check.

## CLI flags
`--version`, `--help`, `-v`/`--verbose`. First non-flag arg is the expression; a second unexpected arg errors and exits 1; no expression prints help.