use serde::{Deserialize, Serialize};
use crate::shortcut::models::default::{Shortcut, ShortcutConfig};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VsCodeShortcut {
    pub key: String,
    pub command: String,
    pub when: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VsCodeShortcutConfig {
    pub key_bindings: Vec<VsCodeShortcut>,
}

impl VsCodeShortcut {
    pub fn new(
        key: impl Into<String>,
        command: impl Into<String>,
        when: Option<String>
    ) -> Self {
        VsCodeShortcut {
            key: key.into(),
            command: command.into(),
            when,
        }
    }
    
    pub fn from_default_shortcut(s: Shortcut) -> Self {
        VsCodeShortcut {
            key: s.keystroke,
            command: s.action,
            when: s.context,
        }
    }
}

impl VsCodeShortcutConfig {
    pub fn new() -> Self {
        Self {
            key_bindings: Vec::new(),
        }
    }

    pub fn add_shortcut(&mut self, shortcut: VsCodeShortcut) {
        self.key_bindings.push(shortcut);
    }

    pub fn from_default_shortcut(config: ShortcutConfig) -> Self {
        let transformed_bindings = config.key_bindings
            .into_iter()
            .map(VsCodeShortcut::from_default_shortcut)
            .collect();

        VsCodeShortcutConfig {
            key_bindings: transformed_bindings,
        }
    }
}
