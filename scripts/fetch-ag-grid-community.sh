#!/usr/bin/env bash
set -euo pipefail

AG_GRID_VERSION="33.2.3"
TARGET_DIR="static/vendor/ag-grid-community"
TMP_DIR=$(mktemp -d)

echo "📦 Fetching AG Grid Community v${AG_GRID_VERSION}..."

echo "🧹 Cleaning old AG Grid files in ${TARGET_DIR}..."
rm -rf "${TARGET_DIR}"

echo "📂 Ensuring vendor directory exists..."
mkdir -p "$(dirname "${TARGET_DIR}")"

echo "📁 Using temp directory: ${TMP_DIR}"
cd "${TMP_DIR}"

echo "📦 Initializing npm project..."
npm init -y > /dev/null

echo "📥 Installing ag-grid-community@${AG_GRID_VERSION}..."
npm install "ag-grid-community@${AG_GRID_VERSION}" > /dev/null

echo "📂 Copying entire ag-grid-community module..."
cp -r node_modules/ag-grid-community "${OLDPWD}/${TARGET_DIR}/"

echo "🧹 Cleaning up temp directory..."
rm -rf "${TMP_DIR}"

echo "✅ AG Grid Community v${AG_GRID_VERSION} copied to ${TARGET_DIR}"
