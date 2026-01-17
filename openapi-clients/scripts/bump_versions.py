#!/usr/bin/env python3
import json
import re
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]

TS_PACKAGE_JSON = ROOT / "typescript" / "package" / "package.json"
PYPROJECT_TOML = ROOT / "python" / "package" / "pyproject.toml"
RUST_CARGO_TOML = ROOT / "rust" / "package" / "Cargo.toml"


def bump_patch(version: str) -> str:
    match = re.match(r"^(\d+)\.(\d+)\.(\d+)$", version.strip())
    if not match:
        raise ValueError(f"Unsupported version format: {version}")
    major, minor, patch = map(int, match.groups())
    return f"{major}.{minor}.{patch + 1}"


def bump_package_json(path: Path) -> str:
    data = json.loads(path.read_text())
    current = data.get("version")
    if not current:
        raise ValueError(f"Missing version in {path}")
    data["version"] = bump_patch(current)
    path.write_text(json.dumps(data, indent=2) + "\n")
    return data["version"]


def bump_toml_version(path: Path) -> str:
    content = path.read_text()
    pattern = re.compile(r'^(version\s*=\s*")(\d+)\.(\d+)\.(\d+)(")', re.MULTILINE)
    match = pattern.search(content)
    if not match:
        raise ValueError(f"Missing version in {path}")
    current = ".".join(match.group(i) for i in range(2, 5))
    bumped = bump_patch(current)
    updated = pattern.sub(rf'\1{bumped}\5', content, count=1)
    path.write_text(updated)
    return bumped


def main() -> None:
    ts_version = bump_package_json(TS_PACKAGE_JSON)
    py_version = bump_toml_version(PYPROJECT_TOML)
    rust_version = bump_toml_version(RUST_CARGO_TOML)
    print(f"TypeScript version -> {ts_version}")
    print(f"Python version     -> {py_version}")
    print(f"Rust version       -> {rust_version}")


if __name__ == "__main__":
    main()
