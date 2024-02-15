#!/bin/bash

set -euo pipefail

tag="${1}"
path="${2}"

container_name="$(buildah from "${tag}")"
container_mount="$(buildah mount "${container_name}")"
cp -a "${container_mount}/${path}" .
buildah rm "${container_name}"
