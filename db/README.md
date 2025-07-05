# Database Setup with Liquibase

This directory contains the Liquibase setup for managing database schema changes in the project using Docker.

## Prerequisites

1. Docker
2. PostgreSQL database server (can be running in Docker or on the host)

## Directory Structure

- `docker-liquibase.properties`: Configuration file for Docker-based Liquibase execution
- `Dockerfile`: Docker image definition for running Liquibase
- `run-liquibase-docker.sh`: Script to run Liquibase in Docker
- `changelog/`: Directory containing all changelog files
  - `db.changelog-master.sql`: Master changelog file that includes all individual changelog files
  - `changes/`: Directory containing individual changelog files
- `lib/`: Directory for the PostgreSQL JDBC driver (downloaded automatically by the script)

## Setup

1. Make sure Docker is installed and running.

2. Run the Docker-based Liquibase script:
   ```
   cd db
   chmod +x run-liquibase-docker.sh
   ./run-liquibase-docker.sh
   ```

   This script will:
   - Build the Docker image for Liquibase
   - Download the PostgreSQL JDBC driver if needed
   - Run the Liquibase container with the default command (status)

## Running Liquibase Commands

### Docker Execution

You can run any Liquibase command using the Docker-based approach by passing the command as an argument to the script:

```
./run-liquibase-docker.sh update
./run-liquibase-docker.sh rollbackCount 1
./run-liquibase-docker.sh status
./run-liquibase-docker.sh dbDoc /liquibase/docs/
```

#### Environment Variables

The Docker-based approach supports the following environment variables:

- `DB_HOST`: Database host (default: localhost)
- `DB_PORT`: Database port (default: 5432)
- `DB_NAME`: Database name (default: enel_stop)
- `DB_USER`: Database username (default: postgres)
- `DB_PASSWORD`: Database password (default: postgres)

Example:

```
DB_HOST=my-postgres-server DB_USER=myuser DB_PASSWORD=mypassword ./run-liquibase-docker.sh update
```

For Kubernetes deployment, these environment variables can be set in the pod specification.

### Kubernetes Deployment

A Kubernetes Job example is provided in the `kubernetes/` directory. This can be used as a starting point for deploying Liquibase in a Kubernetes cluster.

To apply the Kubernetes Job:

```
kubectl apply -f db/kubernetes/liquibase-job.yaml
```

The Job uses environment variables for database connection settings and Kubernetes Secrets for sensitive information (username and password). Make sure to create the necessary Secrets before applying the Job:

```
kubectl create secret generic postgres-credentials \
  --from-literal=username=postgres \
  --from-literal=password=postgres
```

You can modify the Job to use different Liquibase commands by changing the `args` field in the container specification.

## Adding New Changes

1. Create a new SQL file in the `changelog/changes/` directory with a sequential number prefix (e.g., `002-add-new-column.sql`).
2. Add the Liquibase formatted SQL header and changeset information.
3. Include the new file in the master changelog file (`db.changelog-master.sql`).

Example of a new changeset file:

```sql
--liquibase formatted sql

--changeset author:your-name id:002
--comment: Description of the change

ALTER TABLE incidents ADD COLUMN new_column VARCHAR(255);

--rollback ALTER TABLE incidents DROP COLUMN new_column;
```
