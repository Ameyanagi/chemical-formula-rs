import { expect, test } from "bun:test";
import { createRequire } from "node:module";

const require = createRequire(import.meta.url);
const bindings = require("../pkg/chemical_formula_wasm.js");

test("parses flexible catalyst loading notation", () => {
  const summary = bindings.parseFormula("1 wt % Pt / SiO2");

  expect(summary.elements).toEqual(["O", "Si", "Pt"]);
  expect(summary.stoichiometry.O).toBe(2);
  expect(summary.stoichiometry.Si).toBe(1);
  expect(summary.wt_percent.Pt).toBe(1);
});

test("converts to normalized wt percent", () => {
  const summary = bindings.toWtPercent("H2O");

  expect(summary.stoichiometry).toEqual({});
  expect(summary.wt_percent.H).toBeGreaterThan(0);
  expect(summary.wt_percent.O).toBeGreaterThan(0);
});

test("calculates molecular weight", () => {
  expect(bindings.molecularWeight("H2O")).toBeCloseTo(18.015, 6);
});
