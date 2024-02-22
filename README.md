# chemical-formula-rs

[![Lib.rs](https://img.shields.io/badge/Lib.rs-*-84f)](https://lib.rs/crates/chemical-formula)
[![Crates.io](https://img.shields.io/crates/v/chemical-formula)](https://crates.io/crates/chemical-formula)
[![Docs.rs](https://docs.rs/chemical-formula/badge.svg)](https://docs.rs/chemical-formula)

![Rust 1.74](https://img.shields.io/static/v1?logo=Rust&label=&message=1.74&color=grey)

<!-- [![CI](https://github.com/Ameyanagi/chemical-formula-rs/workflows/CI/badge.svg?branch=develop)](https://github.com/Ameyanagi/chemical-formula-rs/actions?query=workflow%3ACI+branch%3Adevelop) -->

![Crates.io - License](https://img.shields.io/crates/l/chemical-formula/0.1.0)

[![GitHub](https://img.shields.io/static/v1?logo=GitHub&label=&message=%20&color=grey)](https://github.com/Ameyanagi/chemical-formula-rs)
[![open issues](https://img.shields.io/github/issues-raw/Ameyanagi/chemical-formula-rs)](https://github.com/Ameyanagi/chemical-formula-rs/issues)
[![open pull requests](https://img.shields.io/github/issues-pr-raw/Ameyanagi/chemical-formula-rs)](https://github.com/Ameyanagi/chemical-formula-rs/pulls)
[![good first issues](https://img.shields.io/github/issues-raw/Ameyanagi/chemical-formula-rs/good%20first%20issue?label=good+first+issues)](https://github.com/Ameyanagi/chemical-formula-rs/contribute)

This crate provides a simple way to parse and manipulate chemical formulas including weight percent and nested formulas.

The initial motivation was to parse a formula such as `Pt5wt%/SiO2`, which are heavily used annotation in the field of Heterogeneous catalysis.
Another motivatation was to parse a formula that is nested such as `(Pt5wt%/SiO2)50wt%(CeO2)50wt%`, which can be used to describe a composite material.
We also provide a way simple API to convert between molecular formula and weight percent.

## Installation

Please use [cargo-edit](https://crates.io/crates/cargo-edit) to always add the latest version of this library:

```cmd
cargo add chemical-formula
```

## Example

```rust
use chemical_formula::prelude::*;

fn main() {
    let formula = parse_formula("H2O").unwrap();

    println!("Orignal formula: {:?}", formula);
    // Orignal formula: ChemicalFormula { element: {O, H}, stoichiometry: {H: 2.0, O: 1.0}, wt_percent: {} }

    println!("Molecular weight: {:?}", formula.molecular_weight());
    // Molecular weight: Ok(18.015)

    println!("Wt%: {:?}", formula.to_wt_percent().unwrap());
    // Wt%: ChemicalFormula { element: {O, H}, stoichiometry: {}, wt_percent: {H: 11.19067443796836, O: 88.80932556203165} }

    let formula = parse_formula("Pt5wt%/SiO2").unwrap();

    println!("Orignal formula: {:?}", formula);
    // Orignal formula: ChemicalFormula { element: {Si, O, Pt}, stoichiometry: {Si: 1.0, O: 2.0}, wt_percent: {Pt: 5.0} }

    println!(
        "Molecular Formula: {:?}",
        formula.to_molecular_formula().unwrap()
    );
    // Molecular Formula: ChemicalFormula { element: {Si, O, Pt}, stoichiometry: {Si: 1.0, Pt: 0.016209751480873558, O: 2.0}, wt_percent: {} }

    println!("Wt%: {:?}", formula.to_wt_percent().unwrap());
    // Wt%: ChemicalFormula { element: {Si, O, Pt}, stoichiometry: {}, wt_percent: {Pt: 5.0, Si: 44.406487692026026, O: 50.59351230797397} }

    let formula = parse_formula("(Pt5wt%/SiO2)50wt%(CeO2)50wt%").unwrap();

    println!("Orignal formula: {:?}", formula);
    // Orignal formula: ChemicalFormula { element: {Si, Ce, Pt, O}, stoichiometry: {}, wt_percent: {Si: 22.203243846013017, O: 34.59233931398559, Pt: 2.5000000000000004, Ce: 40.70441684000139} }

    println!("Wt%: {:?}", formula.to_wt_percent().unwrap());
    // Wt%: ChemicalFormula { element: {Si, Ce, Pt, O}, stoichiometry: {}, wt_percent: {Si: 22.203243846013017, O: 34.59233931398559, Pt: 2.5000000000000004, Ce: 40.70441684000139} }
}
```

## License

Licensed under either of

- Apache License, Version 2.0
  ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license
  ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

See [CONTRIBUTING](CONTRIBUTING.md) for more information.

## [Code of Conduct](CODE_OF_CONDUCT.md)

## [Changelog](CHANGELOG.md)

## Versioning

`chemical-formula-rs` strictly follows [Semantic Versioning 2.0.0](https://semver.org/spec/v2.0.0.html) with the following exceptions:

- The minor version will not reset to 0 on major version changes (except for v1).  
  Consider it the global feature level.
- The patch version will not reset to 0 on major or minor version changes (except for v0.1 and v1).  
  Consider it the global patch level.

This includes the Rust version requirement specified above.  
Earlier Rust versions may be compatible, but this can change with minor or patch releases.

Which versions are affected by features and patches can be determined from the respective headings in [CHANGELOG.md](CHANGELOG.md).

<!-- Note that dependencies of this crate may have a more lenient MSRV policy! -->
<!-- Please use `cargo +nightly update -Z minimal-versions` in your automation if you don't generate Cargo.lock manually (or as necessary) and require support for a compiler older than current stable. -->
