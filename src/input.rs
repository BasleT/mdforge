use anyhow::{Context, Result, bail};
use std::fs;
use std::path::Path;

pub fn read_file(path: &Path) -> Result<String> {
    if !path.exists() {
        bail!(
            "Input file '{}' does not exist. Check the path and try again.",
            path.display()
        );
    }

    if !path.is_file() {
        bail!(
            "Input path '{}' is not a file. mdforge v0.1.0 supports individual text files only.",
            path.display()
        );
    }

    let contents = fs::read_to_string(path).with_context(|| {
        format!(
            "Failed to read input file '{}'. mdforge v0.1.0 supports UTF-8 text files.",
            path.display()
        )
    })?;

    if contents.trim().is_empty() {
        bail!("Input file '{}' is empty.", path.display());
    }

    Ok(contents)
}

#[cfg(test)]
mod tests {
    use super::read_file;
    use std::{fs, path::PathBuf};

    #[test]
    fn reads_text_file() {
        let path = test_path("reads_text_file.txt");
        fs::write(&path, "hello").unwrap();

        assert_eq!(read_file(&path).unwrap(), "hello");

        let _ = fs::remove_file(path);
    }

    #[test]
    fn rejects_missing_file() {
        let error = read_file(&test_path("missing.txt"))
            .unwrap_err()
            .to_string();

        assert!(error.contains("does not exist"));
    }

    fn test_path(name: &str) -> PathBuf {
        std::env::temp_dir().join(format!("mdforge-{name}"))
    }
}
