//! # Chemical Formula Parser
//!
//! This module contains the parser for chemical formulas.
//! The parser is implemented using the pest crate and the grammar is defined in the formula.pest file.
//!
//! The main function is `parse_formula` which takes a string and returns a `ChemicalFormula` struct.
//!
//! # Example:
//! ```
//! use chemical_formula::parser::parse_formula;
//! use chemical_formula::prelude::*;
//!
//! let formula_str = "SiO2";
//! let formula = parse_formula(formula_str).unwrap();
//!
//! assert_eq!(formula.stoichiometry[&ElementSymbol::O], 2.0);
//! assert_eq!(formula.stoichiometry[&ElementSymbol::Si], 1.0);
//! ```

use crate::element::{ChemicalFormula, ElementSymbol, FormulaError};
use pest::error::InputLocation;
use pest::iterators::Pair;
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "formula.pest"]
pub struct ChemicalFormulaParser {}

enum ParsedStoichiometry {
    Number(f64),
    WeightPercent(f64),
}

fn parse_error(input: &str, position: Option<usize>, reason: impl Into<String>) -> FormulaError {
    FormulaError::ParseError {
        input: input.to_owned(),
        position,
        reason: reason.into(),
    }
}

fn parse_error_from_pair(
    input: &str,
    pair: &Pair<Rule>,
    reason: impl Into<String>,
) -> FormulaError {
    parse_error(input, Some(pair.as_span().start()), reason)
}

fn parse_number_pair(pair: Pair<Rule>) -> Result<f64, FormulaError> {
    let raw = pair.as_str();
    raw.parse::<f64>()
        .map_err(|_| FormulaError::InvalidNumber(raw.to_owned()))
}

fn parse_weight_percent_pair(pair: Pair<Rule>, input: &str) -> Result<f64, FormulaError> {
    let position = pair.as_span().start();
    let mut inner = pair.into_inner();
    let wt_pair = inner.next().ok_or_else(|| {
        parse_error(
            input,
            Some(position),
            "missing numeric value for weight percent",
        )
    })?;
    parse_number_pair(wt_pair)
}

fn parse_stoichiometry_pair(
    pair: Pair<Rule>,
    input: &str,
) -> Result<ParsedStoichiometry, FormulaError> {
    let mut inner = pair.into_inner();
    let stoichiometry = inner
        .next()
        .ok_or_else(|| parse_error(input, None, "missing stoichiometry value"))?;

    match stoichiometry.as_rule() {
        Rule::number => Ok(ParsedStoichiometry::Number(parse_number_pair(
            stoichiometry,
        )?)),
        Rule::weight_percent => Ok(ParsedStoichiometry::WeightPercent(
            parse_weight_percent_pair(stoichiometry, input)?,
        )),
        _ => Err(parse_error(
            input,
            None,
            format!(
                "unexpected stoichiometry rule: {:?}",
                stoichiometry.as_rule()
            ),
        )),
    }
}

/// A recursive function to parse the chemical formula.
fn parse_formula_pairs(pair: Pair<Rule>, input: &str) -> Result<ChemicalFormula, FormulaError> {
    match pair.as_rule() {
        Rule::formula => {
            let mut formula = ChemicalFormula::new();
            for child in pair.into_inner() {
                match child.as_rule() {
                    Rule::element | Rule::group | Rule::prefix_loading => {
                        let parsed = parse_formula_pairs(child, input)?;
                        formula.add_formula(&parsed);
                    }
                    Rule::EOI => {}
                    _ => {
                        return Err(parse_error_from_pair(
                            input,
                            &child,
                            format!("unexpected rule in formula: {:?}", child.as_rule()),
                        ));
                    }
                }
            }

            Ok(formula)
        }
        Rule::group => {
            let mut formula = ChemicalFormula::new();
            let mut factor: Option<ParsedStoichiometry> = None;

            for child in pair.into_inner() {
                match child.as_rule() {
                    Rule::element | Rule::group | Rule::prefix_loading => {
                        let parsed = parse_formula_pairs(child, input)?;
                        formula.add_formula(&parsed);
                    }
                    Rule::stoichiometry => {
                        if factor.is_some() {
                            return Err(parse_error_from_pair(
                                input,
                                &child,
                                "group has multiple stoichiometry segments",
                            ));
                        }
                        factor = Some(parse_stoichiometry_pair(child, input)?);
                    }
                    _ => {
                        return Err(parse_error_from_pair(
                            input,
                            &child,
                            format!("unexpected rule in group: {:?}", child.as_rule()),
                        ));
                    }
                }
            }

            if let Some(parsed_factor) = factor {
                match parsed_factor {
                    ParsedStoichiometry::Number(multiplier) => {
                        formula.multiply(multiplier);
                    }
                    ParsedStoichiometry::WeightPercent(multiplier) => {
                        formula.multiply_wt_percent(multiplier)?;
                    }
                }
            }

            Ok(formula)
        }
        Rule::prefix_loading => {
            let pair_position = pair.as_span().start();
            let mut inner = pair.into_inner();
            let weight_percent_pair = inner.next().ok_or_else(|| {
                parse_error(
                    input,
                    Some(pair_position),
                    "missing weight percent in prefix loading",
                )
            })?;
            let element_pair = inner.next().ok_or_else(|| {
                parse_error(
                    input,
                    Some(pair_position),
                    "missing element symbol in prefix loading",
                )
            })?;
            let symbol: ElementSymbol = element_pair.as_str().parse()?;
            let value = parse_weight_percent_pair(weight_percent_pair, input)?;

            if inner.next().is_some() {
                return Err(parse_error(
                    input,
                    Some(pair_position),
                    "prefix loading has unexpected extra tokens",
                ));
            }

            let mut formula = ChemicalFormula::new();
            formula.add_wt_percent(symbol, value);
            Ok(formula)
        }
        Rule::element => {
            let pair_position = pair.as_span().start();
            let mut inner = pair.into_inner();
            let element_pair = inner.next().ok_or_else(|| {
                parse_error(
                    input,
                    Some(pair_position),
                    "missing element symbol in element rule",
                )
            })?;
            let symbol: ElementSymbol = element_pair.as_str().parse()?;

            let mut formula = ChemicalFormula::new();

            if let Some(stoichiometry_pair) = inner.next() {
                match parse_stoichiometry_pair(stoichiometry_pair, input)? {
                    ParsedStoichiometry::Number(value) => {
                        formula.add_element(symbol, value);
                    }
                    ParsedStoichiometry::WeightPercent(value) => {
                        formula.add_wt_percent(symbol, value);
                    }
                }
            } else {
                formula.add_element(symbol, 1.0);
            }

            if inner.next().is_some() {
                return Err(parse_error(
                    input,
                    Some(pair_position),
                    "element has unexpected extra tokens",
                ));
            }

            Ok(formula)
        }
        _ => Err(parse_error_from_pair(
            input,
            &pair,
            format!("unexpected parser rule: {:?}", pair.as_rule()),
        )),
    }
}

/// Parse a chemical formula from a string.
///
/// # Example
///
/// ```
/// use chemical_formula::parser::parse_formula;
/// use chemical_formula::prelude::*;
///
/// let formula_str = "SiO2";
/// let formula = parse_formula(formula_str).unwrap();
///
/// assert_eq!(formula.stoichiometry[&ElementSymbol::O], 2.0);
/// assert_eq!(formula.stoichiometry[&ElementSymbol::Si], 1.0);
/// ```
pub fn parse_formula(s: &str) -> Result<ChemicalFormula, FormulaError> {
    if s.trim().is_empty() {
        return Err(FormulaError::NoFormula);
    }

    let mut pairs = ChemicalFormulaParser::parse(Rule::formula, s).map_err(|err| {
        let position = match err.location {
            InputLocation::Pos(pos) => Some(pos),
            InputLocation::Span((start, _)) => Some(start),
        };

        parse_error(s, position, err.to_string())
    })?;

    let root = pairs
        .next()
        .ok_or_else(|| parse_error(s, None, "missing root formula pair"))?;
    let formula = parse_formula_pairs(root, s)?;

    if formula.element.is_empty() {
        return Err(FormulaError::NoFormula);
    }

    Ok(formula)
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_abs_diff_eq;

    const TOL: f64 = 1e-10;

    fn assert_pt_sio2_loading(formula_str: &str, expected_pt_wt: f64) {
        let formula = parse_formula(formula_str).unwrap();

        assert_abs_diff_eq!(formula.stoichiometry[&ElementSymbol::O], 2.0);
        assert_abs_diff_eq!(formula.stoichiometry[&ElementSymbol::Si], 1.0);

        let wt_percent = formula.to_wt_percent().unwrap().wt_percent;
        assert_abs_diff_eq!(wt_percent[&ElementSymbol::Pt], expected_pt_wt);
        assert_abs_diff_eq!(
            wt_percent[&ElementSymbol::Si] + wt_percent[&ElementSymbol::O],
            100.0 - expected_pt_wt
        );
    }

    #[test]
    fn test_chemical_formula_parser() {
        let formula_str = "SiO2";

        let expected_mw = 60.083;
        let expected_si = 1.0;
        let expected_o = 2.0;

        let formula = parse_formula(formula_str).unwrap();

        assert_abs_diff_eq!(formula.stoichiometry[&ElementSymbol::O], expected_o);
        assert_abs_diff_eq!(formula.stoichiometry[&ElementSymbol::Si], expected_si);
        assert_abs_diff_eq!(formula.molecular_weight().unwrap(), expected_mw);
    }

    #[test]
    fn test_chemical_formula_parser_wt_percent() {
        assert_pt_sio2_loading("Pt5wt%/SiO2", 5.0);
    }

    #[test]
    fn test_parse_flexible_catalyst_loading_notation() {
        for formula_str in [
            "1%Pt/SiO2",
            "Pt1%/SiO2",
            "1wt%Pt/SiO2",
            "1 wt % Pt / SiO2",
            "Pt 1 wt % / SiO2",
            "1wt%Pt@SiO2",
        ] {
            assert_pt_sio2_loading(formula_str, 1.0);
        }
    }

    #[test]
    fn test_parse_prefix_loading_binds_to_next_element() {
        let formula = parse_formula("1%PtSiO2").unwrap();

        assert_abs_diff_eq!(formula.wt_percent[&ElementSymbol::Pt], 1.0);
        assert_abs_diff_eq!(formula.stoichiometry[&ElementSymbol::Si], 1.0);
        assert_abs_diff_eq!(formula.stoichiometry[&ElementSymbol::O], 2.0);
    }

    #[test]
    fn test_parse_prefix_loading_does_not_bind_to_compound() {
        let formula = parse_formula("10%CeO2/SiO2").unwrap();

        assert_abs_diff_eq!(formula.wt_percent[&ElementSymbol::Ce], 10.0);
        assert_abs_diff_eq!(formula.stoichiometry[&ElementSymbol::Si], 1.0);
        assert_abs_diff_eq!(formula.stoichiometry[&ElementSymbol::O], 4.0);
    }

    #[test]
    fn test_parse_group_bare_percent_loading() {
        let formula = parse_formula("(CeO2)10%/SiO2").unwrap();

        assert_abs_diff_eq!(
            formula.wt_percent[&ElementSymbol::Ce] + formula.wt_percent[&ElementSymbol::O],
            10.0,
            epsilon = TOL
        );
        assert_abs_diff_eq!(formula.stoichiometry[&ElementSymbol::Si], 1.0);
        assert_abs_diff_eq!(formula.stoichiometry[&ElementSymbol::O], 2.0);
    }

    #[test]
    fn test_parse_duplicate_and_signed_loading() {
        let formula = parse_formula("1%Pt/2%Pt/SiO2").unwrap();
        assert_abs_diff_eq!(formula.wt_percent[&ElementSymbol::Pt], 3.0);

        let formula = parse_formula("-1%Pt/SiO2").unwrap();
        assert_abs_diff_eq!(formula.wt_percent[&ElementSymbol::Pt], -1.0);
    }

    #[test]
    fn test_parse_full_loading_keeps_existing_conversion_error() {
        let formula = parse_formula("Pt100%/SiO2").unwrap();
        assert!(matches!(
            formula.to_molecular_formula(),
            Err(FormulaError::DivisionByZero)
        ));
    }

    #[test]
    fn test_parse_invalid_loading_notation() {
        assert!(parse_formula("1Pt/SiO2").is_err());
        assert!(parse_formula("1%/SiO2").is_err());
        assert!(parse_formula("Pt 1 / SiO2").is_err());
        assert!(matches!(
            parse_formula("1%Xx/SiO2"),
            Err(FormulaError::InvalidElementSymbol(symbol)) if symbol == "Xx"
        ));
    }

    #[test]
    fn test_chemical_formula_parser_nested() {
        let formula_str = "(Pt5wt%SiO2)50wt%(Au5wt%/SiO2)50wt%";

        let expected_pt_wt = 5.0 / 2.0;
        let expected_au_wt = 5.0 / 2.0;
        let expected_sio2_wt = 100.0 - expected_pt_wt - expected_au_wt;

        let formula = parse_formula(formula_str).unwrap();

        let wt_percent = formula.to_wt_percent().unwrap().wt_percent;
        assert_abs_diff_eq!(
            wt_percent[&ElementSymbol::Pt],
            expected_pt_wt,
            epsilon = TOL
        );

        assert_abs_diff_eq!(
            wt_percent[&ElementSymbol::Si] + wt_percent[&ElementSymbol::O],
            expected_sio2_wt,
            epsilon = TOL
        );

        assert_abs_diff_eq!(
            wt_percent[&ElementSymbol::Au],
            expected_au_wt,
            epsilon = TOL
        );
    }

    #[test]
    fn test_parse_empty_string() {
        let result = parse_formula("   \n\t");
        assert!(matches!(result, Err(FormulaError::NoFormula)));
    }

    #[test]
    fn test_parse_single_element() {
        let formula = parse_formula("H").unwrap();
        assert_abs_diff_eq!(formula.stoichiometry[&ElementSymbol::H], 1.0);
    }

    #[test]
    fn test_parse_with_decimal_stoichiometry() {
        let formula = parse_formula("H2.5O").unwrap();
        assert_abs_diff_eq!(formula.stoichiometry[&ElementSymbol::H], 2.5);
        assert_abs_diff_eq!(formula.stoichiometry[&ElementSymbol::O], 1.0);
    }

    #[test]
    fn test_parse_nested_groups() {
        let formula = parse_formula("Ca(OH)2").unwrap();
        assert_abs_diff_eq!(formula.stoichiometry[&ElementSymbol::Ca], 1.0);
        assert_abs_diff_eq!(formula.stoichiometry[&ElementSymbol::O], 2.0);
        assert_abs_diff_eq!(formula.stoichiometry[&ElementSymbol::H], 2.0);
    }

    #[test]
    fn test_parse_multiple_groups() {
        let formula = parse_formula("Mg3(PO4)2").unwrap();
        assert_abs_diff_eq!(formula.stoichiometry[&ElementSymbol::Mg], 3.0);
        assert_abs_diff_eq!(formula.stoichiometry[&ElementSymbol::P], 2.0);
        assert_abs_diff_eq!(formula.stoichiometry[&ElementSymbol::O], 8.0);
    }

    #[test]
    fn test_parse_invalid_element() {
        let result = parse_formula("Xx2");
        assert!(matches!(
            result,
            Err(FormulaError::InvalidElementSymbol(symbol)) if symbol == "Xx"
        ));
    }

    #[test]
    fn test_parse_wt_overflow() {
        let formula = parse_formula("H60wt%O60wt%").unwrap();
        assert!(matches!(
            formula.to_molecular_formula(),
            Err(FormulaError::WeightPercentOverflow)
        ));
    }

    #[test]
    fn test_parse_complex_formula() {
        let formula = parse_formula("(NH4)2SO4").unwrap();
        assert_abs_diff_eq!(formula.stoichiometry[&ElementSymbol::N], 2.0);
        assert_abs_diff_eq!(formula.stoichiometry[&ElementSymbol::H], 8.0);
        assert_abs_diff_eq!(formula.stoichiometry[&ElementSymbol::S], 1.0);
        assert_abs_diff_eq!(formula.stoichiometry[&ElementSymbol::O], 4.0);
    }
}
