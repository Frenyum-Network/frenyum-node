use crate::{block_header::BlockHeader, transaction::SignedTransaction, Address};
use std::sync::Arc;

pub struct Block
{
    header: BlockHeader,
    body: BlockBody,
}

impl Block
{
    pub fn new(header: BlockHeader, body: BlockBody) -> Self {
        Block {
            header,
            body,
        }
    }

    pub fn header(&self) -> &BlockHeader
    {
        &self.header
    }

    pub fn body(&self) -> &BlockBody
    {
        &self.body
    }
}

pub struct BlockBody
{
    transaction: Vec<Arc<SignedTransaction>>,
    gas_used: Gas,
    gas_limit: Gas,
}

