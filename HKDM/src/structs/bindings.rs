use super::held_keys::CurrentKeys;
use crate::keys::to_key;

#[derive(Debug, serde::Deserialize)]
pub struct BindingSet {
    pub Bindings: Vec<Binding>,
}

#[derive(Debug, serde::Deserialize)]
pub struct Binding {
    pub key: String,
    pub modifiers: Vec<String>,
    pub command: Vec<String>,
}

impl Binding {
    pub fn get_binding_combination(&self) -> Result<CurrentKeys, ()> {
        let mut binding_keys = CurrentKeys::new();
        for modifier in &self.modifiers {
            match modifier.as_str() {
                "command" => binding_keys.command = true,
                "function" => binding_keys.function = true,
                "option" => binding_keys.option = true,
                "control" => binding_keys.control = true,
                "shift" => binding_keys.shift = true,
                other => panic!("There is a not know modifier: {}", other),
            }
        }

        binding_keys.key = to_key(&self.key);

        Ok(binding_keys)
    }

    pub fn debug_binding(&self) {
        println!("{:?}", self)
    }
}
