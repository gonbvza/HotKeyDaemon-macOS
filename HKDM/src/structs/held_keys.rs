#[derive(Debug)]
pub struct CurrentKeys {
    pub command: bool,
    pub shift: bool,
    pub option: bool,
    pub control: bool,
    pub function: bool,
    // Setting the key to be a key enum from rdev
    pub key: Option<rdev::Key>,
}

impl CurrentKeys {
    pub fn new() -> Self {
        CurrentKeys {
            command: false,
            shift: false,
            option: false,
            control: false,
            function: false,
            key: None,
        }
    }

    pub fn toggle(&mut self, event_key: rdev::Key, held: bool) {
        use rdev::Key::*;
        match event_key {
            ControlLeft => self.control = held,
            MetaLeft => self.command = held,
            ShiftLeft => self.shift = held,
            Alt => self.option = held,
            Function => self.function = held,
            other => {
                if held {
                    self.key = Some(other)
                } else {
                    self.key = None
                }
            }
        }
    }

    pub fn struct_debug(&self) {
        println!("{:?}", self);
    }
}
