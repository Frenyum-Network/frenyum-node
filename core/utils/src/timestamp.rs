use std::time::{SystemTime, UNIX_EPOCH};
use chrono::{DateTime, Utc};
use std::fmt;
use std::str::FromStr;

pub struct Timestamp(u64);

impl Timestamp
{
    pub const MAX: Timestamp = Timestamp(u64::MAX);

    pub fn now() -> Self 
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
        let now = Timestamp::now();
        if now >= *self
        {
            now.0 - self.0
        } else {
            self.0 - now.0
        }
    }
}

impl fmt::Display for Timestamp
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        let duration_since_epoch = std::time::Duration::from_secs(self.0);
        let datetime = UNIX_EPOCH + duration_since_epoch;
        let datetime: DateTime<Utc> = datetime.into();
        write!(f, "{}", datetime.format("%Y-%m-%d %H:%M:%S"))
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

#[cfg(test)]
mod test
{
    use super::*;

    #[test]
    fn test_now_returns_current_time()
    {
        let now = Timestamp::now();
        let system_now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        assert_eq!(now.0, system_now);
    }

    #[test]
    fn test_reset_sets_timestamp_to_zero()
    {
        let mut timestamp = Timestamp::now();
        timestamp.reset();
        assert_eq!(timestamp.0, 0);
    }

    #[test]
    fn test_elapsed_function()
    {
        let start = Timestamp::now();
        std::thread::sleep(std::time::Duration::from_secs(2));
        let end = Timestamp::now();
        let elapsed = start.elapsed();
        assert_eq!(elapsed, 2);
    }

    #[test]
    fn test_display_formats_timestap_correctly()
    {
        let timestamp = Timestamp(123456789);
        assert_eq!(format!("{}", timestamp), "1973-11-29 21:33:09");
    }

    #[test]
    fn test_subtracting_timestamps_returns_correct_difference()
    {
        let timestamp1 = Timestamp(1000);
        let timestamp2 = Timestamp(500);
        let result = timestamp1 - timestamp2;
        assert_eq!(result.0, 500);
    }

    #[test]
    fn test_adding_duration_to_timestamp_returns_correct_result()
    {
        let timestamp = Timestamp(1000);
        let result = timestamp + 500;
        assert_eq!(result.0, 1500);
    }

    #[test]
    fn test_timestamp_ord_trait_implementation()
    {
        let timestamp1 = Timestamp(1000);
        let timestamp2 = Timestamp(2000);
        assert!(timestamp1 < timestamp2);
    }

}

