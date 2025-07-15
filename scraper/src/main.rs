use tokio::main;
use db;

#[main]
async fn main() {
    db::init().await;
    // let conn = db::connect_to_db();

    // TODO: fill out the database with information
}
