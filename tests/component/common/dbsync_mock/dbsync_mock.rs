use crate::common::db_mock::DbMock;

pub struct DbSyncMock {
    mock_db_instance: DbMock,
}

impl DbSyncMock {
    pub async fn new() -> Self {
        DbSyncMock {
            mock_db_instance: DbMock::new_with_random_name(Some("dbsync".to_string())).await,
        }
    }
}
