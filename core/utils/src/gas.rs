pub struct Gas(u64);

impl Gas
{
   pub fn new(amount: u64) -> Self { Gas(amount) } 

   pub fn amount(&self) -> u64 { self.0 }
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
