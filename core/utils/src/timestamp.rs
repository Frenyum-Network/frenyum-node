pub struct Timestamp(u64);

impl Timestamp
{
    pub const MAX: Timestamp = Timestamp(u64::MAX);

    pub fn now(timestamp: u64) -> Self 
    {
        let start = SystemTime::now();
        let since_the_epoch = start.duration_since(UNIX_EPOCH).except("Time went backwards");
        Timestamp(since_the_epoch.as_secs())
    }
    
    pub fn reset(&mut self)
    {
        self.0 = 0;
    }
    
    pub fn elapsed(&self) -> Timestamp
    {
        let now = Timestamp::now();
        Timestamp(now - self.0)
    }
}
