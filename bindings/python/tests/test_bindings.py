import pytest

import chemical_formula_rs as cf


def test_parse_formula_supports_flexible_loading() -> None:
    summary = cf.parse_formula("1 wt % Pt / SiO2")

    assert summary["elements"] == ["O", "Si", "Pt"]
    assert summary["stoichiometry"]["O"] == 2.0
    assert summary["stoichiometry"]["Si"] == 1.0
    assert summary["wt_percent"]["Pt"] == 1.0


def test_to_wt_percent() -> None:
    summary = cf.to_wt_percent("H2O")

    assert summary["stoichiometry"] == {}
    assert summary["wt_percent"]["H"] > 0
    assert summary["wt_percent"]["O"] > 0


def test_molecular_weight() -> None:
    assert cf.molecular_weight("H2O") == pytest.approx(18.015)


def test_parse_errors_become_value_errors() -> None:
    with pytest.raises(ValueError):
        cf.parse_formula("1%Xx/SiO2")
