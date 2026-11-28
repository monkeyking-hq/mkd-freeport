#!/usr/bin/env bash
# SPDX-FileCopyrightText: 2026 Nathan A Chadwick
# SPDX-License-Identifier: MIT
set -euo pipefail

BIN="${MKD_FREEPORT:-mkd-freeport}"

echo "--- UDP over IPv4 loopback ---"
"$BIN" --protocol udp --bind 127.0.0.1 --count 2 --pretty

echo "--- IPv6 loopback (skipped if IPv6 is unavailable on host) ---"
if "$BIN" --bind ::1 --plain >/dev/null 2>&1; then
  "$BIN" --bind ::1 --human
  "$BIN" --ipv6 --protocol udp --human
else
  echo "IPv6 unavailable in this environment; skipping IPv6 demo."
fi
