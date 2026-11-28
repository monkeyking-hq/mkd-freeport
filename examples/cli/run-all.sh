#!/usr/bin/env bash
# SPDX-FileCopyrightText: 2026 Nathan A Chadwick
# SPDX-License-Identifier: MIT
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"

if [ -z "${MKD_FREEPORT:-}" ]; then
  if [ -x "${ROOT}/target/release/mkd-freeport" ]; then
    export MKD_FREEPORT="${ROOT}/target/release/mkd-freeport"
  else
    export MKD_FREEPORT="mkd-freeport"
  fi
fi

for script in \
  "${ROOT}/examples/cli/01-basic-json.sh" \
  "${ROOT}/examples/cli/02-output-formats.sh" \
  "${ROOT}/examples/cli/03-ranges-and-first.sh" \
  "${ROOT}/examples/cli/04-protocols-and-bind.sh" \
  "${ROOT}/examples/cli/05-foreground-reserve.sh"
do
  echo "=== Running $(basename "$script") ==="
  bash "$script"
done

echo "All CLI examples passed."
