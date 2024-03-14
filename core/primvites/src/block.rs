use crate::{block_header::BlockHeader, transaction::SignedTransaction};
use utils::gas::Gas;
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

impl BlockBody
{
    pub fn new(
        transaction: Vec<Arc<SignedTransaction>>, 
        gas_used: Gas, 
        gas_limit: Gas
        ) -> Self {
           BlockBody {
               transaction,
               gas_used,
               gas_limit,
           }
    }

    pub fn transaction(&self) -> &[Arc<SignedTransaction>]
    {
        &self.transaction
    }

    pub fn gas_used(&self) -> &Gas
    {
        &self.gas_used
    }

    pub fn gas_limit(&self) -> &Gas
    {
        &self.gas_limit
    }
}
