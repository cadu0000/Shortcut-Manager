use crate::shortcut::models::default::ShortcutConfig;
use crate::shortcut::models::vscode::VsCodeShortcutConfig;
use serde::Serialize;
use serde_json::Value;

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
