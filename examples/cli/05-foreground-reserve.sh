#!/usr/bin/env bash
# SPDX-FileCopyrightText: 2026 Nathan A Chadwick
# SPDX-License-Identifier: MIT
set -euo pipefail

BIN="${MKD_FREEPORT:-mkd-freeport}"
TMP_DIR="$(mktemp -d)"
trap 'rm -rf "$TMP_DIR"' EXIT

PIPE="${TMP_DIR}/stdout.fifo"
mkfifo "$PIPE"

# Run reservation in the background and read flushed output from the FIFO.
# Note: command substitution `PORT=$(...)` would block until the reservation exits!
"$BIN" --reserve --bind 127.0.0.1 --count 2 --timeout 2 >"$PIPE" &
PID=$!

IFS= read -r LINE <"$PIPE"
echo "Reserved immediately: $LINE"

wait "$PID"
echo "Reservation released cleanly after timeout."
