use crate::block_header::BlockHeader;

pub struct FinalizeBlock
{
    header: BlockHeader,
    transaction: Vec<Arc<SignedTransaction>>,
    // Validator
}
