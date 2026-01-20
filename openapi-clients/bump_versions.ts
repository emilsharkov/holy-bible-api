import fs from "fs";
import path from "path";
import toml, { JsonMap } from "@iarna/toml";

const ROOT = path.resolve(__dirname, "..");
const TS_PACKAGE_JSON = path.join(ROOT, "typescript", "package", "package.json");
const PYPROJECT_TOML = path.join(ROOT, "python", "package", "pyproject.toml");
const RUST_CARGO_TOML = path.join(ROOT, "rust", "package", "Cargo.toml");

type VersionTuple = [number, number, number];

function versionTuple(version: string): VersionTuple {
  const match = /^(\d+)\.(\d+)\.(\d+)$/.exec(version.trim());
  if (!match) {
    throw new Error(`Unsupported version format: ${version}`);
  }
  return [Number(match[1]), Number(match[2]), Number(match[3])];
}

function compareVersions(a: string, b: string): number {
  const aParts = versionTuple(a);
  const bParts = versionTuple(b);
  for (let i = 0; i < aParts.length; i += 1) {
    if (aParts[i] !== bParts[i]) {
      return aParts[i] - bParts[i];
    }
  }
  return 0;
}

function bumpPatch(version: string): string {
  const [major, minor, patch] = versionTuple(version);
  return `${major}.${minor}.${patch + 1}`;
}

function readPackageJsonVersion(filePath: string): string {
  const data = JSON.parse(fs.readFileSync(filePath, "utf8")) as { version?: string };
  if (!data.version) {
    throw new Error(`Missing version in ${filePath}`);
  }
  return data.version;
}

function writePackageJsonVersion(filePath: string, version: string): void {
  const data = JSON.parse(fs.readFileSync(filePath, "utf8")) as { version?: string };
  data.version = version;
  fs.writeFileSync(filePath, `${JSON.stringify(data, null, 2)}\n`, "utf8");
}

function readTomlVersion(filePath: string, table: string): string {
  const content = fs.readFileSync(filePath, "utf8");
  const data = toml.parse(content) as JsonMap;
  const tableData = data[table] as JsonMap | undefined;
  const version = tableData?.version;
  if (typeof version !== "string") {
    throw new Error(`Missing version in ${filePath}`);
  }
  return version;
}

function writeTomlVersion(filePath: string, table: string, version: string): void {
  const content = fs.readFileSync(filePath, "utf8");
  const data = toml.parse(content) as JsonMap;
  const tableData = (data[table] ?? {}) as JsonMap;
  tableData.version = version;
  data[table] = tableData;
  const updated = toml.stringify(data);
  fs.writeFileSync(filePath, updated.endsWith("\n") ? updated : `${updated}\n`, "utf8");
}

function main(): void {
  const tsCurrent = readPackageJsonVersion(TS_PACKAGE_JSON);
  const pyCurrent = readTomlVersion(PYPROJECT_TOML, "project");
  const rustCurrent = readTomlVersion(RUST_CARGO_TOML, "package");

  const versions = [tsCurrent, pyCurrent, rustCurrent];
  const maxCurrent = versions.reduce((max, current) =>
    compareVersions(current, max) > 0 ? current : max
  );
  const bumped = bumpPatch(maxCurrent);

  writePackageJsonVersion(TS_PACKAGE_JSON, bumped);
  writeTomlVersion(PYPROJECT_TOML, "project", bumped);
  writeTomlVersion(RUST_CARGO_TOML, "package", bumped);

  console.log(`TypeScript version -> ${bumped}`);
  console.log(`Python version     -> ${bumped}`);
  console.log(`Rust version       -> ${bumped}`);
}

main();
