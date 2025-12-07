use device_query::{DeviceQuery, DeviceState, Keycode};

pub struct InputHandler {
    device_state: DeviceState,
}

impl InputHandler {
    pub fn new() -> Self {
        Self {
            device_state: DeviceState::new(),
        }
    }

    pub fn update_keys(&mut self, keys: &mut u16) -> Result<bool, Box<dyn std::error::Error>> {
        let keys_pressed = self.device_state.get_keys();

        let key_mappings = [
            (Keycode::Key1, 0), (Keycode::Key2, 1), (Keycode::Key3, 2), (Keycode::Key4, 3),
            (Keycode::Q, 4),    (Keycode::W, 5),    (Keycode::E, 6),    (Keycode::R, 7),
            (Keycode::A, 8),    (Keycode::S, 9),    (Keycode::D, 10),   (Keycode::F, 11),
            (Keycode::Z, 12),   (Keycode::X, 13),   (Keycode::C, 14),   (Keycode::V, 15),
        ];

        *keys = 0;
        for (keycode, chip8_key) in key_mappings.iter() {
            if keys_pressed.contains(keycode) {
                *keys |= 1 << chip8_key;
            }
        }

        let quit = keys_pressed.contains(&Keycode::Escape) || keys_pressed.contains(&Keycode::Delete);

        Ok(quit)
    }
}
