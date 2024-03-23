use std::fmt;

#[derive(Debug)]
pub enum Column
{
    Block,
    BlockHeader,
    BlockBody,
    RawTransaction,
    SignedTransaction,
}

impl Column
{
    pub fn is_type(&self, column_type: &str) -> bool
    {
        match (self, column_type)
        {
            (Column::Block, "Block") => true,
            (Column::BlockHeader, "BlockHeader") => true,
            (Column::BlockBody, "BlockBody") => true,
            (Column::RawTransaction, "RawTransaction") => true,
            (Column::SignedTransaction, "SignedTransaction") => true,
            _ => false,
        }
    }
}

impl fmt::Display for Column
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        write!(f, "{:?}", self)
    }
}
