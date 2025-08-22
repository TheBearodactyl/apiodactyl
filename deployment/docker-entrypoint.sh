#!/bin/bash
set -euo pipefail

echo "Waiting for postgres to be ready..."
until pg_isready -h db -U bearodactyl; do
  sleep 1
done

export DATABASE_URL="postgres://bearodactyl:${POSTGRES_PASSWORD}@db/bearodata"

echo "Running diesel migrations..."
diesel migration run

echo "Starting api server..."
exec ./apiodactyl
