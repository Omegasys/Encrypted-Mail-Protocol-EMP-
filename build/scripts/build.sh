#!/bin/bash

echo "=== Building EMP ==="

cd "$(dirname "$0")/.."

cargo build

if [ $? -eq 0 ]; then
    echo "Build successful"
else
    echo "Build failed"
fi
