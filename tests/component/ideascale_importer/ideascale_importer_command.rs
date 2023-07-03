use dotenvy::dotenv;
use std::{env, fs, path::PathBuf, process::Command};

use crate::common::ideascale_mock::ideascale;
use crate::common::ideascale_mock::ideascale::IdeascaleSettings;

pub struct IdeascaleImporterCommand {
    pub path: PathBuf,
    pub ideascale_settings: IdeascaleSettings,
    pub event_db_url: String,
    pub event_id: i32,
    pub campaign_group_id: i32,
    pub stage_id: i32,
}
impl Default for IdeascaleImporterCommand {
    fn default() -> Self {
        dotenv().ok();
        let path = fs::canonicalize(
            env::var("IDEASCALE_IMPORTER_PATH").expect("Ideascale path env variable not found"),
        )
        .expect("Ideascale path not correct");
        let event_db_url = env::var("DATABASE_URL").expect("Event db url env variable not found");
        let ideascale_settings =
            ideascale::get_configuration().expect("Failed to load ideascale settings");
        Self {
            path,
            ideascale_settings,
            event_db_url,
            event_id: 1,
            campaign_group_id: 1,
            stage_id: 1,
        }
    }
}

impl IdeascaleImporterCommand {
    pub fn new(ideascale_settings: IdeascaleSettings) -> Self {
        dotenv().ok();
        let path = fs::canonicalize(
            env::var("IDEASCALE_IMPORTER_PATH").expect("Ideascale path env variable not found"),
        )
        .expect("Ideascale path not correct");
        let event_db_url = env::var("DATABASE_URL").expect("Event db url env variable not found");
        Self {
            path,
            ideascale_settings,
            event_db_url,
            event_id: 1,
            campaign_group_id: 1,
            stage_id: 1,
        }
    }

    pub fn ideascale_settings(mut self, ideascale_settings: IdeascaleSettings) -> Self {
        self.ideascale_settings = ideascale_settings;
        self
    }

    pub fn event_db_url(mut self, event_db_url: String) -> Self {
        self.event_db_url = event_db_url;
        self
    }

    pub fn event_id(mut self, event_id: i32) -> Self {
        self.event_id = event_id;
        self
    }

    pub fn campaign_group_id(mut self, campaign_group_id: i32) -> Self {
        self.campaign_group_id = campaign_group_id;
        self
    }

    pub fn stage_id(mut self, stage_id: i32) -> Self {
        self.stage_id = stage_id;
        self
    }

    pub fn path(mut self, path: PathBuf) -> Self {
        self.path = path;
        self
    }

    pub fn import_all(self) -> Command {
        let mut command = Command::new("poetry");
        command.current_dir(self.path);
        command.args([
            "run",
            "ideascale-importer",
            "ideascale",
            "import-all",
            "--api-token",
            &self.ideascale_settings.api_token,
            "--database-url",
            &self.event_db_url,
            "--ideascale-api-url",
            &self.ideascale_settings.api_url,
            "--event-id",
            &self.event_id.to_string(),
            "--campaign-group-id",
            &self.campaign_group_id.to_string(),
            "--stage-id",
            &self.stage_id.to_string(),
        ]);

        command
    }
}
