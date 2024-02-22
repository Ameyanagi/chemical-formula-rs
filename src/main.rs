use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::{default, error::Error};

// use lazy_static;
use pest::{pratt_parser::PrattParser, Parser};
use pest_derive::Parser;

mod element;
mod parser;
use parser::parse_formula;

// pub fn parse_formula(s: &str) -> Result<Formula, Box<dyn Error>> {
//     let pairs = ChemicalFormula::parse(Rule::formula, s)?;
//
//     // let formula = pairs
//     //     .map(|pair| match pair.as_rule() {
//     //         Rule::formula => {
//     //             let mut formula = Formula::default();
//     //             for inner_pair in pair.into_inner() {
//     //                 match inner_pair.as_rule() {
//     //                     Rule::element => {
//     //                         formula
//     //                             .elements
//     //                             .push(element::ElementSymbol::from_str(inner_pair.as_str()));
//     //                     }
//     //                     Rule::number => {
//     //                         formula
//     //                             .mol_ratio
//     //                             .push(inner_pair.as_str().parse::<f64>().unwrap());
//     //                     }
//     //                     _ => {}
//     //                 }
//     //             }
//     //             Ok(formula)
//     //         }
//     //         _ => Err(FormulaError::FileParseError.into()),
//     //     })
//     //     .next()
//     //     .unwrap_or_else(|| Err(FormulaError::FileParseError.into()));
//     //
//     // Ok(Formula::default())
//     //
//     formula
// }

fn main() {
    let input = fs::read_to_string("src/input.txt").unwrap();

    println!("{:?}", parse_formula(&input).unwrap());
}
