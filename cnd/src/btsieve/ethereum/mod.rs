mod transaction_pattern;
mod web3_connector;

pub use self::{
    transaction_pattern::{Event, Topic, TransactionPattern},
    web3_connector::Web3Connector,
};
use crate::{
    btsieve::{BlockByHash, LatestBlock, MatchingTransactions, ReceiptByHash},
    ethereum::TransactionAndReceipt,
};
use futures_core::{compat::Future01CompatExt, TryFutureExt};
use std::ops::Add;
use tokio::{
    prelude::{stream, Stream},
    timer::Delay,
};

impl<C> MatchingTransactions<TransactionPattern> for C
where
    C: LatestBlock<Block = Option<crate::ethereum::Block<crate::ethereum::Transaction>>>
        + BlockByHash<Block = Option<crate::ethereum::Block<crate::ethereum::Transaction>>>
        + ReceiptByHash<
            Receipt = Option<crate::ethereum::TransactionReceipt>,
            TransactionHash = crate::ethereum::H256,
        > + Clone,
{
    type Transaction = TransactionAndReceipt;

    fn matching_transactions(
        &self,
        pattern: TransactionPattern,
        _timestamp: Option<u32>,
    ) -> Box<dyn Stream<Item = Self::Transaction, Error = ()> + Send> {
        let matching_transaction = Box::pin(matching_transaction(self.clone(), pattern)).compat();
        Box::new(stream::futures_unordered(vec![matching_transaction]))
    }
}

async fn matching_transaction<C>(
    mut blockchain_connector: C,
    pattern: TransactionPattern,
) -> Result<TransactionAndReceipt, ()>
where
    C: LatestBlock<Block = Option<crate::ethereum::Block<crate::ethereum::Transaction>>>
        + BlockByHash<Block = Option<crate::ethereum::Block<crate::ethereum::Transaction>>>
        + ReceiptByHash<
            Receipt = Option<crate::ethereum::TransactionReceipt>,
            TransactionHash = crate::ethereum::H256,
        > + Clone,
{
    loop {
        // Delay so that we don't overload the CPU in the event that
        // latest_block() and block_by_hash() resolve quickly.
        Delay::new(std::time::Instant::now().add(std::time::Duration::from_secs(1)))
            .compat()
            .await
            .unwrap_or_else(|e| log::warn!("Failed to wait for delay: {:?}", e));

        let latest_block = match blockchain_connector.latest_block().compat().await {
            Ok(Some(block)) => block,
            Ok(None) => {
                log::warn!("Could not get latest block");
                continue;
            }
            Err(e) => {
                log::warn!("Could not get latest block: {:?}", e);
                continue;
            }
        };

        if pattern.can_skip_block(&latest_block) {
            continue;
        }

        for transaction in latest_block.transactions.iter() {
            let result = blockchain_connector
                .receipt_by_hash(transaction.hash)
                .compat()
                .await;

            let receipt = match result {
                Ok(Some(receipt)) => receipt,
                Ok(None) => {
                    log::warn!("Could not get transaction receipt");
                    continue;
                }
                Err(e) => {
                    log::warn!(
                        "Could not retrieve transaction receipt for {}: {:?}",
                        transaction.hash,
                        e
                    );
                    continue;
                }
            };

            if pattern.matches(transaction, &receipt) {
                return Ok(TransactionAndReceipt {
                    transaction: transaction.clone(),
                    receipt,
                });
            };
        }
    }
}
