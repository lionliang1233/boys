#!/bin/bash

set -euo pipefail

runner=podman
tag=berquist/boys

command -v ${runner} || runner=docker

${runner} buildx build -t ${tag} .
container_id="$(${runner} create ${tag})"
${runner} cp "${container_id}":/code/boys/cobertura.xml .
${runner} container rm "${container_id}"
${runner} rmi ${tag}
