import { copyFile, readFile, writeFile } from "node:fs/promises";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const scriptDir = dirname(fileURLToPath(import.meta.url));
const packageDir = join(scriptDir, "..", "pkg");
const packageJsonPath = join(packageDir, "package.json");
const rootDir = join(scriptDir, "..", "..", "..");

const cargoToml = await readFile(join(rootDir, "Cargo.toml"), "utf8");
const version = cargoToml.match(/^version = "([^"]+)"/m)?.[1];

if (!version) {
  throw new Error("Could not read package version from root Cargo.toml");
}

const packageJson = JSON.parse(await readFile(packageJsonPath, "utf8"));
packageJson.name = "@ameyanagi/chemical-formula";
packageJson.version = version;
packageJson.description =
  "Chemical formula parser with catalyst loading notation support";
packageJson.license = "MIT OR Apache-2.0";
packageJson.repository = {
  type: "git",
  url: "git+https://github.com/Ameyanagi/chemical-formula-rs.git",
  directory: "bindings/wasm"
};
packageJson.sideEffects = false;
packageJson.files = [
  "README.md",
  "chemical_formula_wasm.d.ts",
  "chemical_formula_wasm.js",
  "chemical_formula_wasm_bg.wasm",
  "package.json"
];
packageJson.exports = {
  ".": {
    types: "./chemical_formula_wasm.d.ts",
    default: "./chemical_formula_wasm.js"
  }
};

await writeFile(packageJsonPath, `${JSON.stringify(packageJson, null, 2)}\n`);
await copyFile(join(scriptDir, "..", "README.md"), join(packageDir, "README.md"));
