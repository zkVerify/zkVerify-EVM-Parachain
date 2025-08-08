#!/bin/bash
echo 'Building runtime wasm for nodes with debug rpc enabled.'
cd ..
echo $PWD
export WASM_BUILD_WORKSPACE_HINT=$PWD

cargo build -p vflow-runtime --release --target-dir /tmp/vflow --features evm-tracing
cp /tmp/vflow/release/wbuild/vflow-runtime/vflow_runtime.compact.compressed.wasm tests/tmp

