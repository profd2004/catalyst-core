use super::ideascale_importer_command::IdeascaleImporterCommand;
use crate::common::dbsync_mock::DbSyncMock;
use crate::common::event_db_mock::EventDbMock;

const EVENT_ID: i32 = 1;
const FUND10_SANDBOX_ID: i32 = 87;
const SUBMIT_PROPOSALS_STAGE_ID: i32 = 1;
const FINALIZE_STAGE_ID: i32 = 2;
const COMMUNITY_REVIEW_STAGE_ID: i32 = 3;
const MODERATION_STAGE_ID: i32 = 4;
const ARCHIVE_STAGE_ID: i32 = 5;
const BAD_STAGE_ID: i32 = 999;
const BAD_PROPOSAL_GROUP_ID: i32 = 999;
const BAD_EVENT_ID: i32 = 999;

const FUND10_STAGES: [i32; 5] = [
    SUBMIT_PROPOSALS_STAGE_ID,
    FINALIZE_STAGE_ID,
    COMMUNITY_REVIEW_STAGE_ID,
    MODERATION_STAGE_ID,
    ARCHIVE_STAGE_ID,
];

///Import proposals from one stage at the time, move some proposal to the next stage
#[tokio::test]
async fn import_all_proposals_stage_flow() {
    //setup event database
    let event_db = EventDbMock::new_with_random_name().await;
    event_db.insert_event(EVENT_ID).await;
    let base_command = IdeascaleImporterCommand::default()
        .event_db_url(event_db.get_settings().connection_string());
    //Import proposals one stage at the time, and move some proposal to the next stage
    for stage_id in FUND10_STAGES {
        let mut command = base_command
            .clone()
            .event_id(EVENT_ID)
            .campaign_group_id(FUND10_SANDBOX_ID)
            .stage_id(stage_id)
            .import_all();

        let output = command
            .output()
            .expect(format!("failed to execute command {:?}", command).as_str());

        assert!(
            output.status.success(),
            "Ideascale command {:?}\n failed with {}",
            command,
            output.status.to_string()
        );

        println!(
            "result: {}",
            String::from_utf8(output.stdout).expect("Invalid UTF-8 sequence")
        );

        //TODO CHECK OBJECTIVE and PROPOSALS FOR THAT STAGE ARE IN THE DB
        event_db.get_event(EVENT_ID).await;

        //TODO move proposals to the next stage
    }
}

///Call the tool using non existing parameters
#[tokio::test]
async fn import_all_bad_params() {
    //setup event database
    let event_db = EventDbMock::new_with_random_name().await;
    event_db.insert_event(EVENT_ID).await;
    let test_vec = [
        (BAD_EVENT_ID, FUND10_SANDBOX_ID, FINALIZE_STAGE_ID),
        (EVENT_ID, FUND10_SANDBOX_ID, BAD_STAGE_ID),
        (EVENT_ID, BAD_PROPOSAL_GROUP_ID, FINALIZE_STAGE_ID),
    ];

    let base_command = IdeascaleImporterCommand::default()
        .event_db_url(event_db.get_settings().connection_string());

    for (event_id, proposal_group_id, stage_id) in test_vec {
        let mut command = base_command
            .clone()
            .event_id(event_id)
            .campaign_group_id(proposal_group_id)
            .stage_id(stage_id)
            .import_all();

        let output = command
            .output()
            .expect(format!("failed to execute command {:?}", command).as_str());

        assert!(
            output.status.success(),
            "Ideascale command: {:?}\n failed with {}",
            command,
            output.status.to_string()
        );

        println!(
            "result: {}",
            String::from_utf8(output.stdout).expect("Invalid UTF-8 sequence")
        );
        //TODO CHECK DB IS EMPTY
    }
}

///Import proposals, then edit one and che the pool picks it up
#[tokio::test]
//TODO
async fn import_all_edit_proposal() {
    //setup event database
    let event_id = 1;
    let event_db = EventDbMock::new_with_random_name().await;
    event_db.insert_event(event_id).await;
}

#[tokio::test]
async fn import_snapshot_happy_path() {
    let event_id = 1;
    let event_db = EventDbMock::new_with_random_name().await;
    event_db.insert_event(event_id).await;
    let _dbsync = DbSyncMock::new_with_random_name().await;
}
