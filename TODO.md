# The plan

## TODO
- implement notification based on rules (RSS seems to be NOT updated anymore)

## DONE
- show incidents on a map
- synch secrets across namespaces (app and cnpg-cluster)
- provide pagination for the table
- query postgres for serving incidents
- populate postgres for testing purposes
- switch to storage in postgres using sqlx
- implement data migration from Redis to PostgreSQL
- move postgres migration to a synch postgres client so to avoid the issues with sqlx in a synch env
- set up Liquibase for PostgreSQL database migration (based on the Incident struct)
- integration tests for web api
- show keys and json values in a sorted table (no pagination)
- move the migration to another workspace so to run it independently
- use openapi for generating the structures
- build container for SPA and serve static files with nginx
- basic SPA
- expose number of keys in the redis server (NO migration yet)
- be able to start a web server, deploy it to k8s and access it from outside the k8s cluster
- run web server
