#!/bin/bash
set -eEuo pipefail

IS_A_RELEASE="false"
PROD_RELEASE="false"
DEV_RELEASE="false"
workdir="$( cd "$( dirname "${BASH_SOURCE[0]}" )/.." &> /dev/null && pwd )"
github_tag="${GITHUB_REF_NAME:-}"
prod_release_branch="${PROD_RELEASE_BRANCH:-main}"
dev_release_branch="${DEV_RELEASE_BRANCH:-dev}"
maintainers_keys="${MAINTAINERS_KEYS:-}"
prod_release_regex='^[0-9]+\.[0-9]+\.[0-9]+\-[0-9]+\.[0-9]+\.[0-9]+$'
dev_release_regex='^[0-9]+\.[0-9]+\.[0-9]+\-[0-9]+\.[0-9]+\.[0-9]+(-dev[0-9]*){1}$'

export COMMON_FILE_LOCATION="ci/common.sh"
# Requirement
if ! [ -f "${workdir}/${COMMON_FILE_LOCATION}" ]; then
  echo -e "\n\033[1;31mERROR: ${COMMON_FILE_LOCATION} file is missing !!! \033[0m\n"
  return
else
  # shellcheck disable=SC1090
  source "${COMMON_FILE_LOCATION}"
fi

# Functions
import_gpg_keys() {
  # shellcheck disable=SC2207
  declare -r keys=( $(echo "${@}" | tr " " "\n") )

  if [ "${#keys[@]}" -eq 0 ]; then
    log_warn "WARNING: there are ZERO gpg keys to import. Please check if 'MAINTAINERS_KEYS' variable is set correctly. The build is not going to be released ..."
    IS_A_RELEASE="false"
  else
    # shellcheck disable=SC2145
    printf "%s\n" "Tagged build, fetching keys:" "${@}" ""
    for key in "${keys[@]}"; do
      gpg -v --batch --keyserver hkps://keys.openpgp.org --recv-keys "${key}" ||
      gpg -v --batch --keyserver hkp://keyserver.ubuntu.com --recv-keys "${key}" ||
      gpg -v --batch --keyserver hkp://pgp.mit.edu:80 --recv-keys "${key}" ||
      gpg -v --batch --keyserver hkp://keyserver.ubuntu.com:80 --recv-keys "${key}" ||
      { log_warn "WARNING: ${key} can not be found on GPG key servers. Please upload it to at least one of the following GPG key servers:\nhttps://keys.openpgp.org/\nhttps://keyserver.ubuntu.com/\nhttps://pgp.mit.edu/"
        IS_A_RELEASE="false"
      }
    done
  fi
}

check_signed_tag() {
  local tag="${1}"

  if git verify-tag -v "${tag}"; then
    log_info "INFO: ${tag} is a valid signed tag"
  else
    log_warn "WARNING: GIT's tag = ${tag} signature is NOT valid. The build is not going to be released ..."
    IS_A_RELEASE="false"
  fi
}


####
# Main
####
log_info "Production release branch is:   ${prod_release_branch}"
log_info "Development release branch is:  ${dev_release_branch}"
log_info "GitHub tag is:                  ${github_tag}"

# Checking release requirements
if [ -z "${maintainers_keys}" ]; then
  log_warn "WARNING: 'MAINTAINERS_KEYS' variable is not set. The build is not going to be released ..."
fi

# Checking if it is a release build
if git branch -r --contains "${github_tag}" | grep -xqE ". origin\/${prod_release_branch}$"; then
  IS_A_RELEASE="true"
  import_gpg_keys "${maintainers_keys}"
  check_signed_tag "${github_tag}"

  if [ "${IS_A_RELEASE}" = "true" ]; then
    if [[ "${github_tag}" =~ ${prod_release_regex} ]]; then
      PROD_RELEASE="true"
    else
      log_warn "WARNING: GitHub tag: ${github_tag} is in the wrong format for 'PRODUCTION' release. Expecting the following format: 'd.d.d-d.d.d'. The build is not going to be released ..."
      IS_A_RELEASE="false"
    fi
  fi
elif git branch -r --contains "${github_tag}" | grep -xqE ". origin\/${dev_release_branch}$"; then
  IS_A_RELEASE="true"
  import_gpg_keys "${maintainers_keys}"
  check_signed_tag "${github_tag}"

  if [ "${IS_A_RELEASE}" = "true" ]; then
    if [[ "${github_tag}" =~ ${dev_release_regex} ]]; then
      DEV_RELEASE="true"
    else
      log_warn "WARNING: GitHub tag: ${github_tag} is in the wrong format for 'DEVELOPMENT' release. Expecting the following format: 'd.d.d-d.d.d-dev'. The build is not going to be released ..."
      IS_A_RELEASE="false"
    fi
  fi
else
  log_warn "WARNING: GitHub tag = ${github_tag} does NOT derive from either '${prod_release_branch}' OR '${dev_release_branch}' branch. The build is not going to be released ..."
  IS_A_RELEASE="false"
fi

# Final check for release vs non-release build
if [ "${IS_A_RELEASE}" = "true" ]; then
  export IS_A_RELEASE="true"
  if [ "${PROD_RELEASE}" = "true" ]; then
    echo "" && log_info "=== This is a 'Production' release build ===" && echo ""
    export PROD_RELEASE="true"
  elif [ "${DEV_RELEASE}" = "true" ]; then
    echo "" && log_info "=== This is a 'Development' release build ===" && echo ""
    export DEV_RELEASE="true"
  fi
elif [ "${IS_A_RELEASE}" = "false" ]; then
  echo "" && log_warn "WARNING: This is NOT a RELEASE build" && echo ""
fi

set +eo pipefail
