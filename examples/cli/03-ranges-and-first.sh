#!/usr/bin/env bash
# SPDX-FileCopyrightText: 2026 Nathan A Chadwick
# SPDX-License-Identifier: MIT
set -euo pipefail

BIN="${MKD_FREEPORT:-mkd-freeport}"

echo "--- --first (searches 1025-65535 in ascending order) ---"
"$BIN" --first --human

echo "--- --bind 127.0.0.1 --range 49152-50000 --first --count 2 ---"
"$BIN" --bind 127.0.0.1 --range 49152-50000 --first --count 2 --pretty
