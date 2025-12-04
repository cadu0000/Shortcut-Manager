use serde::{Deserialize, Serialize};

use crate::shortcut::models::default::Shortcut;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename = "keymap")]
pub struct JetBrainsKeymap {
    #[serde(rename = "version", default)]
    pub version: Option<String>,

    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "parent", default)]
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
    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "keyboard-shortcut", default)]
    pub keyboard_shortcuts: Vec<KeyboardShortcut>,

    #[serde(rename = "mouse-shortcut", default)]
    pub mouse_shortcuts: Vec<MouseShortcut>,
}

impl JetBrainsAction {
    pub fn new(
        id: String, //aqui eh a actionn
        keyboard_shortcuts: Vec<KeyboardShortcut>,
        mouse_shortcuts: Vec<MouseShortcut>,
    ) -> Self {
        JetBrainsAction {
            id,
            keyboard_shortcuts: keyboard_shortcuts,
            mouse_shortcuts: mouse_shortcuts,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct KeyboardShortcut {
    #[serde(rename = "first-keystroke")]
    pub first_keystroke: String,

    #[serde(rename = "second-keystroke", default)]
    pub second_keystroke: Option<String>,

    #[serde(rename = "remove", default)]
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
            remove: remove,
        }
    }

    pub fn from_default(shortcut: Shortcut) -> Self {
        KeyboardShortcut {
            first_keystroke: shortcut.keystroke,
            second_keystroke: shortcut.sequence.map(|sk| sk.into()),
            remove: shortcut.is_removal.into(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MouseShortcut {
    #[serde(rename = "keystroke")]
    pub keystroke: String,

    #[serde(rename = "remove", default)]
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
            remove: shortcut.is_removal.into(),
        }
    }
}
