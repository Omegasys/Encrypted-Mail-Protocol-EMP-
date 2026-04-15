#!/bin/bash

echo "=== Running EMP Node ==="

cd "$(dirname "$0")/.."

cargo run --bin node
