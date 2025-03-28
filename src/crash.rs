use std::panic;
use std::fs::File;
use std::io::Write;
use std::process;
use chrono::Utc;

pub fn init() {
    panic::set_hook(Box::new(|info| {
        let timestamp = Utc::now().timestamp();
        let date = Utc::now().format("%Y-%m-%d").to_string();
        let filename = format!("panic-{}-{}.log", date, timestamp);

        let mut file = match File::create(&filename) {
            Ok(f) => f,
            Err(e) => {
                eprintln!("Failed to open panic log file: {}", e);
                process::exit(1);
            }
        };

        if let Some(message) = info.payload().downcast_ref::<&str>() {
            if let Err(e) = writeln!(file, "Panic occurred: {}", message) {
                eprintln!("Failed to write panic message to log file: {}", e);
                process::exit(1);
            }
        }

        if let Some(location) = info.location() {
            if let Err(e) = writeln!(file, "Panic at file: {} line: {}", location.file(), location.line()) {
                eprintln!("Failed to write panic location to log file: {}", e);
                process::exit(1);
            }
        }

        eprintln!("Panic occurred: {:?}", info);
    }));
}

pub fn main() {
    init();
}