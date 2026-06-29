#!/usr/bin/env python3
"""Remove [patch.*] sections from Cargo.toml for CI (git deps from Cargo.lock)."""
from pathlib import Path
import sys


def main() -> None:
    root = Path.cwd()
    path = root / "Cargo.toml"
    if not path.is_file():
        print(
            f"::error::Cargo.toml not found in {root} — run this script from the repo root "
            "(check workflow working-directory).",
            file=sys.stderr,
        )
        sys.exit(1)
    try:
        lines = path.read_text(encoding="utf-8").splitlines()
    except OSError as e:
        print(f"::error::Cannot read {path}: {e}", file=sys.stderr)
        sys.exit(1)

    out: list[str] = []
    in_patch = False
    for line in lines:
        if line.strip().startswith("[patch."):
            in_patch = True
            continue
        if in_patch and line.startswith("[") and not line.strip().startswith("[patch."):
            in_patch = False
        if in_patch:
            continue
        out.append(line)
    path.write_text("\n".join(out) + "\n", encoding="utf-8")


if __name__ == "__main__":
    main()