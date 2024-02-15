#!/bin/bash

set -euo pipefail

builder="${1}"
# runner="${2}"

#tag=berquist/boys
tag="${FQ_IMAGE_NAME}"
path=/code/boys/cobertura.xml

if [[ "${builder}" == buildah ]]; then
    "${builder}" build -t "${tag}" .
    "${builder}" unshare ./build_image_unshare.sh "${tag}" ${path}
fi
# container_id="$(${runner} create ${tag})"
# ${runner} cp "${container_id}":/code/boys/cobertura.xml .
# ${runner} container rm "${container_id}"
# ${runner} rmi ${tag}
