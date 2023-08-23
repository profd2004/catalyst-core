use crate::{
    service::{axum_handle_result, Error},
    state::State,
    types::SerdeType,
};
use axum::{extract::Path, routing::get, Router};
use event_db::types::{ballot::ProposalBallot, event::EventId, objective::ObjectiveId};
use std::sync::Arc;

pub fn ballots(state: Arc<State>) -> Router {
    Router::new().route(
        "/ballots",
        get(move |path| async { axum_handle_result(ballots_exec(path, state).await) }),
    )
}

async fn ballots_exec(
    Path((SerdeType(event), SerdeType(objective))): Path<(
        SerdeType<EventId>,
        SerdeType<ObjectiveId>,
    )>,
    state: Arc<State>,
) -> Result<Vec<SerdeType<ProposalBallot>>, Error> {
    tracing::debug!(
        "ballots_query, event: {0}, objective: {1}",
        event.0,
        objective.0,
    );

    let ballot = state
        .event_db
        .get_objective_ballots(event, objective)
        .await?
        .into_iter()
        .map(SerdeType)
        .collect();
    Ok(ballot)
}

/// Need to setup and run a test event db instance
/// To do it you can use the following commands:
/// Prepare docker images
/// ```
/// earthly ./containers/event-db-migrations+docker --data=test
/// ```
/// Run event-db container
/// ```
/// docker-compose -f src/event-db/docker-compose.yml up migrations
/// ```
/// Also need establish `EVENT_DB_URL` env variable with the following value
/// ```
/// EVENT_DB_URL="postgres://catalyst-event-dev:CHANGE_ME@localhost/CatalystEventDev"
/// ```
/// https://github.com/input-output-hk/catalyst-core/tree/main/src/event-db/Readme.md
#[cfg(test)]
mod tests {
    use super::*;
    use crate::service::{axum_app, tests::body_data_json_check};
    use axum::{
        body::{Body, HttpBody},
        http::{Request, StatusCode},
    };
    use tower::ServiceExt;

    #[tokio::test]
    async fn ballots_test() {
        let state = Arc::new(State::new(None).await.unwrap());
        let app = axum_app(state);

        let request = Request::builder()
            .uri(format!("/api/v1/event/{0}/objective/{1}/ballots", 1, 1))
            .body(Body::empty())
            .unwrap();
        let response = app.clone().oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        assert!(body_data_json_check(
            response.into_body().data().await.unwrap().unwrap().to_vec(),
            serde_json::json!(
                [
                    {
                        "proposal_id": 10,
                        "ballot": {
                            "choices": ["yes", "no"],
                            "voteplans": [
                                {
                                    "chain_proposal_index": 10,
                                    "group": "direct",
                                    "ballot_type": "public",
                                    "chain_voteplan_id": "1",
                                },
                                {
                                    "chain_proposal_index": 12,
                                    "group": "rep",
                                    "ballot_type": "public",
                                    "chain_voteplan_id": "2",
                                }
                            ]
                        }
                    },
                    {
                        "proposal_id": 20,
                        "ballot": {
                            "choices": ["yes", "no"],
                            "voteplans": [
                                {
                                    "chain_proposal_index": 11,
                                    "group": "direct",
                                    "ballot_type": "public",
                                    "chain_voteplan_id": "1",
                                },
                                {
                                    "chain_proposal_index": 13,
                                    "group": "rep",
                                    "ballot_type": "public",
                                    "chain_voteplan_id": "2",
                                }
                            ]
                        }
                    },
                    {
                        "proposal_id": 30,
                        "ballot": {
                            "choices": ["yes", "no"],
                            "voteplans": []
                        }
                    }
                ]
            )
        ));
    }
}
