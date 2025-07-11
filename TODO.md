# The plan

## TODO
- implement data migration from Redis to PostgreSQL
- provide pagination for the table
- show incidents on a map
- write integration tests for migration utility

## DONE
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
