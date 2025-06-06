# HotKeyDaemon-macOS
HotKeyDaemon-macOS is a fast, lightweight, and customizable background service built in Rust that empowers macOS users to trigger powerful command-line actions with just a keyboard shortcut. 

## Features
- Parses keybindings from a TOML file.
- Tracks currently held keys 
- Listens for key events in real-time usin`rdev`
- Detects and matches key combinations against predefined bindings. 
- Support common modifiers like `command`, `control`, `shift`, `option`, `function`. 

## Binding example

```
[[Bindings]]
key = "a"
modifiers = ["control", "shift"]
command = ["echo", "Hello World"]
```

## How to run
1. Clone the repo
```
git clone https://github.com/yourusername/keybinding-listener.git
cd keybinding-listener
```

2. Add your custom bindings
3. Build and run `cargo run`


