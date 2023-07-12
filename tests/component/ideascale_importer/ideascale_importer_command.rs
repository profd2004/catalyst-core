use dotenvy::dotenv;
use std::{env, fs, path::PathBuf, process::Command};

use crate::common::ideascale_mock::ideascale;
use crate::common::ideascale_mock::ideascale::IdeascaleSettings;
#[derive(Clone)]
pub struct IdeascaleImporterCommand {
    pub path: PathBuf,
    pub ideascale_settings: IdeascaleSettings,
    pub event_db_url: String,
    pub event_id: i32,
    pub campaign_group_id: i32,
    pub stage_id: i32,
}

//TODO add default db and importer path
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
    pub fn new(ideascale_settings: IdeascaleSettings, event_db_url: String, path: PathBuf) -> Self {
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

#[derive(Clone)]
pub struct IdeascaleImporterSnapshotCommand {
    pub path: PathBuf,
    pub event_db_url: String,
    pub event_id: i32,
    pub catalyst_toolbox_path: Option<PathBuf>,
    pub raw_snapshot_file: Option<PathBuf>,
    pub dreps_file: Option<PathBuf>,
}

//TODO add default db and importer path
impl Default for IdeascaleImporterSnapshotCommand {
    fn default() -> Self {
        dotenv().ok();
        let path = fs::canonicalize(
            env::var("IDEASCALE_IMPORTER_PATH").expect("Ideascale path env variable not found"),
        )
        .expect("Ideascale path not correct");
        let event_db_url = env::var("DATABASE_URL").expect("Event db url env variable not found");
        Self {
            path,
            event_db_url,
            event_id: 1,
            catalyst_toolbox_path: None,
            raw_snapshot_file: None,
            dreps_file: None,
        }
    }
}

impl IdeascaleImporterSnapshotCommand {
    pub fn new(event_db_url: String, path: PathBuf) -> Self {
        Self {
            path,
            event_db_url,
            event_id: 1,
            catalyst_toolbox_path: None,
            raw_snapshot_file: None,
            dreps_file: None,
        }
    }

    pub fn event_db_url(mut self, event_db_url: String) -> Self {
        self.event_db_url = event_db_url;
        self
    }

    pub fn event_id(mut self, event_id: i32) -> Self {
        self.event_id = event_id;
        self
    }

    pub fn path(mut self, path: PathBuf) -> Self {
        self.path = path;
        self
    }

    pub fn catalyst_toolbox_path(mut self, catalyst_toolbox_path: PathBuf) -> Self {
        self.catalyst_toolbox_path = Some(catalyst_toolbox_path);
        self
    }

    pub fn raw_snapshot_file(mut self, raw_snapshot_file: PathBuf) -> Self {
        self.raw_snapshot_file = Some(raw_snapshot_file);
        self
    }

    pub fn dreps_file(mut self, dreps_file: PathBuf) -> Self {
        self.dreps_file = Some(dreps_file);
        self
    }

    pub fn snapshot_import(self) -> Command {
        let mut command = Command::new("poetry");
        command.current_dir(self.path);
        command.args([
            "run",
            "ideascale-importer",
            "snapshot",
            "import",
            "--eventdb-url",
            &self.event_db_url,
            "--event-id",
            &self.event_id.to_string(),
        ]);
        if self.catalyst_toolbox_path.is_some() {
            command.args([
                "--catalyst-toolbox-path",
                self.catalyst_toolbox_path.unwrap().to_str().unwrap(),
            ]);
        }
        if self.dreps_file.is_some() {
            command.args(["--dreps-file", self.dreps_file.unwrap().to_str().unwrap()]);
        }

        if self.raw_snapshot_file.is_some() {
            command.args([
                "--raw-snapshot-file",
                self.raw_snapshot_file.unwrap().to_str().unwrap(),
            ]);
        }

        command
    }
}
