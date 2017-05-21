use std::collections::HashMap;
use piston::input::Button;
use game::GameInputEvent;

pub struct KeyMap {
    /// Button to multiple game events
    keymap: HashMap<Button, Vec<GameInputEvent>>,
}

impl KeyMap {
    pub fn new() -> KeyMap {
        KeyMap { keymap: HashMap::new() }
    }

    pub fn get(&mut self, button: &Button) -> Vec<GameInputEvent> {
        self.keymap
            .get(button)
            .unwrap_or(&vec![])
            .to_vec()
    }
}
