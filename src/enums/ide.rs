use std::fmt;

#[derive(Debug, Clone, Copy)]
pub enum IDE {
    VSCode,
    JetBrains,
}

impl fmt::Display for IDE {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            IDE::VSCode => write!(f, "1- Visual Studio Code"),
            IDE::JetBrains => write!(f, "2- JetBrains IDE")
        }
    }
}