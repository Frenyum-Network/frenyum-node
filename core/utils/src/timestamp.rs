use std::time::{SystemTime, UNIX_EPOCH};
use std::fmt;
use std::str::FromStr;

pub struct Timestamp(u64);

impl Timestamp
{
    pub const MAX: Timestamp = Timestamp(u64::MAX);

    pub fn now(timestamp: u64) -> Self 
    {
        let start = SystemTime::now();
        let since_the_epoch = start.duration_since(UNIX_EPOCH).expect("Time went backwards");
        Timestamp(since_the_epoch.as_secs())
    }
    
    pub fn reset(&mut self)
    {
        self.0 = 0;
    }
    
    pub fn elapsed(&self) -> u64
    {
        let now = Timestamp::now(0);
        now.0 - self.0
    }
}

impl fmt::Display for Timestamp
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        write!(f, "{}", self.0)
    }
}

impl FromStr for Timestamp
{
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err>
    {
        match s.parse::<u64>() {
            Ok(value) => Ok(Timestamp(value)),
            Err(_) => Err(()),
        }
    }
}

impl std::ops::Sub<Timestamp> for Timestamp
{
    type Output = Timestamp;

    fn sub(self, rhs: Timestamp) -> Timestamp
    {
        Timestamp(self.0 - rhs.0)
    }
}

impl std::ops::Add<u64> for Timestamp
{
    type Output = Timestamp;

    fn add(self, rhs: u64) -> Timestamp
    {
        Timestamp(self.0 + rhs)
    }
}

impl std::ops::AddAssign<u64> for Timestamp
{
    fn add_assign(&mut self, rhs: u64)
    {
        self.0 += rhs;
    }
}

impl std::ops::SubAssign<u64> for Timestamp
{
    fn sub_assign(&mut self, rhs: u64)
    {
        self.0 -= rhs;
    }
}

impl std::cmp::PartialEq for Timestamp
{
    fn eq(&self, other: &Self) -> bool
    {
        self.0 == other.0
    }
}

impl std::cmp::PartialOrd for Timestamp 
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering>
    {
        self.0.partial_cmp(&other.0)
    }
}

impl std::cmp::Eq for Timestamp {}

impl std::cmp::Ord for Timestamp 
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering
    {
        self.0.cmp(&other.0)
    }
}



