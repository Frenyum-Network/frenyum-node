use crate::{block_header::BlockHeader, transaction::SignedTransaction, Address};
use std::sync::Arc;

pub struct FinalizeBlock
{
    header: BlockHeader,
    transaction: Vec<Arc<SignedTransaction>>,
    validator: Address,
}
