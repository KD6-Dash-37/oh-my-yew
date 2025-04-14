#!/usr/bin/env bash
set -euo pipefail

AG_GRID_VERSION="33.2.3"
TARGET_DIR="static/vendor/ag-grid-enterprise"
TMP_DIR=$(mktemp -d)

echo "📦 Fetching AG Grid Enterprise v${AG_GRID_VERSION}..."

echo "🧹 Cleaning old AG Grid Enterprise files in ${TARGET_DIR}..."
rm -rf "${TARGET_DIR}"

echo "📂 Ensuring vendor directory exists..."
mkdir -p "$(dirname "${TARGET_DIR}")"

echo "📁 Using temp directory: ${TMP_DIR}"
cd "${TMP_DIR}"

echo "📦 Initializing npm project..."
npm init -y > /dev/null

echo "📥 Installing ag-grid-enterprise@${AG_GRID_VERSION}..."
npm install "ag-grid-enterprise@${AG_GRID_VERSION}" > /dev/null

echo "📂 Copying entire ag-grid-enterprise module..."
cp -r node_modules/ag-grid-enterprise "${OLDPWD}/${TARGET_DIR}/"

echo "🧹 Cleaning up temp directory..."
rm -rf "${TMP_DIR}"

echo "✅ AG Grid Enterprise v${AG_GRID_VERSION} copied to ${TARGET_DIR}"
