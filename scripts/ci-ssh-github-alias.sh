#!/usr/bin/env bash
# SPDX-FileCopyrightText: 2026 Nathan A Chadwick
# SPDX-License-Identifier: LicenseRef-FFM-1.0
#
# GitHub Actions: map Host github.com-mkd → github.com (mkd#82).
# Run before cargo/git fetch when Cargo.lock uses ssh://git@github.com-mkd/...
#
# CI order: run before scripts/ci-install-ssh-deploy-key.sh (same Host line pattern in both).
set -euo pipefail

# MKD machine/deploy user for GitHub SSH (override via workflow env if needed)
MKD_SSH_USER="${MKD_GITHUB_SSH_USER:-monkeykingdev-ops}"
# SSH config User: letters, digits, hyphen, underscore only
if ! [[ "${MKD_SSH_USER}" =~ ^[A-Za-z0-9._-]+$ ]]; then
  echo "::error::Invalid MKD_GITHUB_SSH_USER (${MKD_SSH_USER@Q}) — use only [A-Za-z0-9._-]"
  exit 1
fi

mkdir -p "${HOME}/.ssh"
chmod 700 "${HOME}/.ssh"

CONFIG="${HOME}/.ssh/config"
touch "${CONFIG}"
chmod 600 "${CONFIG}"

# Replace any prior github.com-mkd block so User/HostName stay correct
TMP="$(mktemp)"
awk '
  /^[[:space:]]*Host[[:space:]]+github\.com-mkd[[:space:]]*$/ { skip=1; next }
  skip && /^[[:space:]]*Host[[:space:]]+/ { skip=0 }
  skip { next }
  { print }
' "${CONFIG}" > "${TMP}"
mv "${TMP}" "${CONFIG}"

{
  echo 'Host github.com-mkd'
  echo '  HostName github.com'
  printf '  User %s\n' "${MKD_SSH_USER}"
  echo '  StrictHostKeyChecking accept-new'
} >> "${CONFIG}"

ssh-keyscan -t ed25519,rsa github.com >> "${HOME}/.ssh/known_hosts" 2>/dev/null || true
chmod 644 "${HOME}/.ssh/known_hosts" 2>/dev/null || true

echo "SSH alias github.com-mkd → github.com (User ${MKD_SSH_USER}) configured"
