# enel-stop

Web crawler that whenever a new maintenance window is reported notifies users of the service using an SMS.

## Configuration

The properties in the configuration file can be overwritten by setting environment variables in the shell the service is supposed to run in.
Example: ``exec env service.refresh_ms=1000 RUST_LOG=debug cargo run config.toml``

## Deployment

- checkout the repository
- generate the kustomize script in the kustomize folder: ```kubectl kustomize --load-restrictor LoadRestrictionsNone --enable-helm . > output.yaml```
- apply the output yaml file

## TODO

- more test coverage

## Learning topics

- Smart pointers, like Arc
- mocking of structs with impl, without traits
- web application development
