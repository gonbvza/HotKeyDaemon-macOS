use structs::bindings::BindingSet;
use structs::held_keys::CurrentKeys;
use toml::from_str;
use std::sync::Arc;

mod key_logger;
mod structs;
mod keys;

fn read_bindings() -> BindingSet {
    let binding_file = std::fs::read_to_string("./bindings.toml").unwrap();
    let toml_table = from_str(&binding_file).unwrap();
    toml_table
}

fn main() {
    println!("The bindings are for :");
    let bindings: BindingSet = read_bindings();
    for bind in &bindings.Bindings {
        let curr: CurrentKeys = bind.get_binding_combination().unwrap();
        curr.struct_debug();
    }
    key_logger::log_process(Arc::new(bindings));
}
