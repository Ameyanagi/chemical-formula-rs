//! # Prelude for chemical-formula
//! This module re-exports all the necessary types and functions for the chemical-formula crate.
//! It is intended to be used as a prelude for the crate.
//!
//! # Example
//! ```
//! use chemical_formula::prelude::*;
//! ```
//! This will import all the necessary types and functions for the crate.
//! You can import them individually as well.
pub use crate::element::{ChemicalFormula, ElementSymbol, FormulaError};
pub use crate::parser::parse_formula;
