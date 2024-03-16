use crate::{block_header::BlockHeader, transaction::SignedTransaction};
use utils::gas::Gas;
use std::{sync::Arc, fmt::Formatter, fmt};

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

impl fmt::Display for Block
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result
    {
        write!(
            f,
            "Block Information:
             Header: 
               Hash: {}
               Protocol Version: {}
               Parent Hash: {}
               Block Number: {}
               Block Height: {}
               Difficulty: {}
               Timestamp: {}
               Nonce: {}
               Total Difficulty: {}
               Transaction Root: {}
             Body: 
               Transactions: {:?}
               Gas Used: {}
               Gas Limit: {}",
            self.header.hash(),
            self.header.protocol_version(),
            self.header.parent_hash(),
            self.header.block_number(),
            self.header.block_height(),
            self.header.difficulty(),
            self.header.timestamp(),
            self.header.nonce(),
            self.header.total_difficulty(),
            self.header.transaction_root(),
            self.body.transaction(),
            self.body.gas_used(),
            self.body.gas_limit()
        )
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

impl fmt::Display for BlockBody 
{
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result
    {
        write!(
            formatter,
            "Block Body: \nTransactions: {:?}\nGas Used: {:?}\nGas Limit: {:?}",
            self.transaction,
            self.gas_used,
            self.gas_limit
        )
    }
}
