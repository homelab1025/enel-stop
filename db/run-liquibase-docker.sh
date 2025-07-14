#!/bin/bash
set -e

echo "Running Liquibase in Docker..."

# Build the Docker image
echo "Building Liquibase Docker image..."
docker build -t enel-stop-liquibase .

# Check if the PostgreSQL JDBC driver exists, download if not
if [ ! -f "lib/postgresql-42.6.0.jar" ]; then
    echo "Downloading PostgreSQL JDBC driver..."
    mkdir -p lib
    curl -L https://jdbc.postgresql.org/download/postgresql-42.6.0.jar -o lib/postgresql-42.6.0.jar
fi

# Get the command to run (default to 'status')
LIQUIBASE_COMMAND=${1:-status}

# Run the Docker container
echo "Running Liquibase command: $LIQUIBASE_COMMAND"
docker run --rm \
    -e DB_HOST=${DB_HOST:-localhost} \
    -e DB_PORT=${DB_PORT:-5432} \
    -e DB_NAME=${DB_NAME:-enel_stop} \
    -e DB_USER=${DB_USER:-postgres} \
    -e DB_PASSWORD=${DB_PASSWORD:-postgres} \
    --network="host" \
    enel-stop-liquibase \
    $LIQUIBASE_COMMAND

echo "Liquibase Docker run completed!"