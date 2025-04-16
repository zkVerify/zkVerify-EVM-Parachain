#!/bin/bash
echo 'Building runtime wasm for nodes with debug rpc enabled.'
cd ..
echo $PWD
export WASM_BUILD_WORKSPACE_HINT=$PWD

cargo build -p zkv-para-evm-runtime --release --target-dir /tmp/zkv_para_evm --features evm-tracing
cp /tmp/zkv_para_evm/release/wbuild/zkv-para-evm-runtime/zkv_para_evm_runtime.compact.compressed.wasm tests/tmp

