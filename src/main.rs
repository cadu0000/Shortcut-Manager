use crate::shortcut::models::default::{Shortcut, ShortcutConfig};
use crate::shortcut::models::vscode::VsCodeShortcutConfig;
use crate::shortcut::service::{
    default_to_vscode, generate_shortcut_config_json, default_to_jetbrains
};
use crate::shortcut::input_device::InputDevice;

pub mod shortcut;
pub mod ui;

fn main() {
    let shortcut_save = Shortcut::new("Ctrl+S", "save_document", None, InputDevice::Keyboard);

    let shortcut_reload = Shortcut::new("F5", "refresh_view", Some("browser".to_string()), InputDevice::Keyboard);

    let shortcut_exit = Shortcut::new("Alt+F4", "exit_application", None, InputDevice::Keyboard);

    let mut config = ShortcutConfig::new();

    config.add_shortcut(shortcut_save);
    config.add_shortcut(shortcut_reload);
    config.add_shortcut(shortcut_exit);

    let default_json = generate_shortcut_config_json(config.clone());

    println!("{}", serde_json::to_string_pretty(&default_json).unwrap());
    let vscode_config: VsCodeShortcutConfig = default_to_vscode(default_json.clone());

    let vscode_json = generate_shortcut_config_json(vscode_config.clone());
    println!("{}", serde_json::to_string_pretty(&vscode_json).unwrap());

    let jetbrains = default_to_jetbrains(default_json.clone());
    println!(
        "{}",
        serde_json::to_string_pretty(&jetbrains).unwrap()
    );
}
