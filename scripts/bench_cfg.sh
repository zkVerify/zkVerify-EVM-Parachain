DEFAULT_BM_STEPS=50
DEFAULT_BM_REPEAT=20
DEFAULT_BM_HEAP_PAGES=4096
DEFAULT_NODE_EXE="target/production/zkv-evm-para-node"
DEFAULT_WASM="target/production/wbuild/horizen-runtime/horizen_runtime.compact.compressed.wasm"
DEFAULT_LOCAL_WEIGHT_TEMPLATE="scripts/templates/zkv-evm-pallets-weight-template.hbs"

# The following line ensure we know the project root
PROJECT_ROOT=${PROJECT_ROOT:-$(git rev-parse --show-toplevel)}
WEIGHT_OUT_PATH=${WEIGHT_OUT_PATH:-""}
WEIGHT_TEMPLATE=${WEIGHT_TEMPLATE:-"${PROJECT_ROOT}/${DEFAULT_LOCAL_WEIGHT_TEMPLATE}"}
NODE_EXE=${NODE_EXE:-"${PROJECT_ROOT}/${DEFAULT_NODE_EXE}"}
RUNTIME=${RUNTIME:-"${PROJECT_ROOT}/${DEFAULT_WASM}"}
BM_STEPS=${BM_STEPS:-${DEFAULT_BM_STEPS}}
BM_REPEAT=${BM_REPEAT:-${DEFAULT_BM_REPEAT}}
BM_HEAP_PAGES=${BM_HEAP_PAGES:-${DEFAULT_BM_HEAP_PAGES}}
