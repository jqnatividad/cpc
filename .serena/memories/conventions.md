# Conventions

- Indentation: HARD TABS (`rustfmt.toml`: `hard_tabs=true`). Run `cargo fmt`.
- D128 literals via `d!(...)`, the `dec128!` alias: `use fastnum::{dec128 as d, D128}`.
- Input is case-insensitive (lexer lowercases).
- Stage error strings are prefixed by stage (see `mem:core` invariants).

## Units (`units.rs`)
- `create_units!` macro generates the `Unit` enum + `category()`, `weight()`, `singular()`, `plural()`.
- Declaration form: `Variant: (UnitType, weight, "singular", "plural")` (e.g. `NoUnit: (NoType, d!(1), "", "")`).
- `weight` = conversion factor within a `UnitType` (e.g. Second=1e9 ns, Minute=60e9). Conversion via `get_conversion_factor` / `convert`.

## Adding a unit
1. Add the variant to the `create_units!` invocation in `src/units.rs`.
2. Add a conversion test in units.rs `mod tests` (pattern: `convert_test(1000.0, Meter, Kilometer) == 1.0`).
3. Register spelling(s) in `src/lexer.rs` so the word lexes to `Token::Unit(...)`.

## Parser precedence
Recursive descent; precedence encoded by call order: parse_plus → parse_mult_level → parse_caret → parse_unary_high → parse_suffix → parse_highest. Change precedence by re-wiring these.

## Docs
- The README "API Usage" example must stay byte-identical to the `src/lib.rs` module doc-comment — that doc-comment is the tested doctest, the README itself is not compiled.
- README ` ```rs ` fences are NOT doctests (rustdoc only compiles `rust`/untagged blocks), so partial macro snippets there are documentation-only.

## Release
Update CHANGELOG.md → bump version in Cargo.toml → `cargo test` → git tag `v#.#.#` → GitHub release notes → `cargo publish`.