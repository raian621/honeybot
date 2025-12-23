use crate::datastore::database::{Database, DatabaseOptions};

const TEST_DATABASE_PATH: &str = "/tmp/honeybot-test.db";

pub async fn get_test_db() -> Database {
    Database::new(&DatabaseOptions {
        filename: TEST_DATABASE_PATH.to_string(),
        migrations_path: "migrations".to_string(),
    })
    .await
}
