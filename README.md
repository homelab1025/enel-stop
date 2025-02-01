# enel-stop

Web crawler that whenever a new maintenance window is reported notifies users of the service using an SMS.

## How to run test coverage

```bash
cargo llvm-cov --html
```

## How to vet dependencies

```bash
cargo vet check
```

The reports are generated in target/llvm-cov/html and you can serve them directly over http with `python3 -m http.server 8000`.

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
