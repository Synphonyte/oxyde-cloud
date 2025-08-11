#!/bin/bash

# Get the directory path of this script file
SCRIPT_DIR="$(dirname "$(readlink -f "${BASH_SOURCE[0]}")")"

export OXYDE_CLOUD_API_URL="http://localhost:3000/api/v1/"
export OXYDE_CLOUD_API_KEY="e9625027-619d-4232-9c6b-199c76d389fd"
export OXYDE_CLOUD_BIN_DIR="target"
export RUST_BACKTRACE=1
cargo run --features with-deploy-test --manifest-path $SCRIPT_DIR/Cargo.toml $1
