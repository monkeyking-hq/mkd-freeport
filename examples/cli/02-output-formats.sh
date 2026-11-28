#!/usr/bin/env bash
# SPDX-FileCopyrightText: 2026 Nathan A Chadwick
# SPDX-License-Identifier: MIT
set -euo pipefail

BIN="${MKD_FREEPORT:-mkd-freeport}"

echo "--- compact (default) ---"
"$BIN" --format compact

echo "--- pretty (--pretty / --format json) ---"
"$BIN" --pretty

echo "--- human (--human / --format human) ---"
"$BIN" --human

echo "--- plain (--plain) ---"
PORT="$("$BIN" --plain)"
echo "Bare port: $PORT"
