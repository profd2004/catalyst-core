use std::{fs, path::Path, process::Command};

use crate::common::ideascale_mock::ideascale::IdeascaleSettings;
pub struct IdeascaleImporterCommand {
    pub command: Command,
}

impl IdeascaleImporterCommand {
    pub fn new() -> Self {
        Self {
            command: Command::new("poetry"),
        }
    }

    pub fn api_token(mut self, api_token: String) -> Self {
        self.command.arg("--api-token").arg(api_token);
        self
    }

    pub fn event_db_url(mut self, event_db_url: String) -> Self {
        self.command.arg("--database-url").arg(event_db_url);
        self
    }

    pub fn ideascale_api_url(mut self, ideascale_api_url: String) -> Self {
        self.command
            .arg("--ideascale-api-url")
            .arg(ideascale_api_url);
        self
    }

    pub fn event_id(mut self, event_id: i32) -> Self {
        self.command.arg("--event-id").arg(event_id.to_string());
        self
    }

    pub fn campaign_group_id(mut self, campaign_group_id: i32) -> Self {
        self.command
            .arg("--campaign-group-id")
            .arg(campaign_group_id.to_string());
        self
    }

    pub fn stage_id(mut self, stage_id: i32) -> Self {
        self.command.arg("--stage-id").arg(stage_id.to_string());
        self
    }

    pub fn import_all(mut self) -> Self {
        let ideascale_importer_path = fs::canonicalize("../utilities/ideascale-importer")
            .expect("Ideascale path not correct");
        self.command.current_dir(&ideascale_importer_path).args([
            "run",
            "ideascale-importer",
            "ideascale",
            "import-all",
        ]);
        self
    }
}
