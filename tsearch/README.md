# `tsearch` Server Application

If smaller image or build time is required, see additional links from: https://hub.docker.com/_/rust

For faster build times, it may make sense to copy a static binary into a `FROM scratch` container.

## Instructions for Running
Running Locally:
```bash
cargo run .
```

Running in Container:
```bash
# create the docker image
docker build -t tsearch .

# run this container, and automatically clean it up when it's stopped
docker run --rm tsearch

# stop the container
docker stop tsearch
```