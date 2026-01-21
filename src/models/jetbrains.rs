use crate::enums::input_device::InputDevice;
use crate::models::default::Shortcut;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename = "keymap")]
pub struct JetBrainsKeymap {
    #[serde(rename = "@version", skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,

    #[serde(rename = "@name")]
    pub name: String,

    #[serde(rename = "@parent", skip_serializing_if = "Option::is_none")]
    pub parent: Option<String>,

    #[serde(rename = "action", default)]
    pub actions: Vec<JetBrainsAction>,
}

impl JetBrainsKeymap {
    pub fn new(
        version: Option<String>,
        name: impl Into<String>,
        parent: Option<String>,
        actions: Vec<JetBrainsAction>,
    ) -> Self {
        JetBrainsKeymap {
            version,
            name: name.into(),
            parent,
            actions,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct JetBrainsAction {
    #[serde(rename = "@id")]
    pub id: String,

    #[serde(rename = "keyboard-shortcut", default)]
    pub keyboard_shortcuts: Vec<KeyboardShortcut>,

    #[serde(rename = "mouse-shortcut", default)]
    pub mouse_shortcuts: Vec<MouseShortcut>,
}

impl JetBrainsAction {
    pub fn from_default(shortcut: Shortcut) -> Self {
        let mut keyboard_shortcuts = Vec::new();
        let mut mouse_shortcuts = Vec::new();

        match shortcut.device {
            InputDevice::Keyboard => {
                keyboard_shortcuts.push(KeyboardShortcut::from_default(shortcut.clone()));
            }
            InputDevice::Mouse => {
                mouse_shortcuts.push(MouseShortcut::from_default(shortcut.clone()));
            }
        }

        JetBrainsAction {
            id: shortcut.action, 
            keyboard_shortcuts,
            mouse_shortcuts,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct KeyboardShortcut {
    #[serde(rename = "@first-keystroke")]
    pub first_keystroke: String,

    #[serde(rename = "@second-keystroke", skip_serializing_if = "Option::is_none")]
    pub second_keystroke: Option<String>,

    #[serde(skip)] 
    pub remove: Option<bool>, 
}

impl KeyboardShortcut {
    pub fn new(
        first_keystroke: impl Into<String>,
        second_keystroke: Option<impl Into<String>>,
        remove: Option<bool>,
    ) -> Self {
        KeyboardShortcut {
            first_keystroke: first_keystroke.into(),
            second_keystroke: second_keystroke.map(|sk| sk.into()),
            remove,
        }
    }

    pub fn from_default(shortcut: Shortcut) -> Self {
        KeyboardShortcut {
            first_keystroke: shortcut.keystroke,
            second_keystroke: shortcut.sequence,
            remove: if shortcut.is_removal { Some(true) } else { None },
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MouseShortcut {

    #[serde(rename = "@keystroke")]
    pub keystroke: String,

    #[serde(skip)]
    pub remove: Option<bool>,
}

impl MouseShortcut {
    pub fn new(keystroke: impl Into<String>, remove: Option<bool>) -> Self {
        MouseShortcut {
            keystroke: keystroke.into(),
            remove,
        }
    }

    pub fn from_default(shortcut: Shortcut) -> Self {
        MouseShortcut {
            keystroke: shortcut.keystroke,
            remove: if shortcut.is_removal { Some(true) } else { None },
        }
    }
}