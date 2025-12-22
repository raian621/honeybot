use crate::datastore::database::Database;

const TEST_DATABASE_PATH: &str = "/tmp/honeybot-test.db";

pub async fn get_test_db() -> Database {
    let db: Database = Database::new(TEST_DATABASE_PATH).await;
    db.apply_migrations("migrations".to_string()).await;
    db
}
