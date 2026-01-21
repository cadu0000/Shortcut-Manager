use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IDE {
    VSCode,
    JetBrains,
}

impl IDE {
    pub fn all() -> Vec<IDE> {
        vec![IDE::VSCode, IDE::JetBrains]
    }

    pub fn get_default_filename(&self) -> &'static str {
        match self {
            IDE::VSCode => "vscode_keybindings.json",
            IDE::JetBrains => "jetbrains_keymap.xml",
        }
    }
}

impl fmt::Display for IDE {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            IDE::VSCode => write!(f, "1- Visual Studio Code"),
            IDE::JetBrains => write!(f, "2- JetBrains IDE")
        }
    }
}