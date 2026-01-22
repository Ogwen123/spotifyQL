macro_rules! success {
    () => {
        $crate::print!("\n")
    };
    ($($arg:tt)*) => {{
        println!("\x1b[2K\r\x1b[32m[SUCCESS]\x1b[0m {}", format!($($arg)*));
    }};
}

macro_rules! info {
    () => {
        $crate::print!("\n")
    };
    ($($arg:tt)*) => {{
        const INFO_NAME: &str = "spotifyQL";
        println!("\x1b[2K\r\x1b[34m[{}]\x1b[0m {}", INFO_NAME, format!($($arg)*));
    }};
}

macro_rules! warning {
    () => {
        $crate::print!("\n")
    };
    ($($arg:tt)*) => {{
        println!("\x1b[2K\r\x1b[33m[WARNING]\x1b[0m {}", format!($($arg)*));
    }};
}

macro_rules! error {
    () => {
        $crate::print!("\n")
    };
    ($($arg:tt)*) => {{
        println!("\x1b[2K\r\x1b[31m\x1b[1m[ERROR]\x1b[0m {}", format!($($arg)*));
    }};
}

macro_rules! fatal {
    () => {
        $crate::print!("\n")
    };
    ($($arg:tt)*) => {{
        println!("\x1b[2K\r\x1b[31m\x1b[1m[FATAL] {}\x1b[0m ", format!($($arg)*));
    }};
}

// macro_rules! success_nnl { // no new line
//     () => {
//         $crate::print!("\n")
//     };
//     ($($arg:tt)*) => {{
//         print!("\x1b[32m[SUCCESS]\x1b[0m {}", format!($($arg)*));
//     }};
// }

macro_rules! info_nnl {
    () => {
        $crate::print!("\n")
    };
    ($($arg:tt)*) => {{
        const INFO_NAME: &str = "spotifyQL";
        print!("\x1b[34m[{}]\x1b[0m {}", INFO_NAME, format!($($arg)*));
    }};
}

// macro_rules! warning_nnl {
//     () => {
//         $crate::print!("\n")
//     };
//     ($($arg:tt)*) => {{
//         print!("\x1b[33m[WARNING]\x1b[0m {}", format!($($arg)*));
//     }};
// }
// 
// macro_rules! fatal_nnl {
//     () => {
//         $crate::print!("\n")
//     };
//     ($($arg:tt)*) => {{
//         print!("\x1b[31m\x1b[1m[FATAL] {}\x1b[0m ", format!($($arg)*));
//     }};
// }

pub(crate) use error;
pub(crate) use fatal;
pub(crate) use info;
pub(crate) use success;
pub(crate) use warning;

// pub(crate) use fatal_nnl;
pub(crate) use info_nnl;
// pub(crate) use success_nnl;
// pub(crate) use warning_nnl;
