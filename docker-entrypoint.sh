#!/bin/bash
set -euo pipefail

echo "Waiting for postgres to be ready..."
until pg_isready -h db -U bearodactyl; do
  sleep 1
done

export DATABASE_URL="postgres://bearodactyl:${POSTGRES_PASSWORD}@db/bearodata"

echo "Running diesel migrations..."
diesel migration run

echo "Generating admin key..."
GEN_OUTPUT=$(./gen_admin_key)
echo "$GEN_OUTPUT"

ADMIN_KEY=$(echo "$GEN_OUTPUT" | grep "Admin Key:" | awk '{print $3}')
export BEARO_API_TOKEN="$ADMIN_KEY"

echo "Registering admin key..."
./setup_admin

echo "Starting api server..."
exec ./apiodactyl
