use crate::{block_header::BlockHeader, transaction::SignedTransaction, Address};
use std::sync::Arc;

pub struct Block
{
    header: BlockHeader,
    transaction: Vec<Arc<SignedTransaction>>,
}

impl Block
{

}
