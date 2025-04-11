#!/bin/bash

ECHO_CMD="${ECHO_CMD:-false}"
[ "${ECHO_CMD}" = "true" ] && set -x

PROJECT_ROOT=${PROJECT_ROOT:-$(git rev-parse --show-toplevel)}
SOURCE_ROOT=${SOURCE_ROOT:-${PROJECT_ROOT}}

. "${SOURCE_ROOT}/scripts/bench_cfg.sh"

DEFAULT_SKIP_BUILD="false"
DEFAULT_CODE_HEADER="${PROJECT_ROOT}/HEADER-APACHE2"

SKIP_BUILD=${SKIP_BUILD:-"${DEFAULT_SKIP_BUILD}"}
CODE_HEADER=${CODE_HEADER:-"${DEFAULT_CODE_HEADER}"}

PALLET=$1

function usage {
    local message=${1:-""};

    echo "$0 <pallet> : get pallet crate name, execute benchamark and save the weight file.
    Environment:
    WEIGHT_TEMPLATE : the template file path to use for rendering [${DEFAULT_LOCAL_WEIGHT_TEMPLATE} in project root].
    WEIGHT_OUT_PATH : the path of the rendered weight file. If empty it will use <pallet_path>/src/weight.rs.
    BM_STEPS        : benchmark steps [${DEFAULT_BM_STEPS}].
    BM_REPEAT       : benchmark repeat [${DEFAULT_BM_REPEAT}].
    BM_HEAP_PAGES   : benchmark heap pages [${DEFAULT_BM_HEAP_PAGES}].
    BASE_PATH_ARG   : file path to use for disk I/O benchmarks, default CWD.
    CODE_HEADER     : the path for the header file to prepend to the template render [${DEFAULT_CODE_HEADER}].
    PROJECT_ROOT    : the root of the project [the root of git project].
    SOURCE_ROOT     : the root of the source [the root of git project].
    SKIP_BUILD      : skip the build step if true [${DEFAULT_SKIP_BUILD}].
    NODE_EXE        : the path to the node executable [${DEFAULT_NODE_EXE} in project root]
    RUNTIME         : the path to the wasm runtime [${DEFAULT_WASM} in project root]
    "
    if [ -n "${message}" ]; 
    then
        echo "ERROR: $message"
    fi
    exit 1
}

check_cargo() {
    if ! cargo --list | grep -q -P "^\s+workspaces$" ; 
    then 
        usage "You need cargo-workspaces installed -> cargo install cargo-workspaces"
    fi
}

if [ -z "${PALLET}" ] ;
then
    usage
fi

if [ -z "${WEIGHT_OUT_PATH}" ];
then
    check_cargo

    if ! cargo workspaces list -l -a | grep -q -w "${PALLET}" ;
    then 
        usage "Pallet '${PALLET}' not found"
    fi

    CRATE_PATH=$(cargo workspaces list -l -a | grep -w  "${PALLET}" | awk '{print $3 }')
    
    WEIGHT_OUT_PATH="${CRATE_PATH}/src/weight.rs"
fi

echo "------------------------------------------------------------------
Use:
SKIP_BUILD=${SKIP_BUILD}
NODE_EXE=${NODE_EXE}
PALLET=${PALLET}
WEIGHT_OUT_PATH=${WEIGHT_OUT_PATH}
WEIGHT_TEMPLATE=${WEIGHT_TEMPLATE}
BM_STEPS=${BM_STEPS}
BM_REPEAT=${BM_REPEAT}
BM_HEAP_PAGES=${BM_HEAP_PAGES}
BASE_PATH_ARG=${BASE_PATH_ARG}
------------------------------------------------------------------"

if [ "${SKIP_BUILD}" = "false" ]; 
then
    check_cargo

    cd "${PROJECT_ROOT}" &&
    cargo build \
        --profile production \
        --locked \
        --features=runtime-benchmarks \
        --bin zkv-relay
    FAILED=$?
    cd - || exit 1
    if [ "${FAILED}" -ne 0 ]; then
        exit 1
    fi
fi

${NODE_EXE} \
    benchmark pallet \
    --genesis-builder=spec \
    --pallet "${PALLET}" \
    --extrinsic "*" \
    --steps "${BM_STEPS}" \
    --repeat "${BM_REPEAT}" \
    --heap-pages="${BM_HEAP_PAGES}" \
    --header "${CODE_HEADER}" \
    --output "${WEIGHT_OUT_PATH}" \
    --template "${WEIGHT_TEMPLATE}" \
    ${BASE_PATH_ARG}
