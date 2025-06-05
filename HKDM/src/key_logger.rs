use crate::structs::{bindings::BindingSet, held_keys::CurrentKeys};
use rdev::listen;
use std::sync::{Arc, Mutex};

pub fn log_process(binding_arc: Arc<BindingSet>) {
    // Listens to the keys inputs

    let current_keys = Arc::new(Mutex::new(CurrentKeys::new()));
    let bindings = Arc::new(binding_arc);

    let callback = move |event: rdev::Event| -> () {
        let mut current_keys = current_keys.lock().unwrap();

        match event.event_type {
            rdev::EventType::KeyPress(key) => {
                current_keys.toggle(key, true);
                current_keys.check_binding(&bindings.bindings);
                return ();
            }

            rdev::EventType::KeyRelease(key) => {
                current_keys.toggle(key, false);
                return ();
            }

            _ => (),
        }
    };

    if let Err(error) = listen(callback) {
        println!("Error: {:?}", error)
    }
}
