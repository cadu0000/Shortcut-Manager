use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Shortcut {
    key: String,
    value: String,
    optional: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ShortcutConfig {
    pub key_bindings: Vec<Shortcut>,
}

impl Shortcut {
    pub fn new(key: impl Into<String>, value: impl Into<String>, optional: Option<String>) -> Self {
         Shortcut {
             key: key.into(),
             value: value.into(),
             optional,
        }
    }
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
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VsCodeShortcut {
    key: String,
    command: String,
    when: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VsCodeShortcutConfig {
    pub key_bindings: Vec<VsCodeShortcut>,
}

impl VsCodeShortcut {
    pub fn new(key: impl Into<String>, command: impl Into<String>, when: Option<String>) -> Self {
         VsCodeShortcut {
             key: key.into(),
             command: command.into(),
             when,
        }
    }
    fn from_default_shortcut(s: Shortcut) -> Self {
        VsCodeShortcut {
            key: s.key,
            command: s.value, 
            when: s.optional, 
        }   
    }
}

impl VsCodeShortcutConfig {
    pub fn new() -> Self {
        Self {
            key_bindings: Vec::new(),
        }
    }
    
    pub fn add_shortcut(&mut self, shortcut: VsCodeShortcut){
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

