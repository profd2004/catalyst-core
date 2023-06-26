use crate::common::db_event::{self, configure_new_database};
use std::fs;
use std::process::Command;

#[tokio::test]
async fn import_all() {
    //Setup event database
    let event_id = 1;

    let mut configuration =
        db_event::get_configuration().expect("Failed to read db event configuration");
    //randomize db name
    configuration.database_name = "event_db_test1".to_string();

    let connection = configure_new_database(&configuration).await;

    //insert a empty event to pass to ideascale importer
    sqlx::query!(r#"INSERT INTO event (row_id) VALUES($1)"#, event_id)
        .execute(&connection)
        .await
        .expect("Failed to insert event id into event database");

    let ideascale_importer_path =
        fs::canonicalize("../utilities/ideascale-importer").expect("Ideascale path not correct");
    let api_token = "4dc0a585-ad0a-476f-8dd9-56667edb9353";
    let ideascale_api_url = "https://cat-test.ideascaleapp.com";
    let campaign_group_id = "33";
    let stage_id = "44";

    let connection_string = configuration.connection_string();

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
            api_token,
            "--database-url",
            &connection_string,
            "--ideascale-api-url",
            ideascale_api_url,
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
