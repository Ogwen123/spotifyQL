use std::time::{SystemTime, UNIX_EPOCH};

pub fn secs_now() -> u64 {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("time should go forward");
    
    since_the_epoch.as_secs()
}