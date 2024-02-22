//! # Chemical Formula Parser
//!
//! This module contains the parser for chemical formulas.
//! The parser is implemented using the pest crate and the grammar is defined in the formula.pest file.
//!
//! The main function is `parse_formula` which takes a string and returns a `ChemicalFormula` struct.
//!
//! #Example:
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
use pest::Parser;
use pest_derive::Parser;

use pest::iterators::Pair;
use std::error::Error;

#[derive(Parser)]
#[grammar = "formula.pest"]
pub struct ChemicalFormulaParser {}

/// A recursive function to parse the chemical formula
fn parse_formula_pairs(pair: Pair<Rule>) -> ChemicalFormula {
    match pair.as_rule() {
        Rule::formula => pair
            .into_inner()
            .map(|x| parse_formula_pairs(x))
            .fold(&mut ChemicalFormula::new(), |acc, x| acc.add_formula(&x))
            .to_owned(),
        Rule::group => {
            let mut formula = ChemicalFormula::new();
            for p in pair.into_inner() {
                match p.as_rule() {
                    Rule::element => {
                        formula.add_formula(&parse_formula_pairs(p));
                    }
                    Rule::stoichiometry => {
                        let stoichiometry = p.into_inner().next();

                        if stoichiometry.is_none() {
                            return formula.to_owned();
                        }

                        let stoichiometry = stoichiometry.unwrap();

                        match stoichiometry.as_rule() {
                            Rule::number => {
                                formula.multiply(stoichiometry.as_str().parse().unwrap());
                            }
                            Rule::weight_percent => {
                                formula
                                    .multiply_wt_percent(
                                        stoichiometry
                                            .into_inner()
                                            .next()
                                            .unwrap()
                                            .as_str()
                                            .parse()
                                            .unwrap(),
                                    )
                                    .unwrap();
                            }
                            _ => unreachable!(),
                        }
                    }
                    _ => unreachable!(),
                }
            }

            formula.to_owned()
        }
        Rule::element => {
            let mut rule = pair.into_inner();
            let element = rule.next().unwrap();
            let stoichiometry = rule.next().unwrap().into_inner().next();

            if stoichiometry.is_none() {
                return ChemicalFormula::new()
                    .add_element(ElementSymbol::from_str(element.as_str()), 1.)
                    .to_owned();
            }

            let stoichiometry = stoichiometry.unwrap();

            match stoichiometry.as_rule() {
                Rule::number => ChemicalFormula::new()
                    .add_element(
                        ElementSymbol::from_str(element.as_str()),
                        stoichiometry.as_str().parse().unwrap_or(1.0),
                    )
                    .to_owned(),
                Rule::weight_percent => ChemicalFormula::new()
                    .add_wt_percent(
                        ElementSymbol::from_str(element.as_str()),
                        stoichiometry
                            .into_inner()
                            .next()
                            .unwrap()
                            .as_str()
                            .parse()
                            .unwrap(),
                    )
                    .to_owned(),
                _ => {
                    unreachable!()
                }
            }
        }
        Rule::EOI => ChemicalFormula::new(),
        Rule::number
        | Rule::separator
        | Rule::expr
        | Rule::weight_percent
        | Rule::stoichiometry
        | Rule::element_symbol => {
            unreachable!();
        }
    }
}

/// Parse a chemical formula from a string
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
pub fn parse_formula(s: &str) -> Result<ChemicalFormula, Box<dyn Error>> {
    let mut pairs = ChemicalFormulaParser::parse(Rule::formula, s)?;

    Ok(parse_formula_pairs(pairs.next().unwrap()))
}

#[cfg(test)]
mod tests {
    use crate::element::ElementSymbol;

    use super::*;
    use approx::{assert_abs_diff_eq, assert_abs_diff_ne};
    const TOL: f64 = 1e-10;

    #[test]
    fn test_chmical_formula_parser() {
        let formula_str = "SiO2";

        let expected_MW = 60.083;
        let expected_Si = 1.0;
        let expected_O = 2.0;

        let formula = parse_formula(formula_str).unwrap();

        assert_abs_diff_eq!(formula.stoichiometry[&ElementSymbol::O], expected_O);
        assert_abs_diff_eq!(formula.stoichiometry[&ElementSymbol::Si], expected_Si);
        assert_abs_diff_eq!(formula.molecular_weight().unwrap(), expected_MW);
    }

    #[test]
    fn test_chmical_formula_parser_wt_percent() {
        let formula_str = "Pt5wt%/SiO2";

        let expected_Si = 1.0;
        let expected_O = 2.0;
        let expected_Pt_wt = 5.0;
        let expected_SiO2_wt = 100.0 - expected_Pt_wt;

        let formula = parse_formula(formula_str).unwrap();

        assert_abs_diff_eq!(formula.stoichiometry[&ElementSymbol::O], expected_O);
        assert_abs_diff_eq!(formula.stoichiometry[&ElementSymbol::Si], expected_Si);

        let wt_percent = formula.to_wt_percent().unwrap().wt_percent;
        assert_abs_diff_eq!(wt_percent[&ElementSymbol::Pt], expected_Pt_wt);

        assert_abs_diff_eq!(
            wt_percent[&ElementSymbol::Si] + wt_percent[&ElementSymbol::O],
            expected_SiO2_wt
        );
    }

    #[test]
    fn test_chmical_formula_parser_nested() {
        let formula_str = "(Pt5wt%SiO2)50wt%(Au5wt%/SiO2)50wt%";

        let expected_Pt_wt = 5.0 / 2.;
        let expected_Au_wt = 5.0 / 2.;
        let expected_SiO2_wt = 100.0 - expected_Pt_wt - expected_Au_wt;

        let formula = parse_formula(formula_str).unwrap();

        let wt_percent = formula.to_wt_percent().unwrap().wt_percent;
        assert_abs_diff_eq!(
            wt_percent[&ElementSymbol::Pt],
            expected_Pt_wt,
            epsilon = TOL
        );

        assert_abs_diff_eq!(
            wt_percent[&ElementSymbol::Si] + wt_percent[&ElementSymbol::O],
            expected_SiO2_wt,
            epsilon = TOL
        );

        assert_abs_diff_eq!(
            wt_percent[&ElementSymbol::Au],
            expected_Au_wt,
            epsilon = TOL
        );
    }
}
