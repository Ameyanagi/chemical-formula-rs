# chemical-formula-rs

Python bindings for `chemical-formula`.

```python
import chemical_formula_rs as cf

summary = cf.parse_formula("1 wt % Pt / SiO2")
mw = cf.molecular_weight("H2O")
```

## API

- `parse_formula(input: str) -> FormulaSummary`
- `to_molecular_formula(input: str) -> FormulaSummary`
- `to_wt_percent(input: str) -> FormulaSummary`
- `molecular_weight(input: str) -> float`
