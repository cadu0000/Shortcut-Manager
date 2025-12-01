use serde::{Deserialize, Serialize};
use crate::shortcut::models::vscode::{VsCodeShortcut, VsCodeShortcutConfig};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Shortcut {
    pub key: String,
    pub value: String,
    pub optional: Option<String>,
}

impl Shortcut {
    pub fn new(key: impl Into<String>, value: impl Into<String>, optional: Option<String>) -> Self {
         Shortcut {
             key: key.into(),
             value: value.into(),
             optional,
        }
    }
    
    fn from_vscode_shortcut(s: VsCodeShortcut) -> Self {
        Shortcut {
            key: s.key,
            value: s.command, 
            optional: s.when, 
        }   
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ShortcutConfig {
    pub key_bindings: Vec<Shortcut>,
}

impl ShortcutConfig {
    pub fn new() -> Self {
        Self {
            key_bindings: Vec::new(),
        }
    }
    
    pub fn add_shortcut(&mut self, shortcut: Shortcut){
        self.key_bindings.push(shortcut);
    }
    
    pub fn from_vscode_config(config: VsCodeShortcutConfig) -> Self {
        let transformed_bindings = config.key_bindings
            .into_iter()                         
            .map(Shortcut::from_vscode_shortcut)
            .collect();                          
    
            ShortcutConfig {
                key_bindings: transformed_bindings,
        }
    }
}

