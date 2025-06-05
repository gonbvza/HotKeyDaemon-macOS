use std::fs;
use std::io::Result;
use std::path::Path;

pub fn write_plist_file(plist_service: &launchctl::Service) -> Result<()> {
    let plist_file = format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
        <!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN"
         "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
        <plist version="1.0">
        <dict>
            <key>Label</key>
            <string>{}</string>
            <key>ProgramArguments</key>
            <array>
                <string>{}</string>
            </array>
            <key>RunAtLoad</key>
            <true/>
            <key>KeepAlive</key>
            <true/>
            <key>StandardOutPath</key>
            <string>/var/log/my_rust_daemon.log</string>
            <key>StandardErrorPath</key>
            <string>/var/log/my_rust_daemon.err</string>
        </dict>
        </plist>
        "#,
        plist_service.name,
        std::env::current_exe().unwrap().to_str().unwrap()
    );
    let path = Path::new(&plist_service.plist_path);
    if let Some(parent) = path.parent() {
        println!("Creating directory: {:?}", parent);
        fs::create_dir_all(parent)?;
    }
    println!("Plist path: {}", plist_service.plist_path);
    fs::write(plist_service.plist_path.clone(), plist_file)?;
    println!("Plist file written successfully.");
    Ok(())
}
