# @ameyanagi/chemical-formula

WASM/Node.js bindings for `chemical-formula`.

```ts
import { parseFormula, molecularWeight } from "@ameyanagi/chemical-formula";

const summary = parseFormula("1 wt % Pt / SiO2");
const mw = molecularWeight("H2O");
```

## API

- `parseFormula(input: string)`
- `toMolecularFormula(input: string)`
- `toWtPercent(input: string)`
- `molecularWeight(input: string)`
