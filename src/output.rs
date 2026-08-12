use anyhow::{Context, Result, bail};
use std::fs;
use std::path::{Path, PathBuf};

pub fn default_for_file(input: &Path) -> PathBuf {
    let stem = input
        .file_stem()
        .and_then(|value| value.to_str())
        .unwrap_or("output");

    PathBuf::from("output").join(format!("{stem}.md"))
}

pub fn default_for_prompt() -> PathBuf {
    PathBuf::from("output").join("generated.md")
}

pub fn write(path: &Path, contents: &str) -> Result<()> {
    if path.exists() && path.is_dir() {
        bail!(
            "Output path '{}' is a directory. Provide a file path instead.",
            path.display()
        );
    }

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("Failed to create {}", parent.display()))?;
    }

    fs::write(path, contents).with_context(|| format!("Failed to write {}", path.display()))
}

#[cfg(test)]
mod tests {
    use super::{default_for_file, default_for_prompt, write};
    use std::{fs, path::PathBuf};

    #[test]
    fn creates_default_output_for_prompt() {
        assert_eq!(
            default_for_prompt(),
            PathBuf::from("output").join("generated.md")
        );
    }

    #[test]
    fn creates_default_output_for_file() {
        assert_eq!(
            default_for_file(&PathBuf::from("notes.txt")),
            PathBuf::from("output").join("notes.md")
        );
    }

    #[test]
    fn writes_explicit_output_path() {
        let path = std::env::temp_dir()
            .join("mdforge-output-test")
            .join("nested")
            .join("out.md");

        write(&path, "# Test").unwrap();

        assert_eq!(fs::read_to_string(&path).unwrap(), "# Test");

        let _ = fs::remove_file(&path);
    }
}
