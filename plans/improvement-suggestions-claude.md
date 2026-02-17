# Implementation Plan: chemical-formula-rs Improvements

## Context

chemical-formula-rs v0.1.0 is a Pest-grammar-based chemical formula parser. This plan covers 6 improvement tasks (WASM deferred). All changes are based on actual file inspection.

---

## Task 1: Fix Compiler Warnings

**Files:** `src/parser.rs`, `src/element.rs`

### src/parser.rs
- **Line 20**: Remove unused `FormulaError` import — change to `use crate::element::{ChemicalFormula, ElementSymbol};`

### src/element.rs
- **Line 640**: Prefix unused `element` with `_` → `for (_element, stoichiometry)`
- **Line 644**: Prefix unused `element` with `_` → `for (_element, wt_ratio)`
- **Line 696**: Remove `mut` from `let mut molecular_weight_residue` → `let molecular_weight_residue`
- **Line 764**: Prefix unused `element` with `_` → `.map(|(_element, stoichiometry)| stoichiometry)`
- **Line 918**: Prefix `mem::replace` return with `let _ =`
- **Line 953**: Remove empty line between doc comment and `add_formula` function

### Test snake_case fixes (src/parser.rs)
- Lines 162-163: `expected_MW` → `expected_mw`
- Line 179: `expected_Pt_wt` → `expected_pt_wt`, `expected_SiO2_wt` → `expected_sio2_wt`
- Lines 200-202: `expected_Pt_wt` → `expected_pt_wt`, `expected_Au_wt` → `expected_au_wt`, `expected_SiO2_wt` → `expected_sio2_wt`

### Test unused variable fixes (src/element.rs)
- Lines 997-1004: Remove unused `O_mol_ratio` and `H_mol_ratio` in `test_formula`
- Lines 1039-1042: Remove unused `H_mol_ratio` in `test_formula_no_element`
- Lines 1082-1085: Remove unused `O_mol_ratio` in `test_no_wt_percent`
- Lines 1118-1125: Remove unused `O_mol_ratio` and `H_mol_ratio` in `test_add_formula`
- Lines 1163-1170: Remove unused `O_mol_ratio` and `H_mol_ratio` in `test_multiply_formula`

### Test naming fixes (src/parser.rs)
- Line 159: `test_chmical_formula_parser` → `test_chemical_formula_parser` (typo)
- Line 174: `test_chmical_formula_parser_wt_percent` → `test_chemical_formula_parser_wt_percent`
- Line 197: `test_chmical_formula_parser_nested` → `test_chemical_formula_parser_nested`

---

## Task 2: Harden Parser Error Handling

**File:** `src/parser.rs`

### Change `parse_formula_pairs` signature
- Current: `fn parse_formula_pairs(pair: Pair<Rule>) -> ChemicalFormula`
- New: `fn parse_formula_pairs(pair: Pair<Rule>) -> Result<ChemicalFormula, Box<dyn Error>>`
- Add a new `FormulaParseError` variant to `FormulaError` (or use `Box<dyn Error>`) for internal parse failures

### Specific changes
- **Line 57** (`.parse().unwrap()` for number): → `.parse().map_err(|e| ...)?`
- **Line 65-68** (`.unwrap()` on weight_percent inner): → `.ok_or_else(|| ...)?` and `.parse().map_err(...)?`
- **Line 70** (`.unwrap()` on `multiply_wt_percent`): → `?`
- **Lines 72, 75** (`unreachable!()`): → `return Err(...)` with descriptive error
- **Line 83** (`.unwrap()` on element): → `.ok_or_else(|| ...)?`
- **Line 84** (`.unwrap()` on stoichiometry rule): → `.ok_or_else(|| ...)?`
- **Line 107, 109-110** (`.unwrap()` on wt% inner): → `.ok_or_else(|| ...)?` and `.parse().map_err(...)?`
- **Lines 114, 125** (`unreachable!()`): → `return Err(...)` with descriptive error
- **Line 147** (`.unwrap()` on pairs.next()): → `.ok_or_else(|| ...)?`

### Approach: Add `ParseError` variant to `FormulaError`
```rust
pub enum FormulaError {
    FileIOError,
    FileParseError,
    WeightPercentOverflow,
    NoFormula,
    ParseError(String),  // NEW
}
```
- Update `Display` and `Error` impls to handle `ParseError`
- Change `parse_formula` return type from `Result<ChemicalFormula, Box<dyn Error>>` to `Result<ChemicalFormula, FormulaError>`
- Wrap pest parse errors into `FormulaError::ParseError`
- Remove `use std::error::Error;` import from parser.rs (replaced by FormulaError)

### Also fix in element.rs
- **Line 793** (`molecular_weight` method): `.unwrap()` on `to_molecular_formula()` → use `?` operator

---

## Task 3: Add `Display`/`FromStr` Traits

**Files:** `src/element.rs`, `src/lib.rs`

### `Display` for `ElementSymbol`
Add `impl std::fmt::Display for ElementSymbol` that uses a match to return the symbol string (e.g., `"H"`, `"He"`, `"Si"`). The `None` variant displays as `""`.

### `Display` for `ChemicalFormula`
Add `impl std::fmt::Display for ChemicalFormula` that renders:
- Stoichiometric elements: `H2O`, `SiO2` (omit subscript 1)
- Weight percent elements appended: `Pt5wt%`
- Combined: `Pt5wt%SiO2`

### `FromStr` for `ElementSymbol`
Convert the existing `ElementSymbol::from_str()` method (lines 393-515) into a proper `impl std::str::FromStr for ElementSymbol` trait implementation. Keep the inherent method for backward compatibility or remove it (the trait is re-exported via prelude).

Note: `FromStr` requires an `Err` type. Use a simple unit error or a new `ParseElementError` type. The current `from_str` returns `ElementSymbol::None` for unknown strings — `FromStr` should return `Err` for unknown elements, which is a behavior change. **Decision:** Keep the inherent `from_str()` as-is for internal use (returns `None` variant). Add `FromStr` that returns `Err` for unknown strings, using `FormulaError::ParseError`.

### `FromStr` for `ChemicalFormula`
```rust
impl std::str::FromStr for ChemicalFormula {
    type Err = FormulaError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        crate::parser::parse_formula(s)
    }
}
```
This delegates to the existing `parse_formula`. Requires Task 2 to be done first so the return type is `Result<_, FormulaError>`.

---

## Task 4: Add `serde` Support (Feature-Gated)

**Files:** `Cargo.toml`, `src/element.rs`

### Cargo.toml changes
```toml
[features]
default = []
serde = ["dep:serde"]

[dependencies]
serde = { version = "1", features = ["derive"], optional = true }
```

### src/element.rs changes
- Add conditional derive on `ElementSymbol`:
  ```rust
  #[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
  #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
  pub enum ElementSymbol { ... }
  ```
- Add conditional derive on `ChemicalFormula`:
  ```rust
  #[derive(Debug, Clone, Default)]
  #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
  pub struct ChemicalFormula { ... }
  ```
- Add conditional derive on `FormulaError`:
  ```rust
  #[derive(Debug)]
  #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
  pub enum FormulaError { ... }
  ```

### Dev dependencies
Add `serde_json` for testing:
```toml
[dev-dependencies]
serde_json = "1"
```

---

## Task 5: Expand Tests

**Files:** `src/parser.rs`, `src/element.rs`

### New parser tests (src/parser.rs)
- `test_parse_empty_string` — empty string should return an empty formula (or error)
- `test_parse_single_element` — `"H"` → H:1.0
- `test_parse_with_decimal_stoichiometry` — `"H2.5O"`
- `test_parse_nested_groups` — `"Ca(OH)2"`
- `test_parse_multiple_groups` — `"Mg3(PO4)2"`
- `test_parse_invalid_element` — `"Xx"` should error
- `test_parse_wt_overflow` — `"H60wt%O60wt%"` should error on to_wt_percent
- `test_parse_complex_formula` — `"(NH4)2SO4"`

### New element tests (src/element.rs)
- `test_element_display` — verify Display for ElementSymbol
- `test_formula_display` — verify Display for ChemicalFormula
- `test_formula_fromstr` — verify `"H2O".parse::<ChemicalFormula>()`
- `test_formula_roundtrip` — parse → display → parse yields same molecular weight
- `test_serde_roundtrip` — (behind `#[cfg(feature = "serde")]`) serialize/deserialize ChemicalFormula

### Approach
Add tests directly to existing `#[cfg(test)] mod tests` blocks in each file. No new files needed.

---

## Task 6: Fix TODOs and Metadata

### Cargo.toml (line 12-13)
- Fix typo: `"chemisty"` → `"chemistry"`
- Fix categories to valid crates.io values: `["science", "parser-implementations"]`
- Remove `#TODO` comment

### CHANGELOG.md
- Line 1: `TODO_CRATE_NAME` → `chemical-formula`
- Line 13: Remove `TODO: Date` placeholder, replace with `Unreleased`

### SECURITY.md
- Line 12: Remove `.todo_crate` suffix
- Line 33: `TODO_CRATE_NAME` → `chemical-formula`

---

## Execution Order & Dependencies

```
Task 1 (warnings) ──┐
                     ├──→ Task 2 (error handling) ──→ Task 3 (Display/FromStr)
                     │                                        │
                     │    Task 4 (serde) ─────────────────────┤
                     │                                        │
                     └──→ Task 6 (metadata) ──────────→ Task 5 (tests, last)
```

Tasks 1 and 6 can be done independently. Task 2 must precede Task 3 (FromStr needs FormulaError). Task 5 (tests) should be last as it tests all new functionality.

---

## Verification

1. `cargo test` — all existing + new tests pass
2. `cargo clippy -- -D warnings` — zero warnings
3. `cargo test --features serde` — serde feature builds and tests pass
4. `cargo doc` — docs build correctly
