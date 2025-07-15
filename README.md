# tsearch - The Turley Search Engine

## Setup

First set a password for the DB in a local `.env` file:
```bash
echo "POSTGRES_PASSWORD='password'" > .env
```


## Running the Application
```bash
# populate the db
cargo run -p scraper

# run the application
cargo run -p server
```
