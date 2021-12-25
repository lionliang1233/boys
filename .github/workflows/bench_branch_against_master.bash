#!/usr/bin/env bash

# Inspired by
# https://medium.com/@yamafaktory/rust-benchmarking-with-criterion-on-travis-ci-%EF%B8%8F-8b54d321e05

set -euo pipefail

# Clone the repository
remote_url="$(git config --get remote.origin.url)"
cd "${TRAVIS_BUILD_DIR}"/..
git clone "${remote_url}" ${TRAVIS_REPO_SLUG}-bench
cd ${TRAVIS_REPO_SLUG}-bench

# Bench master
git checkout master
cargo bench -- --noplot --save-baseline before

# Bench current branch
git checkout ${TRAVIS_COMMIT}
cargo bench -- --noplot --save-baseline after

# Install https://github.com/BurntSushi/critcmp and compare!
cargo install --force critcmp
critcmp before after
