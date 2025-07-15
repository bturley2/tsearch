use std::env;
use dotenvy::dotenv;
use postgres::Client;

const PASSWORD_KEY: &str = "POSTGRES_PASSWORD";
const DEFAULT_DB_USERNAME: &str = "postgres";

// launches a docker container with the DB.
// If it's already running, it resets the container.
pub fn init_db() {
    // TODO - utilize the "bollard" crate from crates.io to manage postgres DB container
    // the container will essentially be the same as having run:
    // docker run --name tsearch-db --env-file .env -d postgres


}

pub fn connect_to_db() -> Client {
    // load .env into local environment variables
    dotenv().ok();

    let db_password: String = env::var(PASSWORD_KEY)
        .expect("The postgres password is not set in the .env file.");

    let mut db_config = postgres::Config::new();
    db_config.user(DEFAULT_DB_USERNAME)
        .password(db_password)
        .dbname("postgres")
        .host("localhost")
        .port(9999);

    return db_config.connect(postgres::NoTls).expect("failed to connect to db");
}