use std::env;

use bollard::Docker;
use bollard::models::ContainerCreateBody;
use bollard::query_parameters::{CreateContainerOptionsBuilder, RemoveContainerOptionsBuilder, StartContainerOptionsBuilder};
use dotenvy::dotenv;
use postgres::Client;

const CONTAINER_NAME: &str = "tsearch";
const CONTAINER_VERSION: &str = "postgres";

// TODO: remove and pull from repo environment variables
const PASSWORD_KEY: &str = "POSTGRES_PASSWORD";
const DEFAULT_DB_USERNAME: &str = "postgres";

// utilize the "bollard" crate from crates.io to manage postgres DB container.
// the container will essentially be the same as having run:
// docker run --name tsearch-db --env-file .env -d postgres
pub async fn init() {
    // TODO: properly handle errors
    let docker = Docker::connect_with_defaults()
        .expect("ERROR: Failed to connect to docker environment");
    println!("Successfully connected to the docker environment.");

    // clean up existing container if exists
    // let containers = docker.list_containers().await.unwrap();
    let remove_options = RemoveContainerOptionsBuilder::default()
        .force(true)
        .build();
    match docker.remove_container(CONTAINER_NAME, Some(remove_options)).await {
        Ok(_) => println!("Successfully removed container '{CONTAINER_NAME}'"),
        Err(_) => println!("No pre-existing container '{CONTAINER_NAME}' was removed"),
    }

    let db_password = get_db_password();
    let env = Vec::from([format!("POSTGRES_PASSWORD='{db_password}'")]);
    docker.create_container(
        Some(
            CreateContainerOptionsBuilder::default()
                .name(CONTAINER_NAME)
                .build()
        ),
        ContainerCreateBody{
            image: Some(String::from(CONTAINER_VERSION)),
            env: Some(env),
            ..Default::default()
        }
    ).await.expect("ERROR: Failed to create new container.");
    println!("Successfully created '{CONTAINER_NAME}' container.");

    let start_options = StartContainerOptionsBuilder::default().build();
    docker.start_container(CONTAINER_NAME, Some(start_options))
        .await
        .expect("ERROR: failed to start new container.");
    println!("Successfully started '{CONTAINER_NAME}' container.");

    // TODO: apply database migrations

}

pub fn connect_to_db() -> Client {
    let db_password = get_db_password();

    let mut db_config = postgres::Config::new();
    db_config.user(DEFAULT_DB_USERNAME)
        .password(db_password)
        .dbname("postgres")
        .host("localhost")
        .port(9999);

    return db_config.connect(postgres::NoTls).expect("failed to connect to db");
}

fn get_db_password() -> String {
    // load .env into local environment variables
    dotenv().ok();

    let db_password: String = env::var(PASSWORD_KEY)
        .expect("The postgres password is not set in the .env file.");
    return db_password;
}
