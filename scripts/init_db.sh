#!/usr/bin/env bash
set -x
set -eo pipefail

if ! command -v psql > /dev/null; then
  echo >&2 "Error: psql is not installed"
  exit 1
fi

if ! command -v sqlx > /dev/null; then
  echo >&2 "Error: sqlx is not installed"
  exit 1
fi


#check if a custom user has been set, otherwise default
DB_USER="${POSTGRES_USER:=postgres}"
#check if a custom password has been set, otherwise default
DB_PASSWORD="${POSTGRES_PASSWORD:=password}"
#check if a custom database name has been set, otherwise default
DB_NAME="${POSTGRES_DB:=newsletter}"
#check if a custom port has been set, otherwise default
DB_PORT="${DB_PORT:=5432}"
#check if a custom host has been set, otherwise default
DB_HOST="${POSTGRES_HOST:=localhost}"

# Launch postgres using Docker
if [[ -z "${SKIP_DOCKER}" ]]
then
  docker run \
    -e POSTGRES_USER=${POSTGRES_USER} \
    -e POSTGRES_PASSWORD=${POSTGRES_PASSWORD} \
    -e POSTGRES_DB=${DB_NAME} \
    -p "${DB_PORT}":5432 \
    -d postgres \
    postgres -N 1000 # Increases max connections
fi

export PGPASSWORD="${DB_PASSWORD}"

until psql -h "${DB_HOST}" -U "${DB_USER}" -p "${DB_PORT}" -d "postgres" -c '\q'; do
  >&2 echo "Postgres is still unavailable - sleeping"
  sleep 1
done

>&2 echo "Postgres is up and running on port ${DB_PORT}!"

DATABASE_URL=postgres://${DB_USER}:${DB_PASSWORD}@${DB_HOST}
export DATABASE_URL
sqlx database create
sqlx migrate run

>&2 echo "Postgres has been migrated, ready to go!"