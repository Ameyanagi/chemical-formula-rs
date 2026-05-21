# Bindings and Publishing

This repository publishes three packages from one Rust core:

- Rust crate: `chemical-formula`
- npm package: `@ameyanagi/chemical-formula`
- PyPI package: `chemical-formula-rs`, imported as `chemical_formula_rs`

The Rust crate owns parsing, conversion, and error behavior. WASM/TypeScript and Python bindings are thin adapters over the shared summary API.

## Local Rust Checks

```cmd
cargo fmt --all --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
```

## WASM and npm

Use Bun for local package management and tests.

```cmd
cd bindings/wasm
bun install
bun run build
bun run lint
bun run format
bun test
npm pack --dry-run ./pkg
```

`bun run build` uses `wasm-pack` to generate `bindings/wasm/pkg`, then normalizes the generated package metadata to publish as `@ameyanagi/chemical-formula`.

The generated `pkg/` directory is not committed.

## Python and PyPI

Use uv for local Python workflows.

```cmd
cd bindings/python
uv sync --dev --no-install-project
uv run maturin develop
uv run ruff check .
uv run ruff format --check .
uv run pytest
uv run maturin build
```

The Python package exposes the Rust extension through `chemical_formula_rs`.

## Release Publishing

Releases are driven by Git tags like `v0.2.0`.

Before the first automated release:

- Configure npm trusted publishing for `@ameyanagi/chemical-formula`.
  - Workflow: `release.yml`
  - Environment: `npm`
- Configure PyPI trusted publishing for `chemical-formula-rs`.
  - Workflow: `release.yml`
  - Environment: `pypi`

The release workflow publishes with OIDC trusted publishing. It does not require long-lived npm or PyPI tokens.

## Public Binding API

TypeScript:

```ts
import { molecularWeight, parseFormula } from "@ameyanagi/chemical-formula";

const summary = parseFormula("1 wt % Pt / SiO2");
const mw = molecularWeight("H2O");
```

Python:

```python
import chemical_formula_rs as cf

summary = cf.parse_formula("1 wt % Pt / SiO2")
mw = cf.molecular_weight("H2O")
```

Summary objects contain:

- `formula`
- `elements`
- `stoichiometry`
- `wt_percent`
