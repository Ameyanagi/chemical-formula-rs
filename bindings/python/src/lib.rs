#![allow(clippy::useless_conversion)]

use chemical_formula::prelude::{
    molecular_weight as core_molecular_weight, parse_formula_summary, to_molecular_formula_summary,
    to_wt_percent_summary, FormulaError, FormulaSummary,
};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList};

fn py_error(error: FormulaError) -> PyErr {
    PyValueError::new_err(error.to_string())
}

fn summary_to_dict<'py>(py: Python<'py>, summary: FormulaSummary) -> PyResult<Bound<'py, PyDict>> {
    let dict = PyDict::new(py);
    dict.set_item("formula", summary.formula)?;
    dict.set_item("elements", PyList::new(py, summary.elements)?)?;

    let stoichiometry = PyDict::new(py);
    for (element, value) in summary.stoichiometry {
        stoichiometry.set_item(element, value)?;
    }
    dict.set_item("stoichiometry", stoichiometry)?;

    let wt_percent = PyDict::new(py);
    for (element, value) in summary.wt_percent {
        wt_percent.set_item(element, value)?;
    }
    dict.set_item("wt_percent", wt_percent)?;

    Ok(dict)
}

fn result_to_dict<'py>(
    py: Python<'py>,
    result: Result<FormulaSummary, FormulaError>,
) -> PyResult<Bound<'py, PyDict>> {
    summary_to_dict(py, result.map_err(py_error)?)
}

#[pyfunction]
fn parse_formula<'py>(py: Python<'py>, input: &str) -> PyResult<Bound<'py, PyDict>> {
    result_to_dict(py, parse_formula_summary(input))
}

#[pyfunction]
fn to_molecular_formula<'py>(py: Python<'py>, input: &str) -> PyResult<Bound<'py, PyDict>> {
    result_to_dict(py, to_molecular_formula_summary(input))
}

#[pyfunction]
fn to_wt_percent<'py>(py: Python<'py>, input: &str) -> PyResult<Bound<'py, PyDict>> {
    result_to_dict(py, to_wt_percent_summary(input))
}

#[pyfunction]
fn molecular_weight(input: &str) -> PyResult<f64> {
    core_molecular_weight(input).map_err(py_error)
}

#[pymodule]
fn _native(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(parse_formula, m)?)?;
    m.add_function(wrap_pyfunction!(to_molecular_formula, m)?)?;
    m.add_function(wrap_pyfunction!(to_wt_percent, m)?)?;
    m.add_function(wrap_pyfunction!(molecular_weight, m)?)?;
    Ok(())
}
