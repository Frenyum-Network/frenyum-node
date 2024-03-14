use crate::{block_header::BlockHeader, transaction::SignedTransaction, Address};
use std::sync::Arc;

pub struct Block
{
    header: BlockHeader,
    transaction: Vec<Arc<SignedTransaction>>,
}

impl Block
{
    pub fn new(
        header: BlockHeader,
        transaction: Vec<Arc<SignedTransaction>>,
    ) -> Self {
        Block {
            header,
            transaction,
        }
    }

    pub fn header(&self) -> &BlockHeader
    {
        &self.header
    }

    pub fn transaction(&self) -> &Vec<Arc<SignedTransaction>>
    {
        &self.transaction
    }
}
