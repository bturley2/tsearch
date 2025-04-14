use std::env;
use dotenvy::dotenv;
use postgres::Client;

const PASSWORD_KEY: &str = "POSTGRES_PASSWORD";
const DEFAULT_DB_USERNAME: &str = "postgres";

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