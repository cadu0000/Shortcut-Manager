use crate::shortcut::models::default::ShortcutConfig;
use crate::shortcut::models::jetbrains::{JetBrainsAction, JetBrainsKeymap};
use crate::shortcut::models::vscode::VsCodeShortcutConfig;
use quick_xml::reader::Reader;
use serde::Serialize;
use serde_json::Value;
use std::fs;

pub fn generate_shortcut_config_json<T: Serialize>(shortcut_config: T) -> Value {
    let json_value =
        serde_json::to_value(shortcut_config).expect("Falha ao serializar a configuração");

    json_value
}

pub fn default_to_vscode(json: Value) -> VsCodeShortcutConfig {
    let default_config: ShortcutConfig = serde_json::from_value(json)
        .expect("Falha ao desserializar JSON de volta para ShortcutConfig.");

    let vscode_config = VsCodeShortcutConfig::from_default_shortcut(default_config);

    vscode_config
}

pub fn vscode_to_default(json: Value) -> ShortcutConfig {
    let vscode_config: VsCodeShortcutConfig = serde_json::from_value(json)
        .expect("Falha ao desserializar JSON de volta para VsCodeShortcutConfig.");

    let default_config = ShortcutConfig::from_vscode_config(vscode_config);
    default_config
}

pub fn jetbrains_to_default_from_file(path: &str) -> ShortcutConfig {
    let xml_content = fs::read_to_string(path).expect("Falha ao ler arquivo XML.");

    let mut reader = Reader::from_str(&xml_content);
    let config = ShortcutConfig::from_jetbrains_reader(&mut reader);

    config
}

pub fn default_to_jetbrains(json: Value) -> JetBrainsKeymap {
    let default_config: ShortcutConfig = serde_json::from_value(json)
        .expect("Falha ao desserializar JSON de volta para ShortcutConfig.");

    let mut actions: Vec<JetBrainsAction> = Vec::new();
    for shortcut in default_config.key_bindings {
        let jetbrains_config = JetBrainsAction::from_default(shortcut);
        actions.push(jetbrains_config);
    }

    let keymap = JetBrainsKeymap::new(Some("1".to_string()), "Default Keymap", None, actions);

    return keymap;
}
