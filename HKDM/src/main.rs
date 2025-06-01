use rdev::{Event, listen};
mod structs;
mod key_logger;

fn lock() {
    let lock_file = std::fs::File::create("/tmp/hkdm.pid");
}

fn main() {
   key_logger::log_process();
}

