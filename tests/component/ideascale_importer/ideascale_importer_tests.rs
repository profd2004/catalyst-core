use super::ideascale_importer_command::IdeascaleImporterCommand;
use crate::common::event_db_mock::EventDbMock;
static FUND10_SANDBOX_ID: i32 = 87;
static SUBMIT_PROPOSALS_STAGE_ID: i32 = 1;
static FINALIZE_STAGE_ID: i32 = 2;

#[tokio::test]
async fn import_all_happy_path() {
    //setup event database
    let event_id = 2;
    let campaign_group_id = FUND10_SANDBOX_ID;
    let stage_id = FINALIZE_STAGE_ID;
    let mut event_db = EventDbMock::connect_to_default().await;
    event_db.persist();

    event_db.insert_event(event_id).await;

    let output = IdeascaleImporterCommand::default()
        .event_db_url(event_db.settings.connection_string())
        .event_id(event_id)
        .campaign_group_id(campaign_group_id)
        .stage_id(stage_id)
        .import_all()
        .output()
        .unwrap_or_else(|e| panic!("failed to execute process: {}", e));

    //TODO CHECK DATA ARE IN THE DB
    event_db.get_event(event_id).await;

    assert!(
        output.status.success(),
        "Ideascale command failed with {}",
        output.status.to_string()
    );

    let s = match String::from_utf8(output.stdout) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };

    println!("result: {}", s);
}

#[tokio::test]
async fn import_all_bad_params() {
    //setup event database
    let event_db = EventDbMock::new_with_random_name().await;
    let event_id = 2;
    let campaign_group_id = 87;
    let stage_id = 1;
    event_db.insert_event(event_id).await;

    let ideascale_command =
        IdeascaleImporterCommand::default().event_db_url(event_db.settings.connection_string());

    //bad event db
    let output = ideascale_command
        .clone()
        .event_id(999)
        .campaign_group_id(campaign_group_id)
        .stage_id(stage_id)
        .import_all()
        .output()
        .unwrap_or_else(|e| panic!("failed to execute process: {}", e));

    //TODO CHECK DB IS EMPTY

    assert!(
        output.status.success(),
        "Ideascale command failed with {}",
        output.status.to_string()
    );

    let s = match String::from_utf8(output.stdout) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };
    println!("result: {}", s);

    //bad campaign group id
    let output = ideascale_command
        .clone()
        .event_id(event_id)
        .campaign_group_id(999)
        .stage_id(stage_id)
        .import_all()
        .output()
        .unwrap_or_else(|e| panic!("failed to execute process: {}", e));

    assert!(
        output.status.success(),
        "Ideascale command failed with {}",
        output.status.to_string()
    );

    let s = match String::from_utf8(output.stdout) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };

    println!("result: {}", s);

    //bad stage id
    let output = ideascale_command
        .clone()
        .event_id(event_id)
        .campaign_group_id(campaign_group_id)
        .stage_id(999)
        .import_all()
        .output()
        .unwrap_or_else(|e| panic!("failed to execute process: {}", e));

    assert!(
        output.status.success(),
        "Ideascale command failed with {}",
        output.status.to_string()
    );

    let s = match String::from_utf8(output.stdout) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };

    println!("result: {}", s);
}

#[tokio::test]

async fn import_all_same_event_id() {
    //setup event database
    let event_id = 1;
    let campaign_group_id = 87;
    let stage_id = 2;
    let event_db = EventDbMock::new_with_random_name().await;

    event_db.insert_event(event_id).await;

    let command =
        IdeascaleImporterCommand::default().event_db_url(event_db.settings.connection_string());

    let output = command
        .clone()
        .event_id(event_id)
        .campaign_group_id(campaign_group_id)
        .stage_id(stage_id)
        .import_all()
        .output()
        .unwrap_or_else(|e| panic!("failed to execute process: {}", e));

    //TODO CHECK DATA ARE IN THE DB
    event_db.get_event(event_id).await;

    assert!(
        output.status.success(),
        "Ideascale command failed with {}",
        output.status.to_string()
    );

    let s = match String::from_utf8(output.stdout) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };

    println!("result: {}", s);

    let output = command
        .clone()
        .event_id(event_id)
        .campaign_group_id(campaign_group_id)
        .stage_id(stage_id)
        .import_all()
        .output()
        .unwrap_or_else(|e| panic!("failed to execute process: {}", e));

    //TODO CHECK DATA ARE IN THE DB
    event_db.get_event(event_id).await;

    assert!(
        output.status.success(),
        "Ideascale command failed with {}",
        output.status.to_string()
    );

    let s = match String::from_utf8(output.stdout) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };

    println!("result: {}", s);
}

#[tokio::test]
async fn import_snapshot_happy_path() {
    let event_id=1;
    let event_db = EventDbMock::new_with_random_name().await;
    event_db.insert_event(event_id).await;

}