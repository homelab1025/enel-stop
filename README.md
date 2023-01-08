# enel-stop

Simple loop for scanning the RSS from E-distributie.com. Whenever a certain location is scheduled for downtime a notification will be sent out.

## Configuration

The properties in the configuration file can be overwritten by setting environment variables in the shell the service is supposed to run in.
Example: ``exec env service.refresh_ms=1000 RUST_LOG=debug cargo run config.toml``
