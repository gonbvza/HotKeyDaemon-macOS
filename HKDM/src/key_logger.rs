use crate::structs::held_keys::CurrentKeys;
use rdev::listen;
use std::sync::{Arc, Mutex};

pub fn log_process() {
    let current_keys = Arc::new(Mutex::new(CurrentKeys::new()));

    let callback = move |event: rdev::Event| -> () {
        let mut current_keys = current_keys.lock().unwrap();

        match event.event_type {
            rdev::EventType::KeyPress(key) => {
                current_keys.toggle(key, true);
                current_keys.struct_debug();
                return ();
            }

            rdev::EventType::KeyRelease(key) => {
                current_keys.toggle(key, false);
                current_keys.struct_debug();
                return ();
            }

            _ => (),
        }
    };

    if let Err(error) = listen(callback) {
        println!("Error: {:?}", error)
    }
}
