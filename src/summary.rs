//! Binding-friendly summary helpers for parsed formulas.

use crate::element::{ChemicalFormula, ElementSymbol, FormulaError};
use crate::parser::parse_formula;
use std::collections::BTreeMap;

/// Deterministic, string-keyed representation suitable for foreign-language bindings.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct FormulaSummary {
    pub formula: String,
    pub elements: Vec<String>,
    pub stoichiometry: BTreeMap<String, f64>,
    pub wt_percent: BTreeMap<String, f64>,
}

fn sorted_elements(formula: &ChemicalFormula) -> Vec<ElementSymbol> {
    let mut elements = formula
        .element
        .iter()
        .copied()
        .filter(|element| *element != ElementSymbol::None)
        .collect::<Vec<_>>();
    elements.sort_by_key(|element| *element as u16);
    elements
}

fn summary_map(values: &std::collections::HashMap<ElementSymbol, f64>) -> BTreeMap<String, f64> {
    values
        .iter()
        .filter(|(element, _)| **element != ElementSymbol::None)
        .map(|(element, value)| (element.to_string(), *value))
        .collect()
}

/// Convert an existing formula to a deterministic summary.
pub fn summarize_formula(formula: &ChemicalFormula) -> FormulaSummary {
    FormulaSummary {
        formula: formula.to_string(),
        elements: sorted_elements(formula)
            .into_iter()
            .map(|element| element.to_string())
            .collect(),
        stoichiometry: summary_map(&formula.stoichiometry),
        wt_percent: summary_map(&formula.wt_percent),
    }
}

/// Parse a formula and return a deterministic summary of the parsed representation.
pub fn parse_formula_summary(input: &str) -> Result<FormulaSummary, FormulaError> {
    parse_formula(input).map(|formula| summarize_formula(&formula))
}

/// Parse and convert a formula to molecular formula representation.
pub fn to_molecular_formula_summary(input: &str) -> Result<FormulaSummary, FormulaError> {
    let formula = parse_formula(input)?;
    formula
        .to_molecular_formula()
        .map(|formula| summarize_formula(&formula))
}

/// Parse and convert a formula to normalized wt% representation.
pub fn to_wt_percent_summary(input: &str) -> Result<FormulaSummary, FormulaError> {
    let formula = parse_formula(input)?;
    formula
        .to_wt_percent()
        .map(|formula| summarize_formula(&formula))
}

/// Parse a formula and calculate molecular weight.
pub fn molecular_weight(input: &str) -> Result<f64, FormulaError> {
    parse_formula(input)?.molecular_weight()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_formula_summary() {
        let summary = parse_formula_summary("1 wt % Pt / SiO2").unwrap();

        assert_eq!(summary.formula, "Pt1wt%O2Si");
        assert_eq!(summary.elements, ["O", "Si", "Pt"]);
        assert_eq!(summary.stoichiometry["O"], 2.0);
        assert_eq!(summary.stoichiometry["Si"], 1.0);
        assert_eq!(summary.wt_percent["Pt"], 1.0);
    }

    #[test]
    fn test_to_wt_percent_summary() {
        let summary = to_wt_percent_summary("H2O").unwrap();

        assert!(summary.stoichiometry.is_empty());
        assert!(summary.wt_percent.contains_key("H"));
        assert!(summary.wt_percent.contains_key("O"));
    }
}
