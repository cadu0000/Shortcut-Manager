use std::fs;
use std::path::{Path, PathBuf};
use std::error::Error;
use fs_extra::file::{copy, CopyOptions};

pub struct Workspace {
    pub input_dir: PathBuf,
    pub output_dir: PathBuf,
}

impl Workspace {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let current_dir = std::env::current_dir()?;
        let input_dir = current_dir.join("io/input");
        let output_dir = current_dir.join("io/output");

        fs::create_dir_all(&input_dir)?;
        fs::create_dir_all(&output_dir)?;

        Ok(Self { input_dir, output_dir })
    }

    pub fn import_file(&self, source_path: &Path) -> Result<PathBuf, Box<dyn Error>> {
        let file_name = source_path.file_name().ok_or("Invalid file name")?;
        let dest_path = self.input_dir.join(file_name);

        copy(source_path, &dest_path, &CopyOptions::new().overwrite(true))?;
        Ok(dest_path)
    }

    pub fn read_file(&self, path: &Path) -> Result<String, Box<dyn Error>> {
        Ok(fs::read_to_string(path)?)
    }

    pub fn save_output(&self, file_name: &str, content: &str) -> Result<(), Box<dyn Error>> {
        fs::write(self.output_dir.join(file_name), content)?;
        Ok(())
    }
}