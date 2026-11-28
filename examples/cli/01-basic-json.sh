#!/usr/bin/env bash
# SPDX-FileCopyrightText: 2026 Nathan A Chadwick
# SPDX-License-Identifier: MIT
set -euo pipefail

BIN="${MKD_FREEPORT:-mkd-freeport}"

# Default output is a compact single-line JSON envelope.
RESULT="$("$BIN")"
echo "Compact JSON: $RESULT"

# Extract the allocated port with jq (or python3 fallback).
if command -v jq >/dev/null 2>&1; then
  PORT="$(printf '%s\n' "$RESULT" | jq -r '.data.port')"
else
  PORT="$(printf '%s\n' "$RESULT" | python3 -c 'import json,sys; print(json.load(sys.stdin)["data"]["port"])')"
fi

echo "Extracted port: $PORT"
