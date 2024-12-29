# enel-stop

Web crawler that whenever a new maintenance window is reported notifies users of the service using an SMS.

## Configuration

The properties in the configuration file can be overwritten by setting environment variables in the shell the service is supposed to run in.
Example: ``exec env service.refresh_ms=1000 RUST_LOG=debug cargo run config.toml``

## TODO

- parse date from the title of the incident and store it as attribute of the record
- store incidents in redis
- enable persistence in redis

