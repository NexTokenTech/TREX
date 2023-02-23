#!/usr/bin/env bash
# This script is meant to be run on Unix/Linux based systems
set -e

echo "*** Start TREX node ***"

cd $(dirname ${BASH_SOURCE[0]})/..

# migrate from earlier docker-compose Python program to Go-based docker/compose
docker compose down --remove-orphans
docker compose run --rm --service-ports dev $@
