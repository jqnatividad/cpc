# Architecture — eval pipeline & units

`eval()` (`src/lib.rs`) wires three stages; each is its own module and returns `Result<_, String>`:

1. Lexer (`lexer.rs`) — `lex(input, allow_trailing_operators)` → `Vec<Token>`. Recognizes unit words, operator words/phrases (`plus`, `multiplied by`, `÷`, `µs`), named numbers, constants, functions. Case-insensitive. The Token/Operator/Constant/FunctionIdentifier/NamedNumber/LexerKeyword enums are defined in `lib.rs`, not here.
2. Parser (`parser.rs`) — `parse(&tokens)` → `AstNode` tree. Recursive descent ordered by precedence (call chain in `mem:conventions`).
3. Evaluator (`evaluator.rs`) — `evaluate(&ast)` walks the tree via `evaluate_node` → `Number`. Math/trig helpers: factorial, sqrt, cbrt, sin, cos, tan.

`Number { value: D128, unit: Unit }`; its `Display` impl chooses singular/plural unit names and formats the decimal.

## Units arithmetic (`units.rs`)
- Unit-aware ops: `add`, `subtract`, `multiply` (`actual_multiply`), `divide`, `modulo`, `pow`.
- Result-unit selection: `to_ideal_unit`, `to_ideal_joule_unit`, `convert_to_lowest`.
- `lookup.rs`: `lookup_named_number`, `lookup_factorial`.

`verbose=true` prints the lexed token vector, the parsed AstNode, and per-stage ms timings.