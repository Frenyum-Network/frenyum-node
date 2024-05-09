use std::fmt;

#[derive(Debug, Eq, Hash, PartialEq)]
pub enum Column
{
    BlockHeader,
    BlockBody,
    Transaction,
}

impl Column
{
    pub fn is_type(&self, column_type: &str) -> bool
    {
        match (self, column_type)
        {
            (Column::BlockHeader, "BlockHeader") => true,
            (Column::BlockBody, "BlockBody") => true,
            (Column::Transaction, "Transaction") => true,
            _ => false,
        }
    }
    
    pub fn to_string(&self) -> String 
    {
        match self 
        {
            Column::BlockHeader => "BlockHeader".to_string(),
            Column::BlockBody => "BlockBody".to_string(),
            Column::Transaction => "Transaction".to_string(),
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
