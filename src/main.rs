use clap::Parser;
use std::sync::Arc;
use structs::bindings::BindingSet;
use toml::from_str;

mod key_logger;
mod keys;
mod plist;
mod structs;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    cmd: Option<String>,
}

fn read_bindings() -> BindingSet {
    // Read bindings from the binding.toml file
    // Bindings structure should look
    // [[binding]]
    // key = ''
    // command = ['']
    // modifiers = ['']

    let binding_content = include_str!("../bindings.toml");
    let toml_table: BindingSet = from_str(binding_content).expect("Failed to parse bindings.toml");
    toml_table
}

fn main() {
    let args = Args::parse();
    if let Some(cmd) = &args.cmd {
        let plist_service = launchctl::Service::builder()
            .name("com.gonbvza.hkdm")
            .plist_path("/Users/gonbvza/Library/LaunchDaemons/com.gonbvza.hkdm.plist")
            .build();
        let write_plist_result = plist::write_plist_file(&plist_service);
        match write_plist_result {
            Ok(()) => match plist_service.start() {
                Ok(()) => println!("Service started successfully!"),
                Err(e) => eprintln!("Failed to start service: {}", e),
            },
            Err(e) => {
                eprintln!("Failed to write plist file: {}", e);
                eprintln!("Cannot start service without plist file.");
                std::process::exit(1);
            }
        }
    } else {
        let bindings: BindingSet = read_bindings();
        for bind in &bindings.bindings {
            bind.get_binding_combination().unwrap();
        }
        key_logger::log_process(Arc::new(bindings));
    }
}
