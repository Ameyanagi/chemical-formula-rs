# chemical-formula-rs

Python bindings for the Rust `chemical-formula` parser.

Use this package to parse chemical formulas, convert between molecular and
weight-percent representations, and calculate molecular weights. It supports
ordinary formulas such as `H2O`, weight-percent annotations such as
`Pt5wt%/SiO2`, and flexible catalyst loading notation such as `1%Pt/SiO2`,
`Pt1%/SiO2`, and `1wt%Pt@SiO2`.

## Installation

```sh
pip install chemical-formula-rs
```

With uv:

```sh
uv add chemical-formula-rs
```

## Quick Start

```python
import chemical_formula_rs as cf

catalyst = cf.parse_formula("1%Pt/SiO2")

print(catalyst)
# {
#     "formula": "Pt1wt%O2Si",
#     "elements": ["O", "Si", "Pt"],
#     "stoichiometry": {"O": 2.0, "Si": 1.0},
#     "wt_percent": {"Pt": 1.0},
# }

molecular = cf.to_molecular_formula("1%Pt/SiO2")
print(molecular["stoichiometry"]["Pt"])
# 0.0031109624054201776

water = cf.to_wt_percent("H2O")
print(water["wt_percent"])
# {"H": 11.19067443796836, "O": 88.80932556203165}

print(cf.molecular_weight("H2O"))
# 18.015
```

## API

`FormulaSummary` is returned as a dictionary:

```python
{
    "formula": str,
    "elements": list[str],
    "stoichiometry": dict[str, float],
    "wt_percent": dict[str, float],
}
```

- `parse_formula(input: str) -> FormulaSummary`
- `to_molecular_formula(input: str) -> FormulaSummary`
- `to_wt_percent(input: str) -> FormulaSummary`
- `molecular_weight(input: str) -> float`

Parsing and conversion errors raise `ValueError`.

## Supported Notation Examples

These examples all describe 1 wt% Pt on silica:

```python
cf.parse_formula("1%Pt/SiO2")
cf.parse_formula("Pt1%/SiO2")
cf.parse_formula("1wt%Pt/SiO2")
cf.parse_formula("Pt1wt%/SiO2")
cf.parse_formula("1wt%Pt@SiO2")
cf.parse_formula("1 wt % Pt / SiO2")
```

Bare `%` is interpreted as `wt%`.

Nested materials are supported:

```python
composite = cf.to_wt_percent("(Pt5wt%/SiO2)50wt%(CeO2)50wt%")

print(composite["wt_percent"]["Pt"])
# 2.5000000000000004
```

## Package Notes

The Python extension is built with PyO3 and ships the shared Rust parser as a
native module imported through `chemical_formula_rs`.
