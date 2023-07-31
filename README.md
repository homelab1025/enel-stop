# enel-stop

Web crawler that whenever a new maintenance window is reported notifies users of the service using an SMS.

## Configuration

The properties in the configuration file can be overwritten by setting environment variables in the shell the service is supposed to run in.
Example: ``exec env service.refresh_ms=1000 RUST_LOG=debug cargo run config.toml``

## Release

## Versions

### version 1
- print incidents according to the pattern every x seconds (configurable)

### version 2
- store the incidents and updated them in a redis database

## Future features

- Simple loop for scanning the RSS from E-distributie.com. Whenever a certain location is scheduled for downtime a notification will be sent out.
- Persist the maintenance windows and don't send out notifications if they were already sent to the user.
- Instead of using threads switch over to async programming using tokio and keep the functionality as it is.
- Generalize the service to do web crawling as well. For example, watch of the changes of a price of particular product you watch.

## Curiosities
1. What would be needed to send notifications to the browser instead of using SMS.