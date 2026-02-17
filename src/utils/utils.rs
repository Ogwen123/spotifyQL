use std::time::{SystemTime, UNIX_EPOCH};

pub fn secs_now() -> u64 {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("time should go forward");

    since_the_epoch.as_secs()
}

pub fn micro_secs_now() -> u128 {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("time should go forward");

    since_the_epoch.as_micros()
}

#[inline]
pub fn bounds_loc(box_x: u16, box_y: u16, width: u16, height: u16, loc_x: u16, loc_y: u16) -> bool {
    let t = box_y;
    let l = box_x;
    let b = box_y + height;
    let r = box_x + width;
    
    if loc_x > l && loc_x < r && loc_y > t && loc_y < b {
        true
    } else {
        false
    }
}
