use std::fs;

use crate::datastore::database::Database;

const TEST_DB_PATH: &str = "/tmp/honeybot.db";

pub async fn get_test_db() -> Database {
    let db = Database::new(TEST_DB_PATH).await;
    db.apply_migrations("migrations".to_string()).await;
    db
}

pub async fn delete_test_db(db: Database) {
    drop(db);
    fs::remove_file(TEST_DB_PATH).unwrap();
}
