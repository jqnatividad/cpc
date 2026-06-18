# Web Frontend (`web/`)

SvelteKit 2 / Svelte 5 single-page calculator UI, deployed to Vercel (cpc.kasper.space).

- Files: UI entry `web/src/routes/+page.svelte`; layout `+layout.svelte` / `+layout.ts`; helpers `web/src/lib/helpers.ts`; global styles `app.css`.
- Uses the Rust core via WASM: depends on the package built to `/pkg` at repo root (`package.json`: `"cpc": "file:../pkg"`). The single WASM export is `wasm_eval(expr) -> String` (in `src/lib.rs`, cfg wasm32), returning the result or `"Error: …"`.
- `npm run dev` / `npm run build` rebuild WASM first (`build-wasm` = `wasm-pack build --target bundler`). Must be run from `web/`.
- Vite config uses vite-plugin-wasm + vite-plugin-top-level-await.
- Deploy: `.github/workflows/deploy.yml` on push to `main` → `npm ci` → `npm run build` → `vercel deploy --prebuilt --prod`.