use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InputDevice {
    Keyboard,
    Mouse,
}
