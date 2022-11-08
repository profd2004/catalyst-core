use super::logic;
use crate::v0::endpoints::votes::VoteCasterAndVoteplanId;
use crate::v0::{context::SharedContext, result::HandlerResult};
use warp::{Rejection, Reply};

pub async fn get_vote_by_caster_and_voteplan_id(
    body: VoteCasterAndVoteplanId,
    context: SharedContext,
) -> Result<impl Reply, Rejection> {
    Ok(HandlerResult(
        logic::get_vote_by_caster_and_voteplan_id(body.caster, body.vote_plan_id, context).await,
    ))
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::db::{
        migrations as db_testing,
        models::vote::{test as votes_testing, *},
    };
    use crate::v0::context::test::new_db_test_shared_context;
    use crate::v0::endpoints::votes::VoteCasterAndVoteplanId;
    use warp::Filter;

    #[tokio::test]
    async fn get_vote_by_voteplan_id_and_caster() {
        // build context
        let shared_context = new_db_test_shared_context();
        let filter_context = shared_context.clone();
        let with_context = warp::any().map(move || filter_context.clone());

        // initialize db
        let pool = &shared_context.read().await.db_connection_pool;
        db_testing::initialize_db_with_migration(&pool.get().unwrap());
        let vote: Vote = votes_testing::get_test_vote();

        votes_testing::populate_db_with_vote(&vote, pool);

        // build filter
        let filter = warp::any()
            .and(warp::post())
            .and(warp::body::json())
            .and(with_context)
            .and_then(get_vote_by_caster_and_voteplan_id);

        let request = VoteCasterAndVoteplanId {
            vote_plan_id: vote.voteplan_id.clone(),
            caster: vote.caster.clone(),
        };

        let result = warp::test::request()
            .method("POST")
            .json(&request)
            .reply(&filter)
            .await;

        assert_eq!(result.status(), warp::http::StatusCode::OK);
        let result_votes: Vec<Vote> =
            serde_json::from_str(&String::from_utf8(result.body().to_vec()).unwrap()).unwrap();
        assert_eq!(vec![vote], result_votes);
    }
}
