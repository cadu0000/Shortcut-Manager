use inquire::{Select, error::InquireError};
use crate::enums::ide::IDE;

pub fn render_cli () {
    let options = vec![
        IDE::VSCode,
        IDE::JetBrains,
    ];
    
    let ans: Result<IDE, InquireError> = Select::new("Choose an editor to ", options).prompt();
    
    match ans {
        Ok(IDE::JetBrains) => println!("You chose JetBrains"),
        Ok(IDE::VSCode) => println!("You chose Vscode"),
        Err(err) => println!("Invalid choice: {}", err),
    }
}
