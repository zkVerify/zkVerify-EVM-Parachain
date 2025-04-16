#!/usr/bin/env bash
set -eEuo pipefail

# This script is inspired from :
# https://github.com/zkVerify/zkVerify/blob/dr/relay-poc-update/docker/scripts/entrypoint.sh
# (extended to handle also parachain parameters)
#
# It performs the following tasks:
#
# - translation of environment variables to command line arguments
# - preparation before the node start (example keys injection)
# - launch of the actual node
#
# Environment variables should generally be in the form `EVM_*` for the parachain parameters and `ZKV_*` for the relay chain parameters.
# They are translated to command line arguments based on these rules:
#
# 1. `EVM_` and `ZKV_CONF_` prefix are removed
# 2. if underscores (`_`) are present, they are replaced with dashes (`-`)
# 3. letters are replaced with lower case
# 4. prefix `--` is added
#
# Examples:
#
# - `ZKV_CONF_BASE_PATH` -> `--base-path`
# - `ZKV_CONF_BOOTNODES` -> `--bootnodes`
#
# Values of environment variables are used unmodified as values of command line arguments with the exception
# of `true` being dropped (as a flag, example `ZKV_CONF_VALIDATOR`/`--validator`)

####
# Function(s)
####
fn_die() {
  echo -e "\n\033[1;31m${1}\033[0m\n" >&2
  exit "${2:-1}"
}

log_bold_green() {
  echo -e "\n\033[1;32m${1}\033[0m\n"
}

log_green() {
  echo -e "\n\033[0;32m${1}\033[0m\n"
}

log_yellow() {
  echo -e "\n\033[1;33m${1}\033[0m\n"
}

get_arg_name_from_env_name() {
  local env_name="$1"
  local prefix="$2"

  # Extract the base name by removing the prefix
  local base_name="${env_name:${#prefix}}"

  # Replace underscores with hyphens and convert to lowercase
  arg_name="${base_name//_/-}"
  arg_name="${arg_name,,}"

  # Prefix the argument name with --
  echo "--${arg_name}"
}

get_arg_value_from_env_value() {
  local env_value="$1"
  local arg_value="${env_value}"

  # Check if the value is exactly "true".
  if [ "${arg_value}" == "true" ]; then
    # If it is "true", set arg_value to an empty string (""), indicating no flag should be set.
    arg_value=""
  fi

  # Output the processed arg_value
  echo "${arg_value}"
}

# Function to validate chain specification and download if necessary
validate_and_download() {
  local CHAIN_VAR_NAME="$1"
  local URL_VAR_NAME="$2"

  # Dynamically retrieve the values of the variables using indirect expansion
  local CHAIN_VALUE="${!CHAIN_VAR_NAME}"
  local SPEC_FILE_URL="${!URL_VAR_NAME}"

  # Check if the chain variable is empty
  if [ -z "${CHAIN_VALUE}" ]; then
    fn_die "ERROR: '${CHAIN_VAR_NAME}' variable can not be empty or undefined. Aborting ..."
  fi

  # Echo the chain value
  echo "  ${CHAIN_VAR_NAME}=${CHAIN_VALUE}"

  # Check if CHAIN_VALUE points to an existing .json file and download it otherwise
  if [[ "${CHAIN_VALUE}" == *.json ]] && [ ! -f "${CHAIN_VALUE}" ] ; then
    # Attempt to download the file if it doesn't exist
    if [ -n "${SPEC_FILE_URL}" ]; then
      log_green "INFO: Spec file '${CHAIN_VALUE}' does not exist. Downloading it from '${SPEC_FILE_URL}' ..."
      mkdir -p "$(dirname "${CHAIN_VALUE}")" || fn_die "ERROR: could not create directory '$(dirname "${CHAIN_VALUE}")' for spec file. Aborting ..."
      cd "$(dirname "${CHAIN_VALUE}")"
      aria2c --file-allocation=none -s16 -x16 --max-tries=3 --continue=true "${SPEC_FILE_URL}" -o "$(basename "${CHAIN_VALUE}")" || fn_die "ERROR: Failed to download spec file from '${SPEC_FILE_URL}' url. Aborting ..."
    else
      fn_die "ERROR: The variable '${CHAIN_VAR_NAME}' (spec file) is set to '${CHAIN_VALUE}', which is a .json file that does not exist. The variable '${URL_VAR_NAME}' is empty, therefore the file can not be downloaded. Aborting ..."
    fi
  fi
}

####
# Main
####
# Sanity check for BINARY variable being defined under Dockerfile
if [ -z "${BINARY:-}" ]; then
  fn_die "ERROR: Required environment variable 'BINARY' is not defined. This should never happen. Aborting ..."
else
  ZKV_EVM_PARA_NODE_BIN="${BINARY}"
  log_bold_green "🔧 zkv-para-evm node binary: ${ZKV_EVM_PARA_NODE_BIN}"
fi

####
# Parachain node's configurations
####
log_bold_green "=== Parachain node's configuration:"
EVM_CONF_BASE_PATH="${EVM_CONF_BASE_PATH:-}"
EVM_CONF_CHAIN="${EVM_CONF_CHAIN:-}"
EVM_SPEC_FILE_URL="${EVM_SPEC_FILE_URL:-}"

# Call the function for EVM_CONF_CHAIN
validate_and_download "EVM_CONF_CHAIN" "EVM_SPEC_FILE_URL"

EVM_SECRET_PHRASE_PATH="${EVM_SECRET_PHRASE_PATH:-"/data/config/secret_phrase_para.dat"}"
EVM_NODE_KEY_FILE="${EVM_NODE_KEY_FILE:-"/data/config/node_key_para.dat"}"
echo "  EVM_SECRET_PHRASE_PATH=${EVM_SECRET_PHRASE_PATH}"
echo -e "  EVM_NODE_KEY_FILE=${EVM_NODE_KEY_FILE}\n"

prefix="EVM_CONF_"
conf_args=()
while IFS='=' read -r -d '' var_name var_value; do
  if [[ "${var_name}" == "${prefix}"* ]]; then
    # Get argument name from the environment variable name
    arg_name="$(get_arg_name_from_env_name "${var_name}" "${prefix}")"

    # If the value contains commas, handle it by splitting the values
    if [[ "${var_value}" == *","* ]]; then
      IFS=',' read -ra values <<< "${var_value}"
      for value in "${values[@]}"; do
        # Add the argument name and each value
        conf_args+=("${arg_name}")
        conf_args+=("${value}")
      done
    else
      # If there is no comma, just add the argument with the value
      #arg_value="$(get_arg_value_from_env_value "${var_value}")"
      if [ "${var_value}" != "true" ]; then
        conf_args+=("${arg_name}")
        conf_args+=("${var_value}")
      else
        conf_args+=("${arg_name}")
      fi
    fi

    # Debug output
    echo "  ${var_name}=${var_value} -> ${arg_name} ${var_value}"
  fi
done < <(env -0)

# Parachain keys handling
if [ -f "${EVM_SECRET_PHRASE_PATH}" ]; then
  injection_args=()
  if [ -n "${EVM_CONF_BASE_PATH}" ]; then
    injection_args+=("$(get_arg_name_from_env_name EVM_CONF_BASE_PATH "${prefix}")")
    injection_args+=("$(get_arg_value_from_env_value "${EVM_CONF_BASE_PATH}")")
  fi
  if [ -n "${EVM_CONF_CHAIN}" ]; then
    injection_args+=("$(get_arg_name_from_env_name EVM_CONF_CHAIN "${prefix}")")
    injection_args+=("$(get_arg_value_from_env_value "${EVM_CONF_CHAIN}")")
  fi
  log_green "INFO: injecting keys with ${injection_args[*]} ..."

  log_green "INFO: injecting key (Aura) ..."
  "${ZKV_EVM_PARA_NODE_BIN}" key insert "${injection_args[@]}" \
    --scheme Sr25519 \
    --suri "${EVM_SECRET_PHRASE_PATH}" \
    --key-type aura

  #TODO: ethereum key maybe not needed?
  log_green "INFO: injecting key (Ethereum) ..."
  "${ZKV_EVM_PARA_NODE_BIN}" key insert "${injection_args[@]}" \
    --scheme ecdsa \
    --suri "${EVM_SECRET_PHRASE_PATH}"  \
    --key-type acco
fi

# Parachain Node-key (used for p2p) handling
if [[ (-n "${EVM_CONF_BASE_PATH}") && (-n "${EVM_CONF_CHAIN}") && (-f "${EVM_NODE_KEY_FILE}") ]]; then
  base_path="$(get_arg_value_from_env_value "${EVM_CONF_BASE_PATH}")"
  chain="$(get_arg_value_from_env_value "${EVM_CONF_CHAIN}")"
  chain_id="$("${ZKV_EVM_PARA_NODE_BIN}" build-spec --chain "${chain}" 2>/dev/null | jq -r '.id')" || chain_id='null'
  if [ -z "${chain_id}" ] || [ "${chain_id}" == 'null' ]; then
    fn_die "ERROR: could not find 'id' under parachain spec file. Aborting ..."
  fi
  destination="${base_path}/chains/${chain_id}/network"

  mkdir -p "${destination}"
  log_green "INFO: copying parachain node key file to ${destination} location ..."
  cp "${EVM_NODE_KEY_FILE}" "${destination}/secret_ed25519"
fi

####
# Relaychain node's configurations
####
log_bold_green "=== Relaychain collator's configuration:"
ZKV_CONF_BASE_PATH="${ZKV_CONF_BASE_PATH:-}"
ZKV_CONF_CHAIN="${ZKV_CONF_CHAIN:-}"
ZKV_SPEC_FILE_URL="${ZKV_SPEC_FILE_URL:-}"

# Call the function for ZKV_CONF_CHAIN
validate_and_download "ZKV_CONF_CHAIN" "ZKV_SPEC_FILE_URL"

ZKV_SECRET_PHRASE_PATH="${ZKV_SECRET_PHRASE_PATH:-"/data/config/secret_phrase_relay.dat"}"
ZKV_NODE_KEY_FILE="${ZKV_NODE_KEY_FILE:-"/data/config/node_key_relay.dat"}"

echo "  ZKV_SECRET_PHRASE_PATH=${ZKV_SECRET_PHRASE_PATH}"
echo -e "  ZKV_NODE_KEY_FILE=${ZKV_NODE_KEY_FILE}\n"

prefix="ZKV_CONF_"
relaychain_appended_any=""
# Read environment variables
while IFS='=' read -r -d '' var_name var_value; do
  if [[ "${var_name}" == "${prefix}"* ]]; then
    # Append separator only once
    if [[ -z "${relaychain_appended_any}" ]]; then
      relaychain_appended_any="true"
      conf_args+=("--")
    fi

    # Get argument name from the environment variable name
    arg_name="$(get_arg_name_from_env_name "${var_name}" "${prefix}")"

    # If the value contains commas, handle it by splitting the values
    if [[ "${var_value}" == *","* ]]; then
      IFS=',' read -ra values <<< "${var_value}"
      for value in "${values[@]}"; do
        # Add the argument name and each value
        conf_args+=("${arg_name}")
        conf_args+=("${value}")
      done
    else
      # If there is no comma, just add the argument with or without the value depending on the condition
      if [ "${var_value}" != "true" ]; then
        conf_args+=("${arg_name}")
        conf_args+=("${var_value}")
      else
        conf_args+=("${arg_name}")
      fi
    fi

    # Debug output
    echo "  ${var_name}=${var_value} -> ${arg_name} ${var_value}"
  fi
done < <(env -0)

# Relay Keys handling is not done in parahcain nodes (Babe, Grandpa, Imonline keys are only used on relay chain validators)
if [ -f "${ZKV_SECRET_PHRASE_PATH}" ]; then
  log_yellow "WARNING: ZKV_SECRET_PHRASE_PATH ENV is not used on the relaychain node. It will be ignored."
fi

# Relay Node-key (used for p2p) handling (Not sure really needed, maybe the parachain one is enough?)
if [[ (-n "${ZKV_CONF_BASE_PATH}") && (-n "${ZKV_CONF_CHAIN}") && (-f "${ZKV_NODE_KEY_FILE}") ]]; then
  relay_base_path="$(get_arg_value_from_env_value "${ZKV_CONF_BASE_PATH}")"
  chain="$(get_arg_value_from_env_value "${EVM_CONF_CHAIN}")"
  relay_chain_id="$("${ZKV_EVM_PARA_NODE_BIN}" build-spec --chain "${chain}" 2>/dev/null | jq -r '.relay_chain')" || relay_chain_id='null'
  if [ -z "${relay_chain_id}" ] || [ "${relay_chain_id}" == 'null' ]; then
    fn_die "ERROR: could not find 'relay_chain' under parachain spec file. Aborting ..."
  fi
  destination="${relay_base_path}/chains/${relay_chain_id}/network"

  mkdir -p "${destination}"
  log_green "INFO: copying relaychain node key file to ${destination} location ..."
  cp "${ZKV_NODE_KEY_FILE}" "${destination}/secret_ed25519"
fi

log_green "INFO: launching ${ZKV_EVM_PARA_NODE_BIN} with the following args:"
echo "  ${conf_args[*]}" "$@"

exec "${ZKV_EVM_PARA_NODE_BIN}" "${conf_args[@]}" "$@"
