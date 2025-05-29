# The plan

## TODO
- move the migration to another workspace so to run it independently
- show keys and json values (no pagination)
- run the migration and expose the incidents in a table
- provide pagination

## DONE
- use openapi for generating the structures
- build container for SPA and serve static files with nginx
- basic SPA
- expose number of keys in the redis server (NO migration yet)
- be able to start a web server, deploy it to k8s and access it from outside the k8s cluster
- run web server