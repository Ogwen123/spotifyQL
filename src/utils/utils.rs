use crate::ui::tui::{Log, Severity};
use arboard::{Clipboard, GetExtLinux, LinuxClipboardKind};
use chrono::Utc;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn iso_str() -> String {
    Utc::now().to_rfc3339()
}

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

pub fn get_clipboard_item(_lb: Option<&mut Vec<Log>>) -> String {
    let clipboard_res = Clipboard::new();
    let mut clipboard = match clipboard_res {
        Ok(res) => res,
        Err(err) => {
            if let Some(lb) = _lb {
                lb.push(Log::new(
                    format!("Could not access clipboard ({})", err),
                    Severity::Error,
                ));
            }
            return String::new();
        }
    };
    
    let text = match clipboard.get().clipboard(LinuxClipboardKind::Clipboard).text() {
        Ok(res) => res,
        Err(err) => {
            if let Some(lb) = _lb {
                lb.push(Log::new(
                    format!("Could not access clipboard ({})", err),
                    Severity::Error,
                ));
            }
            return String::new();
        }
    };

    text
}
