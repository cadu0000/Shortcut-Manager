use crate::shortcut::model::Shortcut;
use crate::shortcut::model::ShortcutConfig;
use crate::shortcut::service::export_to_vscode_type;
use serde_json::Value;
use crate::shortcut::service::generate_shortcut_config_json;

pub mod shortcut;
pub mod ui;

fn main() {
    println!("Iniciando a geração de configuração de atalhos...");

    let shortcut_save = Shortcut::new(
        "Ctrl+S", 
        "save_document", 
        None 
    );

    let shortcut_reload = Shortcut::new(
        "F5", 
        "refresh_view", 
        Some(String::from("browser")) 
    );

    let shortcut_exit = Shortcut::new(
        "Alt+F4", 
        "exit_application", 
        None
    );

    let mut config = ShortcutConfig::new();

    config.add_shortcut(shortcut_save);
    config.add_shortcut(shortcut_reload);
    config.add_shortcut(shortcut_exit);

    let json_output: Value = generate_shortcut_config_json(config);

    match serde_json::to_string_pretty(&json_output){
        Ok(json_string) => {
            println!("\n✅ Configuração JSON Gerada:");
            println!("{}", json_string);
        },
        Err(e) => {
            eprintln!("❌ Erro ao formatar o JSON: {}", e);
        }
    }
    
    let vscode_json: Value = export_to_vscode_type(json_output);

    match serde_json::to_string_pretty(&vscode_json){
        Ok(jsonn) => {
            println!("{}", jsonn);
        },
        Err(e) => {
            eprintln!("{}", e);
        }
    }
}