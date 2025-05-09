#!/bin/bash
set -eEuo pipefail

workdir="$( cd "$( dirname "${BASH_SOURCE[0]}" )/.." &> /dev/null && pwd )"
docker_hub_org="${DOCKER_HUB_ORG:-zkvparaevm}"
docker_image_build_name="${DOCKER_IMAGE_BUILD_NAME:-zkvparaevm-node}"
docker_hub_username="${DOCKER_HUB_USERNAME:-}"
docker_hub_token="${DOCKER_HUB_TOKEN:-}"
is_a_release="${IS_A_RELEASE:-false}"
prod_release="${PROD_RELEASE:-false}"
dev_release="${DEV_RELEASE:-false}"
github_ref_name="${GITHUB_REF_NAME:-}"
common_file_location="${COMMON_FILE_LOCATION:-not-set}"
docker_file_path='docker/dockerfiles/zkvparaevm-node.Dockerfile'

# Requirement
if ! [ -f "${common_file_location}" ]; then
  echo -e "\n\033[1;31mERROR: ${common_file_location} file is missing !!!  Exiting ...\033[0m\n"
  exit 1
else
  # shellcheck disable=SC1090
  source "${common_file_location}"
fi


####
# Main
####
cd "${workdir}"

if [ -z "${docker_hub_token:-}" ]; then
  fn_die "ERROR: DOCKER_HUB_TOKEN variable is not set. Exiting ..."
fi

if [ -z "${docker_hub_username:-}" ]; then
  fn_die "ERROR: DOCKER_HUB_USERNAME variable is not set. Exiting ..."
fi

docker_tag_full=""
if [ "${is_a_release}" = "true" ]; then
  docker_tag_full="${github_ref_name}"
fi

# Building and publishing docker image
if [ -n "${docker_tag_full:-}" ]; then
  log_info "=== Building Docker image: ${docker_hub_org}/${docker_image_build_name}:${docker_tag_full} ==="
  docker build -f "${docker_file_path}" -t "${docker_hub_org}/${docker_image_build_name}:${docker_tag_full}" .

  # Publishing to DockerHub
  log_info "=== Publishing Docker image(s) on Docker Hub ==="
  echo "${docker_hub_token}" | docker login -u "${docker_hub_username}" --password-stdin

  # Docker image(s) tags for PROD vs DEV release
  if [ "${prod_release}" = "true" ]; then
    docker_tag_node="$(cut -d '-' -f1 <<< "${docker_tag_full}")"
    publish_tags=("${docker_tag_full}" "${docker_tag_node}" "latest")
  elif [ "${dev_release}" = "true" ]; then
    docker_tag_node="$(cut -d '-' -f1 <<< "${docker_tag_full}")-$(cut -d '-' -f3- <<< "${docker_tag_full}")"
    publish_tags=("${docker_tag_full}" "${docker_tag_node}")
  fi

  for publish_tag in "${publish_tags[@]}"; do
    log_info "Publishing docker image: ${docker_image_build_name}:${publish_tag}"
    docker tag "${docker_hub_org}/${docker_image_build_name}:${docker_tag_full}" "index.docker.io/${docker_hub_org}/${docker_image_build_name}:${publish_tag}"
    docker push "index.docker.io/${docker_hub_org}/${docker_image_build_name}:${publish_tag}"
  done
else
  fn_die "ERROR: the build did NOT satisfy 'RELEASE' build requirements. Docker image(s) was(were) NOT build and/or published."
fi

exit 0
