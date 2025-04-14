# tsearch - The Turley Search Engine

## Setup

First set a password for the DB in a local `.env` file:
```bash
echo "POSTGRES_PASSWORD='password'" > .env
```


```bash
# populate the db
cargo run -p scraper

# run the application
cargo run -p server
```










## OLD

Running in Container:
```bash
# create the docker image
docker build -t tsearch .

# run this container, and automatically clean it up when it's stopped
docker run --rm tsearch

# stop the container
docker stop tsearch
```


## Populating the Database
```bash
# start the DB
docker run --name tsearch-db --env-file .env -d postgres

# launch the web_scraper script to populate the DB
cargo run --bin web_scraper
```


If smaller image or build time is required, see additional links from: https://hub.docker.com/_/rust

For faster build times, it may make sense to copy a static binary into a `FROM scratch` container.

## Running the Application
Running Locally:
```bash
cargo run .
```

