use crate::shortcut::models::ide::IDE;
use quick_xml::events::Event; 
use crate::shortcut::models::vscode::{VsCodeShortcut, VsCodeShortcutConfig};
use quick_xml::reader::Reader;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::io::BufRead;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Shortcut {
    pub keystroke: String,
    pub action: String,
    pub context: Option<String>,
    pub sequence: Option<String>,
    pub is_removal: bool,
}

impl Shortcut {
    pub fn to_ide_format(&self, ide: IDE) -> serde_json::Value {
        match ide {
            IDE::VSCode => self.to_vscode(),
            IDE::JetBrains => self.to_jetbrains(),
        }
    }

    pub fn new(
        keystroke: impl Into<String>,
        action: impl Into<String>,
        context: Option<String>,
    ) -> Self {
        Shortcut {
            keystroke: keystroke.into(),
            action: action.into(),
            context,
            sequence: None,
            is_removal: false,
        }
    }

    fn from_vscode_shortcut(s: VsCodeShortcut) -> Self {
        Shortcut {
            keystroke: s.key,
            action: s.command,
            context: s.when,
            sequence: None,
            is_removal: false,
        }
    }

    pub fn to_vscode(&self) -> serde_json::Value {
        let mut obj = json!({
            "key": self.keystroke,
            "command": self.action,
        });

        if let Some(context) = &self.context {
            obj["when"] = json!(context);
        }

        obj
    }

    pub fn to_jetbrains(&self) -> serde_json::Value {
        json!({
            "ToDO": self.keystroke,
        })
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

    pub fn add_shortcut(&mut self, shortcut: Shortcut) {
        self.key_bindings.push(shortcut);
    }

    pub fn from_vscode_config(config: VsCodeShortcutConfig) -> Self {
        let transformed_bindings = config
            .key_bindings
            .into_iter()
            .map(Shortcut::from_vscode_shortcut)
            .collect();

        ShortcutConfig {
            key_bindings: transformed_bindings,
        }
    }

    pub fn from_jetbrains_reader<R: BufRead>(reader: &mut Reader<R>) -> Self {
        let config = ShortcutConfig::new();

        let mut buf = Vec::new();

        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(e)) => {
                    println!("Tag: {}", String::from_utf8_lossy(e.name().0));
                }
                Ok(Event::Eof) => break,
                Err(e) => panic!("Erro no XML: {}", e),
                _ => {}
            }
            buf.clear();
        }

        config
    }
}
