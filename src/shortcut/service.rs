use crate::shortcut::model::{VsCodeShortcutConfig, ShortcutConfig};
use serde_json::{Value};
use serde::Serialize;

pub fn generate_shortcut_config_json<T: Serialize>(shortcut_config: T) -> Value {
    let json_value = serde_json::to_value(shortcut_config)
        .expect("Falha ao serializar a configuração");
        
    json_value
}

pub fn export_to_vscode_type(json: Value) -> Value {
    let default_config: ShortcutConfig = serde_json::from_value(json)
        .expect("Falha ao desserializar JSON de volta para ShortcutConfig.");
    
    let vscode_config = VsCodeShortcutConfig::from_default_shortcut(default_config);

    let vscode_json = serde_json::to_value(vscode_config)
        .expect("Falha ao serializar VsCodeShortcutConfig.");

    vscode_json
}

pub fn vscode_to_default(json: Value) -> ShortcutConfig {
    let vscode_config: VsCodeShortcutConfig = serde_json::from_value(json)
        .expect("Falha ao desserializar JSON de volta para VsCodeShortcutConfig.");

    vscode_config()
}