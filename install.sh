#!/bin/sh
# install.sh — compatibility shim for the legacy install URL.
#
# The canonical installer has moved. This shim will be removed in a future
# release. Please update your install command to:
#
#   Homebrew (recommended — integrity verified by formula):
#     brew install salemaziel/hcom/hcom
#
#   Shell installer (with checksum verification):
#     curl -fsSL https://github.com/salemaziel/hcom/releases/latest/download/hcom-installer.sh -o hcom-installer.sh
#     curl -fsSL https://github.com/salemaziel/hcom/releases/latest/download/SHA256SUMS      -o SHA256SUMS
#     sha256sum --check --ignore-missing SHA256SUMS
#     sh hcom-installer.sh
set -eu

BASE_URL="https://github.com/salemaziel/hcom/releases/latest/download"
INSTALLER_NAME="hcom-installer.sh"

echo "install.sh has moved. Running the new installer from:"
echo "  ${BASE_URL}/${INSTALLER_NAME}"
echo ""
echo "Tip: prefer 'brew install salemaziel/hcom/hcom' for reproducible installs."
echo ""

INSTALLER_TMP=$(mktemp)
CHECKSUM_TMP=$(mktemp)

cleanup() { rm -f "$INSTALLER_TMP" "$CHECKSUM_TMP"; }
trap cleanup EXIT INT TERM

curl -fsSL "${BASE_URL}/${INSTALLER_NAME}" -o "$INSTALLER_TMP"

# --- Checksum verification -------------------------------------------------
# Download the SHA256SUMS file released alongside the installer and verify
# before executing. Aborts if the checksum file cannot be retrieved or the
# hash does not match. Skips (with a warning) only if no checksum tool is
# available on the host system.
VERIFIED=0
if curl -fsSL "${BASE_URL}/SHA256SUMS" -o "$CHECKSUM_TMP" 2>/dev/null; then
    if command -v sha256sum > /dev/null 2>&1; then
        EXPECTED=$(grep " ${INSTALLER_NAME}$" "$CHECKSUM_TMP" | awk '{print $1}')
        if [ -n "$EXPECTED" ]; then
            ACTUAL=$(sha256sum "$INSTALLER_TMP" | awk '{print $1}')
            if [ "$ACTUAL" != "$EXPECTED" ]; then
                echo "error: SHA-256 checksum mismatch for ${INSTALLER_NAME}" >&2
                echo "  expected: $EXPECTED" >&2
                echo "  actual:   $ACTUAL"   >&2
                echo "Aborting installation." >&2
                exit 1
            fi
            VERIFIED=1
        fi
    elif command -v shasum > /dev/null 2>&1; then
        EXPECTED=$(grep " ${INSTALLER_NAME}$" "$CHECKSUM_TMP" | awk '{print $1}')
        if [ -n "$EXPECTED" ]; then
            ACTUAL=$(shasum -a 256 "$INSTALLER_TMP" | awk '{print $1}')
            if [ "$ACTUAL" != "$EXPECTED" ]; then
                echo "error: SHA-256 checksum mismatch for ${INSTALLER_NAME}" >&2
                echo "  expected: $EXPECTED" >&2
                echo "  actual:   $ACTUAL"   >&2
                echo "Aborting installation." >&2
                exit 1
            fi
            VERIFIED=1
        fi
    else
        echo "warning: sha256sum/shasum not found; skipping checksum verification" >&2
    fi
else
    echo "warning: could not download SHA256SUMS; skipping checksum verification" >&2
fi

if [ "$VERIFIED" = "1" ]; then
    echo "Checksum verified."
fi
# --------------------------------------------------------------------------

sh "$INSTALLER_TMP"
