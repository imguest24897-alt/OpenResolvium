//! This file is part of OpenResolvim
//! Deleting this may cause the program to not work properly, because some lines of code use this API.
use core::panic;

use crate::vncscan;
use crate::crash;

pub fn api_init() {
    crash::init();
}

pub async fn scan_ip(ip: &str) {
    let ip = ip.to_string();
    vncscan::scan(&ip).await;
}

pub async fn screenshot(ip: &str) {
    vncscan::screenshot(ip).await;
}

pub fn crash(msg: &str) {
    // DEBUGGING ONLY!!
    if msg.is_empty() {
        panic!("A debug crash occured, but no message was provided.");
    }
    else {
        panic!("{}", msg);
    }
}