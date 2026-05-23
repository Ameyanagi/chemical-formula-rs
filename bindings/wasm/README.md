# @ameyanagi/chemical-formula

WASM/Node.js bindings for the Rust `chemical-formula` parser.

Use this package to parse chemical formulas, convert between molecular and
weight-percent representations, and calculate molecular weights. It supports
ordinary formulas such as `H2O`, weight-percent annotations such as
`Pt5wt%/SiO2`, and flexible catalyst loading notation such as `1%Pt/SiO2`,
`Pt1%/SiO2`, and `1wt%Pt@SiO2`.

## Installation

```sh
npm install @ameyanagi/chemical-formula
```

With Bun:

```sh
bun add @ameyanagi/chemical-formula
```

## Quick Start

```ts
import {
  molecularWeight,
  parseFormula,
  toMolecularFormula,
  toWtPercent,
} from "@ameyanagi/chemical-formula";

const catalyst = parseFormula("1%Pt/SiO2");

console.log(catalyst);
// {
//   formula: "Pt1wt%O2Si",
//   elements: ["O", "Si", "Pt"],
//   stoichiometry: { O: 2, Si: 1 },
//   wt_percent: { Pt: 1 }
// }

const molecular = toMolecularFormula("1%Pt/SiO2");
console.log(molecular.stoichiometry.Pt);
// 0.0031109624054201776

const water = toWtPercent("H2O");
console.log(water.wt_percent);
// { H: 11.19067443796836, O: 88.80932556203165 }

console.log(molecularWeight("H2O"));
// 18.015
```

## API

```ts
type FormulaSummary = {
  formula: string;
  elements: string[];
  stoichiometry: Record<string, number>;
  wt_percent: Record<string, number>;
};
```

- `parseFormula(input: string): FormulaSummary`
- `toMolecularFormula(input: string): FormulaSummary`
- `toWtPercent(input: string): FormulaSummary`
- `molecularWeight(input: string): number`

All parsing and conversion errors are thrown as JavaScript errors.

## Supported Notation Examples

These examples all describe 1 wt% Pt on silica:

```ts
parseFormula("1%Pt/SiO2");
parseFormula("Pt1%/SiO2");
parseFormula("1wt%Pt/SiO2");
parseFormula("Pt1wt%/SiO2");
parseFormula("1wt%Pt@SiO2");
parseFormula("1 wt % Pt / SiO2");
```

Bare `%` is interpreted as `wt%`.

Nested materials are supported:

```ts
const composite = toWtPercent("(Pt5wt%/SiO2)50wt%(CeO2)50wt%");

console.log(composite.wt_percent.Pt);
// 2.5000000000000004
```

## Package Notes

This package is generated from the Rust crate with `wasm-pack` and targets
Node.js-compatible runtimes.
