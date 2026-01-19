use crate::enums::ide::IDE;
use crate::services::service;
use crate::utils::file_manager::Workspace;
use inquire::{Select, Text};
use std::env;                
use std::path::PathBuf;        

fn resolve_path(path_str: &str) -> PathBuf {
    if path_str.starts_with("~") {
        match env::var("HOME") {
            Ok(home) => {
                let new_path = path_str.replacen("~", &home, 1);
                PathBuf::from(new_path)
            },
            Err(_) => PathBuf::from(path_str), 
        }
    } else {
        PathBuf::from(path_str)
    }
}

pub fn run() {
    let workspace = match Workspace::new() {
        Ok(ws) => ws,
        Err(e) => { eprintln!("❌ Init Error: {}", e); return; }
    };
    let ide_options = IDE::all(); 
    
    let source_ide = match Select::new("Select the SOURCE IDE:", ide_options).prompt() {
        Ok(ide) => ide,
        Err(_) => return,
    };

    let path_str = match Text::new("Enter the source file path:").prompt() {
        Ok(p) => p,
        Err(_) => return,
    };

    if let Err(e) = execute_broadcast(source_ide, &path_str, &workspace) {
        eprintln!("❌ Error: {}", e);
    } else {
        println!("✨ All conversions completed successfully!");
    }
}

fn execute_broadcast(source_ide: IDE, path_str: &str, workspace: &Workspace) -> Result<(), Box<dyn std::error::Error>> {
    let source_path = resolve_path(path_str);

    if !source_path.exists() {
        return Err(format!("Path {:?} does not exist or you don't have access!", source_path).into());
    }
    
    let local_path = workspace.import_file(&source_path)?;
    let content = workspace.read_file(&local_path)?;
    
    let default_config = service::parse_source(source_ide, &content)?;

    let default_json = service::to_json_pretty(&default_config)?;
    workspace.save_output("default_schema.json", &default_json)?;
    println!("✅ Saved: io/output/default_schema.json");
    
    for target_ide in IDE::all() {
        if target_ide == source_ide {
            continue; 
        }

        let output_content = service::generate_target(target_ide, default_config.clone())?;
        let filename = target_ide.get_default_filename();
        
        workspace.save_output(filename, &output_content)?;
        println!("✅ Generated: io/output/{} ({:?})", filename, target_ide);
    }
    Ok(())
}