use crate::common::db_event_mock::db_event;
use crate::common::ideascale_mock::ideascale;
use std::fs;
use std::process::Command;

#[tokio::test]
async fn import_all() {
    //Setup event database
    let event_id = 1;
    let mut db_event_config = db_event::get_configuration_with_random_db_name()
        .expect("Failed to read db event configuration");
    let connection_string = db_event_config.connection_string();
    let db_event_connection = db_event::configure_new_database(&db_event_config).await;
    //insert a empty event to pass to ideascale importer
    db_event::insert_event(db_event_connection, event_id).await;

    let ideascale_config =
        ideascale::get_configuration().expect("Failed to read ideascale configuration");
    let ideascale_importer_path =
        fs::canonicalize("../utilities/ideascale-importer").expect("Ideascale path not correct");

    let campaign_group_id = "87";
    let stage_id = "138";

    Command::new("poetry")
        .current_dir(&ideascale_importer_path)
        .arg("install")
        .output()
        .unwrap_or_else(|e| panic!("failed to execute process: {}", e));

    let output = Command::new("poetry")
        .current_dir(&ideascale_importer_path)
        .args([
            "run",
            "ideascale-importer",
            "ideascale",
            "import-all",
            "--api-token",
            &ideascale_config.api_token,
            "--database-url",
            &connection_string,
            "--ideascale-api-url",
            &ideascale_config.api_url,
            "--event-id",
            &event_id.to_string(),
            "--campaign-group-id",
            campaign_group_id,
            "--stage-id",
            stage_id,
        ])
        .output()
        .unwrap_or_else(|e| panic!("failed to execute process: {}", e));

    let s = match String::from_utf8(output.stdout) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };

    println!("result: {}", s);
}
