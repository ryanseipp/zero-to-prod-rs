#!/usr/bin/env bash
set -e

if ! [ -x "$(command -v psql)" ]; then
    echo "Error: psql is not installed."
    exit 1
fi

if ! [ -x "$(command -v sqlx)" ]; then
    echo "Error: sqlx is not installed."
    echo 'Use:'
    echo "    cargo install sqlx-cli --version=0.6.3 --no-default-features --features rustls,postgres"
    echo "to install it."
    exit 1
fi

if [ -e .env ]; then
    source .env
fi

DATABASE_USER="${DATABASE_USER:=postgres}"
DATABASE_PASSWORD="${DATABASE_PASSWORD}"
DATABASE_NAME="${DATABASE_NAME:=newsletter}"
DATABASE_PORT="${DATABASE_PORT:=5432}"
DATABASE_HOST="${DATABASE_HOST:=localhost}"

if [[ -z "${SKIP_DOCKER}" ]]; then
    CURRENT_POSTGRES=$(docker ps | rg postgres | awk '{print $1}')
    if [[ -n "${CURRENT_POSTGRES}" ]]; then
        docker stop $CURRENT_POSTGRES
        docker rm $CURRENT_POSTGRES
    fi

    docker run \
        -e POSTGRES_USER=${DATABASE_USER} \
        -e POSTGRES_PASSWORD=${DATABASE_PASSWORD} \
        -e POSTGRES_DB=${DATABASE_NAME} \
        -p "${DATABASE_PORT}":5432 \
        -d postgres \
        postgres -N 1000
fi

# Keep pinging Postgres until it's ready to accept commands
export PGPASSWORD="${DATABASE_PASSWORD}"
until psql -h "${DATABASE_HOST}" -U "${DATABASE_USER}" -p "${DATABASE_PORT}" -d "postgres" -c '\q'; do
    echo "Postgres is still unavailable - sleeping"
    sleep 1
done

echo "Postgres is up and running on port ${DATABASE_PORT}!"

export DATABASE_URL=postgres://${DATABASE_USER}:${DATABASE_PASSWORD}@${DATABASE_HOST}:${DATABASE_PORT}/${DATABASE_NAME}
sqlx database create
sqlx migrate run

echo "Postgres has been migrated, ready to go!"
