# Task Completion

## Rust changes
- `cargo test` must pass (includes `mod tests` unit tests + doctests).
- `cargo fmt` (hard tabs).
- `cargo clippy` advisable (some lints allow-listed in Cargo.toml).

## Web changes (`web/`)
- `npm run check` (svelte-check) clean.
- `npm run build` succeeds (also rebuilds WASM into `/pkg`).

## CI mirrors
- `.github/workflows/test.yml`: `cargo test` on every push/PR.
- `deploy.yml`: builds + deploys `web/` to Vercel on push to `main`.
- `release.yml`: builds release binaries on tag.