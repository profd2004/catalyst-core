use super::ideascale_importer_command::IdeascaleImporterCommand;
use crate::common::event_db_mock::EventDbMock;

#[tokio::test]
async fn import_all() {
    //setup event database
    let event_db = EventDbMock::new(None).await;
    let event_id = 2;
    event_db.insert_event(event_id).await;

    let output = IdeascaleImporterCommand::default()
        .event_db_url(event_db.settings.connection_string())
        .event_id(event_id)
        .campaign_group_id(87)
        .stage_id(1)
        .import_all()
        .output()
        .unwrap_or_else(|e| panic!("failed to execute process: {}", e));

    let s = match String::from_utf8(output.stdout) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };

    println!("result: {}", s);
}

#[tokio::test]
async fn import_all_missing_params() {
    //setup event database
    let event_db = EventDbMock::new(None).await;
    let event_id = 2;
    event_db.insert_event(event_id).await;

    let output = IdeascaleImporterCommand::default()
        .event_db_url(event_db.settings.connection_string())
        .event_id(3)
        .campaign_group_id(87)
        .stage_id(1)
        .import_all()
        .output()
        .unwrap_or_else(|e| panic!("failed to execute process: {}", e));

    let s = match String::from_utf8(output.stdout) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };

    println!("result: {}", s);
}
