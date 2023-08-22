use crate::common::dbsync_mock::DbSyncMock;
use crate::common::event_db_mock::EventDbMock;

#[tokio::test]
async fn import_snapshot_happy_path() {
    let event_id = 1;
    let event_db = EventDbMock::new_with_random_name().await;
    event_db.insert_event(event_id).await;
    let _dbsync = DbSyncMock::new_with_random_name().await;
}