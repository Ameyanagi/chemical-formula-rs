use chemical_formula::prelude::{
    molecular_weight, parse_formula_summary, to_molecular_formula_summary, to_wt_percent_summary,
    FormulaError, FormulaSummary,
};
use serde::Serialize;
use serde_wasm_bindgen::Serializer;
use wasm_bindgen::prelude::*;

fn js_error(error: FormulaError) -> JsValue {
    JsValue::from_str(&error.to_string())
}

fn summary_to_js(result: Result<FormulaSummary, FormulaError>) -> Result<JsValue, JsValue> {
    let serializer = Serializer::new().serialize_maps_as_objects(true);
    result
        .map_err(js_error)
        .and_then(|summary| summary.serialize(&serializer).map_err(|err| err.into()))
}

#[wasm_bindgen(js_name = parseFormula)]
pub fn parse_formula(input: &str) -> Result<JsValue, JsValue> {
    summary_to_js(parse_formula_summary(input))
}

#[wasm_bindgen(js_name = toMolecularFormula)]
pub fn to_molecular_formula(input: &str) -> Result<JsValue, JsValue> {
    summary_to_js(to_molecular_formula_summary(input))
}

#[wasm_bindgen(js_name = toWtPercent)]
pub fn to_wt_percent(input: &str) -> Result<JsValue, JsValue> {
    summary_to_js(to_wt_percent_summary(input))
}

#[wasm_bindgen(js_name = molecularWeight)]
pub fn molecular_weight_js(input: &str) -> Result<f64, JsValue> {
    molecular_weight(input).map_err(js_error)
}
