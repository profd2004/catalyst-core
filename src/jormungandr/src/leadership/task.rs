use crate::{
    blockcfg::{BlockBuilder, BlockDate, ChainLength, HeaderHash},
    blockchain::Tip,
    intercom::BlockMsg,
    leadership::{LeaderSchedule, Leadership},
    secure::enclave::{Enclave, LeaderId},
    transaction::TPoolR,
    utils::async_msg::MessageBox,
};
use chain_core::property::ChainLength as _;
use chain_time::timeframe::TimeFrame;
use slog::Logger;
use std::sync::Arc;
use tokio::{prelude::*, sync::watch};

custom_error! {pub HandleLeadershipError
    Schedule { source: tokio::timer::Error } = "Error in the leadership schedule",
}

custom_error! {pub TaskError
    LeadershipReceiver { extra: String } = "Cannot continue the leader task: {extra}",
    LeadershipHandle { source: HandleLeadershipError } = "Error while handling an epoch's leader schedule",
}

#[derive(Clone)]
pub struct TaskParameters {
    pub leadership: Arc<Leadership>,
    pub time_frame: TimeFrame,
}

pub struct Task {
    logger: Logger,
    leader: LeaderId,
    enclave: Enclave,
    blockchain_tip: Tip,
    epoch_receiver: watch::Receiver<Option<TaskParameters>>,
    transaction_pool: TPoolR,
    block_message: MessageBox<BlockMsg>,
}

impl Task {
    #[inline]
    pub fn new(
        logger: Logger,
        leader: LeaderId,
        enclave: Enclave,
        blockchain_tip: Tip,
        transaction_pool: TPoolR,
        epoch_receiver: watch::Receiver<Option<TaskParameters>>,
        block_message: MessageBox<BlockMsg>,
    ) -> Self {
        let logger = Logger::root(
            logger,
            o!(
                ::log::KEY_TASK => "Leader Task",
                // TODO: add some general context information here (leader alias?)
            ),
        );

        Task {
            logger,
            leader: leader,
            enclave: enclave,
            blockchain_tip,
            transaction_pool,
            epoch_receiver,
            block_message,
        }
    }

    pub fn start(self) -> impl Future<Item = (), Error = ()> {
        let handle_logger = self.logger.clone();
        let crit_logger = self.logger;
        let leader = self.leader;
        let enclave = self.enclave;
        let blockchain_tip = self.blockchain_tip;
        let transaction_pool = self.transaction_pool;
        let block_message = self.block_message;

        self.epoch_receiver
            .map_err(|error| TaskError::LeadershipReceiver {
                extra: format!("{}", error),
            })
            // filter_map so we don't have to do the pattern match on `Option::Nothing`.
            .filter_map(|task_parameters| task_parameters)
            .for_each(move |task_parameters| {
                handle_leadership(
                    block_message.clone(),
                    leader,
                    enclave.clone(),
                    handle_logger.clone(),
                    blockchain_tip.clone(),
                    transaction_pool.clone(),
                    task_parameters,
                )
                .map_err(|error| {
                    TaskError::LeadershipHandle { source: error }
                })
            })
            .map_err(move |error| {
                crit!(crit_logger, "critical error in the Leader task" ; "reason" => error.to_string())
            })
    }
}

/// function that will run for the length of the Epoch associated
/// to the given leadership
///
fn handle_leadership(
    mut block_message: MessageBox<BlockMsg>,
    leader_id: LeaderId,
    enclave: Enclave,
    logger: Logger,
    blockchain_tip: Tip,
    transaction_pool: TPoolR,
    task_parameters: TaskParameters,
) -> impl Future<Item = (), Error = HandleLeadershipError> {
    let schedule = LeaderSchedule::new(logger.clone(), &leader_id, &enclave, &task_parameters);

    schedule
        .map_err(|err| HandleLeadershipError::Schedule { source: err })
        .for_each(move |scheduled_event| {
            let scheduled_event = scheduled_event.into_inner();

            info!(logger, "Leader scheduled event" ;
                "scheduled at_time" => format!("{:?}", scheduled_event.expected_time),
                "scheduled_at_date" => format!("{}", scheduled_event.leader_output.date),
            );

            let block = prepare_block(
                &transaction_pool,
                scheduled_event.leader_output.date,
                blockchain_tip.chain_length().unwrap().next(),
                blockchain_tip.hash().unwrap(),
            );

            let block = enclave.create_block(block, scheduled_event.leader_output);

            block_message
                .try_send(BlockMsg::LeadershipBlock(block))
                .unwrap();

            future::ok(())
        })
}

fn prepare_block(
    transaction_pool: &TPoolR,
    date: BlockDate,
    chain_length: ChainLength,
    parent_id: HeaderHash,
) -> BlockBuilder {
    let mut bb = BlockBuilder::new();

    bb.date(date).parent(parent_id).chain_length(chain_length);
    let messages = transaction_pool.write().unwrap().collect(250 /* TODO!! */);
    bb.messages(messages);

    bb
}
