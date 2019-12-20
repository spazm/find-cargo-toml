use std::env::current_dir;
use std::path::{Path,PathBuf};
use std::io;


pub fn find_config_toml<P: Into<PathBuf>>(path: Option<P>) -> Result<(PathBuf),(io::Error)>{
    //! Walks upward from `path` or `current_dir()` looking for the first `Cargo.toml` file.
    let path = match path {
        Some(p) => p.into(),
        None => current_dir()?
    };

    println!("path is {:?}", path);

    find_file_upwards(path, "Cargo.toml")
}

pub fn find_file_upwards<P: Into<PathBuf>, S: AsRef<Path>>(path: P, file_name: S) -> Result<(PathBuf),(io::Error)>{
    //! Walks upward from `path` looking for the first file named `file_name`.
    let path = path.into().canonicalize()?;

    for current in path.ancestors() {
        let manifest = current.join(&file_name);
        if manifest.is_file() {
            return Ok(manifest)
        }
    }
    Err(io::Error::new(io::ErrorKind::NotFound,"Manifest Not Found, not a rust package"))
}

#[cfg(test)]
mod tests {
    use super::{find_config_toml,find_file_upwards};
    use std::path::Path;
    use std::env::current_dir;

    #[test]
    /// look for `lib.rs` in `src/` dir.
    fn test_find_file_upwards() {
        let src = current_dir().expect("expected a working current_directory").join("src");
        let expected = src.join("lib.rs");

        assert_eq!(find_file_upwards(src, "lib.rs").unwrap(), expected);

        //
        let expected = Path::new("src/lib.rs").canonicalize()
            .expect("relative path should turn into full path");
        assert_eq!(find_file_upwards("src/", "lib.rs").unwrap(), expected);
    }
    #[test]
    fn test_find_config_toml() {
        // Expects to find our own Cargo.toml file.  Tests run from the root directory
        let cwd = current_dir().expect("expected a working current_directory");
        println!("cwd is {:?}", cwd);
        let expected = cwd.join("Cargo.toml");
        assert_eq!(find_config_toml(Some(cwd)).unwrap(), expected);

        // find Cargo.toml starting in `src/` directory
        assert_eq!(find_config_toml(Some("src/")).unwrap(), expected);
    }
}
