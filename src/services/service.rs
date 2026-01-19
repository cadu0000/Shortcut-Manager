use crate::models::default::ShortcutConfig;
use crate::models::jetbrains::{JetBrainsAction, JetBrainsKeymap};
use crate::models::vscode::VsCodeShortcutConfig;
use crate::enums::ide::IDE;
use quick_xml::reader::Reader;
use quick_xml::se::to_string as to_xml_string;
use std::error::Error;

type ServiceResult<T> = Result<T, Box<dyn Error>>;

fn parse_vscode(content: &str) -> ServiceResult<ShortcutConfig> {
    let vscode_config: VsCodeShortcutConfig = serde_json::from_str(content)?;
    Ok(ShortcutConfig::from_vscode_config(vscode_config))
}

fn parse_jetbrains(content: &str) -> ServiceResult<ShortcutConfig> {
    let mut reader = Reader::from_str(content);
    reader.config_mut().trim_text(true);
    Ok(ShortcutConfig::from_jetbrains_reader(&mut reader))
}

pub fn parse_source(ide: IDE, content: &str) -> ServiceResult<ShortcutConfig> {
    match ide {
        IDE::VSCode => parse_vscode(content),
        IDE::JetBrains => parse_jetbrains(content),
    }
}

pub fn to_json_pretty<T: serde::Serialize>(data: &T) -> ServiceResult<String> {
    Ok(serde_json::to_string_pretty(data)?)
}

fn generate_vscode(config: ShortcutConfig) -> ServiceResult<String> {
    let vscode_config = VsCodeShortcutConfig::from_default_shortcut(config);
    Ok(serde_json::to_string_pretty(&vscode_config)?)
}

fn generate_jetbrains(config: ShortcutConfig) -> ServiceResult<String> {
    let mut actions: Vec<JetBrainsAction> = Vec::new();
    for shortcut in config.key_bindings {
        actions.push(JetBrainsAction::from_default(shortcut));
    }
    
    let keymap = JetBrainsKeymap::new(Some("1".to_string()), "Default Keymap", None, actions);
    
    Ok(to_xml_string(&keymap)?) 
}

pub fn generate_target(ide: IDE, config: ShortcutConfig) -> ServiceResult<String> {
    match ide {
        IDE::VSCode => generate_vscode(config),
        IDE::JetBrains => generate_jetbrains(config),
    }
}