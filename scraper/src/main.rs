use db;

fn main() {
    db::init_db();
    let conn = db::connect_to_db();
}
