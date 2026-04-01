#!/bin/bash

set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)"
cd "$REPO_ROOT"

cd crates/app
wasm-pack build --target web --out-dir ../../pkg --out-name astra --release --no-default-features
cd "$REPO_ROOT"

mv pkg/astra_bg.wasm pkg/astra.wasm

WASM_FILE="pkg/astra.wasm"
if [ ! -f "$WASM_FILE" ]; then
    echo "::error::WASM build output not found at $WASM_FILE"
    exit 1
fi

wasm-opt -Oz --strip-debug --strip-dwarf --strip-producers --vacuum --dce --duplicate-function-elimination --enable-bulk-memory --merge-similar-functions --converge -o "$WASM_FILE" "$WASM_FILE"

rm -f pkg/.gitignore pkg/package.json pkg/README.md
rm -rf web/pkg
cp -r pkg web/pkg

if [ -d "assets/icons" ]; then
    mkdir -p web/assets/icons
    cp assets/icons/*.png web/assets/icons/ 2>/dev/null || true
fi
