# Tech Stack

## Rust crate
- Rust edition 2024. `crate-type = ["cdylib", "rlib"]`.
- Numbers: `fastnum` 0.2 `D128` (128-bit decimal float). `dec128!` macro imported as `d` throughout. Decimal (not binary) float is deliberate for accuracy.
- `unicode-segmentation` for lexing; `web-time` for `Instant` (wasm-compatible timing).
- WASM-only deps (cfg target_arch="wasm32"): `wasm-bindgen` 0.2, `js-sys` 0.3, `console_error_panic_hook`.
- dev-dep: `regex` (tests only).
- Cargo.toml allow-lists clippy lints: comparison_chain, if_same_then_else, match_like_matches_macro, get_first.

## Web (`web/`)
- SvelteKit 2 + Svelte 5, Vite 6, Tailwind 4, TypeScript; `@sveltejs/adapter-vercel`.
- WASM glue: vite-plugin-wasm + vite-plugin-top-level-await + wasm-pack.
- Package manager: npm.
- Depends on the locally built WASM crate: `"cpc": "file:../pkg"`.