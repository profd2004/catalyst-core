use std::fs;
use std::process::Command;

#[test]
fn ideascale_importer_import_all() {
    let ideascale_importer_path =
        fs::canonicalize("../utilities/ideascale-importer").expect("Ideascale path not correct");
    let api_token = "4dc0a585-ad0a-476f-8dd9-56667edb9353";
    let db_url = "postgres://postgres:password@localhost:5432/CatalystEventDev";
    let ideascale_api_url = "https://cat-test.ideascaleapp.com";
    let event_id = "3";
    let campaign_group_id = "33";
    let stage_id = "44";

    let containers_path =
        fs::canonicalize("../containers").expect("Containers path not correct");

    Command::new("earthly")
        .current_dir(&containers_path)
        .args(["event-db-migrations+docker", "--data=test"])
        .output()
        .unwrap_or_else(|e| panic!("failed to execute process: {}", e));

    let event_db_path =
        fs::canonicalize("../src/event-db").expect("Event db path not correct");

    Command::new("docker-compose")
        .current_dir(&event_db_path)
        .args(["-f", "docker-compose.yml", "up", "migrations"])
        .output()
        .unwrap_or_else(|e| panic!("failed to execute process: {}", e));

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
            db_url,
            "--ideascale-api-url",
            ideascale_api_url,
            "--event-id",
            event_id,
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
