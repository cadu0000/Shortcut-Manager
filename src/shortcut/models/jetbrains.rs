use serde::{Deserialize, Serialize};

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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct JetBrainsAction {
    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "keyboard-shortcut", default)]
    pub keyboard_shortcuts: Vec<KeyboardShortcut>,

    #[serde(rename = "mouse-shortcut", default)]
    pub mouse_shortcuts: Vec<MouseShortcut>,
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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MouseShortcut {
    #[serde(rename = "keystroke")]
    pub keystroke: String,

    #[serde(rename = "remove", default)]
    pub remove: Option<bool>,
}
