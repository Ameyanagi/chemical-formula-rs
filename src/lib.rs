// chemical-formula-rs
// Copyright (c) 2024 Ameyanagi <contact@ameyanagi.com>
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! # chemical-formula-rs
//!
//! This crate provides a simple way to parse and manipulate chemical formulas including weight percent and nested formulas.
//!
//! The initial motivation was to parse a formula such as `Pt5wt%/SiO2`, which are heavily used annotation in the field of Heterogeneous catalysis.
//! Another motivatation was to parse a formula that is nested such as `(Pt5wt%/SiO2)50wt%(CeO2)50wt%`, which can be used to describe a composite material.
//! We also provide a way simple API to convert between molecular formula and weight percent.
//!
//! ## Installation
//!
//! Please use [cargo-edit](https://crates.io/crates/cargo-edit) to always add the latest version of this library:
//!
//! ```cmd
//! cargo add chemical-formula
//! ```
//!
//! ## Example
//!
//! ```rust
//! use chemical_formula::prelude::*;
//!
//! fn main() {
//!     let formula = parse_formula("H2O").unwrap();
//!
//!     println!("Orignal formula: {:?}", formula);
//!     // Orignal formula: ChemicalFormula { element: {O, H}, stoichiometry: {H: 2.0, O: 1.0}, wt_percent: {} }
//!
//!     println!("Molecular weight: {:?}", formula.molecular_weight());
//!     // Molecular weight: Ok(18.015)
//!
//!     println!("Wt%: {:?}", formula.to_wt_percent().unwrap());
//!     // Wt%: ChemicalFormula { element: {O, H}, stoichiometry: {}, wt_percent: {H: 11.19067443796836, O: 88.80932556203165} }
//!
//!     let formula = parse_formula("Pt5wt%/SiO2").unwrap();
//!
//!     println!("Orignal formula: {:?}", formula);
//!     // Orignal formula: ChemicalFormula { element: {Si, O, Pt}, stoichiometry: {Si: 1.0, O: 2.0}, wt_percent: {Pt: 5.0} }
//!
//!     println!(
//!         "Molecular Formula: {:?}",
//!         formula.to_molecular_formula().unwrap()
//!     );
//!     // Molecular Formula: ChemicalFormula { element: {Si, O, Pt}, stoichiometry: {Si: 1.0, Pt: 0.016209751480873558, O: 2.0}, wt_percent: {} }
//!
//!     println!("Wt%: {:?}", formula.to_wt_percent().unwrap());
//!     // Wt%: ChemicalFormula { element: {Si, O, Pt}, stoichiometry: {}, wt_percent: {Pt: 5.0, Si: 44.406487692026026, O: 50.59351230797397} }
//!
//!     let formula = parse_formula("(Pt5wt%/SiO2)50wt%(CeO2)50wt%").unwrap();
//!
//!     println!("Orignal formula: {:?}", formula);
//!     // Orignal formula: ChemicalFormula { element: {Si, Ce, Pt, O}, stoichiometry: {}, wt_percent: {Si: 22.203243846013017, O: 34.59233931398559, Pt: 2.5000000000000004, Ce: 40.70441684000139} }
//!
//!     println!("Wt%: {:?}", formula.to_wt_percent().unwrap());
//!     // Wt%: ChemicalFormula { element: {Si, Ce, Pt, O}, stoichiometry: {}, wt_percent: {Si: 22.203243846013017, O: 34.59233931398559, Pt: 2.5000000000000004, Ce: 40.70441684000139} }
//! }
//! ```
//!
//! ## License
//!
//! Licensed under either of
//!
//! - Apache License, Version 2.0
//!   ([LICENSE-APACHE](https://github.com/Ameyanagi/chemical-formula-rs/blob/main/LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
//! - MIT license
//!   ([LICENSE-MIT](https://github.com/Ameyanagi/chemical-formula-rs/blob/main/LICENSE-MIT) or <http://opensource.org/licenses/MIT>)
//!
//! at your option.
pub mod element;
pub mod parser;
pub mod prelude;
