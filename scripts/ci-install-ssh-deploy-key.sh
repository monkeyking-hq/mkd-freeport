#!/usr/bin/env bash
# SPDX-FileCopyrightText: 2026 Nathan A Chadwick
# SPDX-License-Identifier: LicenseRef-FFM-1.0
#
# Install org deploy key for private monkeykingdev-op/* git deps in CI.
# Requires env MKD_GITHUB_SSH_PRIVATE_KEY (from GitHub Actions secret).
#
# Run order (required): scripts/ci-ssh-github-alias.sh first, then this script.
set -euo pipefail

HOST_GITHUB_COM_MKD_RE='^[[:space:]]*Host[[:space:]]+github\.com-mkd[[:space:]]*$'

if [ -z "${MKD_GITHUB_SSH_PRIVATE_KEY:-}" ]; then
  REPO="${GITHUB_REPOSITORY:-monkeykingdev-op/mkd-secrets}"
  echo "::error::MKD_GITHUB_SSH_PRIVATE_KEY is empty — add repo secret with that exact name on ${REPO} (Settings → Secrets → Actions). Fork PRs do not receive secrets."
  exit 1
fi

mkdir -p "${HOME}/.ssh"
chmod 700 "${HOME}/.ssh"

KEY_FILE="${HOME}/.ssh/mkd_ci_deploy"
umask 077
printf '%s\n' "${MKD_GITHUB_SSH_PRIVATE_KEY}" > "${KEY_FILE}"
chmod 600 "${KEY_FILE}"

CONFIG="${HOME}/.ssh/config"
if ! grep -qE "${HOST_GITHUB_COM_MKD_RE}" "${CONFIG}" 2>/dev/null; then
  echo "::error::Run scripts/ci-ssh-github-alias.sh before scripts/ci-install-ssh-deploy-key.sh"
  exit 1
fi

if ! grep -q 'IdentityFile.*mkd_ci_deploy' "${CONFIG}" 2>/dev/null; then
  sed -i -E "/${HOST_GITHUB_COM_MKD_RE}/a\\
  IdentityFile ~/.ssh/mkd_ci_deploy\\
  IdentitiesOnly yes" "${CONFIG}" 2>/dev/null || {
    {
      echo '  IdentityFile ~/.ssh/mkd_ci_deploy'
      echo '  IdentitiesOnly yes'
    } >> "${CONFIG}"
  }
fi

echo "Deploy key installed for Host github.com-mkd"
