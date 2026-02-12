# enel-stop

This project is a tool for monitoring and visualizing planned electricity maintenance windows reported by utility providers (specifically "Retele Electrice", formerly Enel, in Romania).

## Functionality

The system consists of several integrated components that automate the process of data collection, storage, and visualization:

1.  **Automated Scraping**: A Python-based scraper (`scrape.py`) uses Selenium and a headless Chrome browser to navigate the utility provider's website. It bypasses cookie consents, triggers the generation of an RSS feed for planned outages, and downloads it.
2.  **Data Processing & Backend**: A Rust web server (`web_server`) receives the RSS data. It:
    *   Parses the RSS feed and extracts incident details (location, time, description).
    *   Filters incidents based on user-defined categories or locations.
    *   Stores the processed data in a **PostgreSQL** database for persistent storage and querying.
    *   Provides a REST API for the frontend and exposes Prometheus metrics for monitoring.
3.  **Visualization Frontend**: A Vue.js-based single-page application (`webapp`) that allows users to:
    *   View a list of planned outages with pagination and filtering.
    *   Visualize the geographic distribution of outages on an interactive map.
4.  **Notifications (In Development)**: The project is designed to eventually notify users via SMS when new maintenance windows are reported in their areas of interest.

## Next steps

- run chrome in another container
- show results in a webpage served by a separate container running an http server

## Development Tools

### Dependencies
- npm (for frontend)
- cargo (for backend)
- python3, selenium, requests, xvfbwrapper (for scraper)
- docker (rootless or runnable by non-root)
- openapi-generator
- liquibase (for database migrations)

### How to run test coverage

```bash
cargo llvm-cov --html
```

### Integration test using podman

The project is using testcontainers to run external dependencies. See the following link as podman needs to be installed
and a system service for the user needs to be
active: <https://stackoverflow.com/questions/71549856/testcontainers-with-podman-in-java-tests>

Also an env variable might need to be set:

```bash
export DOCKER_HOST=unix://{$XDG_RUNTIME_DIR}/podman/podman.sock
```

### How to vet dependencies

```bash
cargo vet check
```

The reports are generated in target/llvm-cov/html and you can serve them directly over http with
`python3 -m http.server 8000`.

### Profiling

You can profile the executable using valgrind tools. This should reflect the requirements of the executable, but not the
whole container.

```bash
valgrind --tool=massif target/release/web_server conf/config.toml
```

After the process is finished you get a massif.out file which has the PID as a suffix. You can view that using ms_print

```bash
ms_print massif.out.2789079 > memory.txt
```

Redirect the output of ms_print to a file as it can be quite large.

## Configuration

The properties in the configuration file can be overwritten by setting environment variables in the shell the service is
supposed to run in.
Example: ``exec env service.refresh_ms=1000 RUST_LOG=debug cargo run config.toml``

## Deployment

- checkout the repository
- generate the kustomize script in the kustomize folder:
  ```kubectl kustomize --load-restrictor LoadRestrictionsNone --enable-helm . > output.yaml```
- apply the output yaml file

## How to generate the TS SDK

Generate the openapi spec using the api_gen app.

```bash
openapi-generator generate -g typescript-axios -i openapi.yml -o webapp/src/lib/server/
```
