use std::ops::{Add, Sub, Mul, Div};

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Gas(u64);

impl Gas
{
   pub fn new(amount: u64) -> Self { Gas(amount) } 

   pub fn amount(&self) -> u64 { self.0 }
}

impl AsRef<u64> for Gas
{
    fn as_ref(&self) -> &u64
    {
        &self.0
    }
}


impl std::ops::Sub<Gas> for Gas
{
    type Output = Gas;

    fn sub(self, other: Gas) -> Gas
    {
        if self.0 >= other.0
        {
            Gas(self.0 - other.0)
        } else {
            Gas(0)
        }
    }
}

impl std::ops::Add<u64> for Gas
{
    type Output = Gas;

    fn add(self, other: u64) -> Gas
    {
        Gas(self.0 + other)
    }
}

impl std::ops::Mul<Gas> for Gas
{
    type Output = Gas;

    fn mul(self, other: Gas) -> Gas
    {
        Gas(self.0 * other.0)
    }
}

impl std::ops::Div<Gas> for Gas
{
    type Output = Gas;

    fn div(self, other: Gas) -> Gas
    {
        if other.0 != 0      
        {
            Gas(self.0 / other.0)
        } else {
            Gas(0)
        }
    }
}

impl From<u64> for Gas
{
    fn from(amount: u64) -> Gas
    {
        Gas(amount)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gas_allocation() 
    {
        let initial_gas = Gas::new(1000);
        let additional_gas = Gas::new(500);

        let result_gas = initial_gas.add(additional_gas.amount());

        assert_eq!(result_gas.amount(), 1500);
    }

    #[test]
    fn test_gas_consumption() 
    {
        let initial_gas = Gas::new(1000);
        let consumed_gas = Gas::new(700);

        let result_gas = initial_gas.sub(consumed_gas);

        assert_eq!(result_gas.amount(), 300);
    }

    #[test]
    fn test_gas_multiplication() 
    {
        let gas1 = Gas::new(100);
        let gas2 = Gas::new(5);

        let result_gas = gas1.mul(gas2);

        assert_eq!(result_gas.amount(), 500);
    }

    #[test]
    fn test_gas_division() 
    {
        let initial_gas = Gas::new(1000);
        let divisor_gas = Gas::new(200);

        let result_gas = initial_gas.div(divisor_gas);

        assert_eq!(result_gas.amount(), 5);
    }
}
