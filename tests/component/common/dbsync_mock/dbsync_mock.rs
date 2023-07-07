use crate::common::db_mock::DbMock;
use crate::common::db_mock::DatabaseSettings;
pub struct DbSyncSettings{
    db_settings_instance : DatabaseSettings,
}

impl Default for DbSyncSettings {
    fn default() -> Self {
       let mut db_settings_instance= DatabaseSettings::default();
      db_settings_instance.migrations_path= "component/common/dbsync_mock/migrations".to_string();
      db_settings_instance.name = "dbsync".to_string();
        Self {
            db_settings_instance
        }
    }
}

pub struct DbSyncMock {
    mock_db_instance: DbMock,
}

impl DbSyncMock {
    pub async fn new() -> Self {
        let settings = DbSyncSettings::default();
        DbSyncMock {
            mock_db_instance: DbMock::new_with_random_name(settings.db_settings_instance, Some("dbsync_test".to_string())).await,
        }
    }
}
