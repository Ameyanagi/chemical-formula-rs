## Improve `chemical-formula-rs` with WASM-Ready Core (Rust-only first)

### Summary
Focus first on correctness and error handling, then make the core crate compile/test cleanly on `wasm32-unknown-unknown` without adding a JS wrapper yet.  
Current status from repo checks:
1. `cargo test` passes.
2. `cargo clippy` reports many warnings, including API-quality issues.
3. `cargo check --target wasm32-unknown-unknown` failed only because the target is not installed locally, not because of a confirmed crate-level incompatibility yet.

### Public API / Interface Changes
1. Change parser return type from `Result<ChemicalFormula, Box<dyn Error>>` to a concrete error:
   - `pub fn parse_formula(s: &str) -> Result<ChemicalFormula, FormulaError>`
2. Implement `FromStr` for `ElementSymbol`:
   - `impl core::str::FromStr for ElementSymbol`
   - Keep `from_str` as compatibility shim for one release, then deprecate.
3. Expand `FormulaError` to structured parse/validation errors:
   - `ParseError { input: String, position: Option<usize>, reason: ... }`
   - `InvalidElementSymbol(String)`
   - `InvalidNumber(String)`
   - `WeightPercentOverflow`
   - `DivisionByZero` (for normalization edge cases)
4. Keep current formula data model (`ChemicalFormula`) unchanged in this phase to avoid breaking downstream users.

### Implementation Plan
1. Parser hardening (`src/parser.rs`, `src/formula.pest`)
   - Remove panic paths (`unwrap`, `unreachable!`) from non-test code.
   - Validate numeric grammar more strictly (no empty number, controlled sign handling).
   - Return descriptive parse errors with context.
2. Core math safety (`src/element.rs`)
   - Remove internal `unwrap` in library methods (example: `molecular_weight` path).
   - Guard against zero totals in `to_wt_percent`, `to_mol_percent`, and related normalization.
   - Replace `mem::replace` pattern with direct assignment where appropriate.
3. API hygiene
   - Remove deprecated `Error::description` usage.
   - Reduce warning noise by fixing unused imports/variables and test warning cleanup.
4. WASM target readiness (Rust-only)
   - Add CI job for `cargo check --target wasm32-unknown-unknown`.
   - Add a minimal wasm-target smoke test (compile-time is enough for this phase).
   - Document local setup: `rustup target add wasm32-unknown-unknown`.
5. Documentation updates
   - Add "WASM support" section in `README.md` with current scope:
     - Supported: core crate compilation/use on wasm target.
     - Not yet included: JS bindings (`wasm-bindgen`) in this phase.

### Test Cases and Scenarios
1. Parser correctness
   - Valid: `H2O`, `Pt5wt%/SiO2`, `(Pt5wt%/SiO2)50wt%(CeO2)50wt%`
   - Invalid: empty input, malformed numbers, invalid element symbol, broken parentheses
2. Error behavior
   - Ensure invalid inputs return `FormulaError` variants, never panic.
3. Numeric edge cases
   - `wt%` sum `>100` returns `WeightPercentOverflow`
   - Zero-total normalization paths return explicit error, not `NaN`/panic.
4. Platform matrix
   - Native: `cargo test`, `cargo clippy`
   - WASM: `cargo check --target wasm32-unknown-unknown`

### Assumptions and Defaults
1. Scope is Rust-only wasm support first (selected), no JS wrapper crate yet.
2. Priority is correctness/error quality first (selected), then wasm CI hardening.
3. Backward compatibility is preferred; breaking API changes are minimized to parser error typing and trait additions.
4. `std` remains enabled for now; no `no_std` migration in this phase.
