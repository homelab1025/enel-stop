# enel-stop

Web crawler that whenever a new maintenance window is reported notifies users of the service using an SMS.

## Next steps

- run chrome in another container
- show results in a webpage served by a separate container running an http server

## Development Tools

### How to run test coverage

```bash
cargo llvm-cov --html
```

### How to vet dependencies

```bash
cargo vet check
```

The reports are generated in target/llvm-cov/html and you can serve them directly over http with `python3 -m http.server 8000`.

## Profiling

You can profile the executable using valgrind tools. This should reflect the requirements of the executable, but not the whole container.

```bash
valgrind --tool=massif target/release/browsenscrape conf/config.toml
```

After the process is finished you get a massif.out file which has the PID as a suffix. You can view that using ms_print

```bash
ms_print massif.out.2789079 > memory.txt
```

Redirect the output of ms_print to a file as it can be quite large.

## Configuration

The properties in the configuration file can be overwritten by setting environment variables in the shell the service is supposed to run in.
Example: ``exec env service.refresh_ms=1000 RUST_LOG=debug cargo run config.toml``

## Deployment

- checkout the repository
- generate the kustomize script in the kustomize folder: ```kubectl kustomize --load-restrictor LoadRestrictionsNone --enable-helm . > output.yaml```
- apply the output yaml file
