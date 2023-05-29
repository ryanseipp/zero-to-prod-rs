#!/usr/bin/env bash

set -e

RUNNING_CONTAINER=$(docker ps --filter 'name=redis' --format '{{.ID}}')

if [[ -n $RUNNING_CONTAINER ]]; then
    echo "there is a redis container already running. stop it with"
    echo "    docker stop ${RUNNING_CONTAINER}"
    exit 1
fi

docker run \
    -p "6379:6379" \
    -d \
    --name "redis_$(date '+%s')" \
    redis:7

echo "Redis is ready to go!"
