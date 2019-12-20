use std::env::current_dir;
use std::path::{Path,PathBuf};
use std::io;


pub fn find_config_toml() -> Result<(PathBuf),(io::Error)>{
    //! Walks upward from `current_dir()` looking for the first `Cargo.toml` file.
    find_file_upwards(current_dir()?, "Cargo.toml")
}


pub fn find_config_toml_from_path<P: Into<PathBuf>>(path: P) -> Result<(PathBuf),(io::Error)>{
    //! Walks upward from the specified `path` looking for the first `Cargo.toml` file.
    find_file_upwards(path.into(), "Cargo.toml")
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
    use super::{
        find_config_toml,
        find_config_toml_from_path,
        find_file_upwards};
    use std::path::{Path,PathBuf};
    use std::env::current_dir;
    use std::io;

    #[test]
    fn test_find_config_toml() {
        // Expects to find our own Cargo.toml file.  Tests run from the root directory
        let cwd = current_dir().expect("expected a working current_directory");
        let expected = cwd.join("Cargo.toml");
        assert_eq!(find_config_toml().unwrap(), expected);
    }
    #[test]
    fn test_find_config_toml_from_path() {
        // Expects to find our own `Cargo.toml` file by starting from `src/` subdirectory.
        let cwd = current_dir().expect("expected a working current_directory");
        let expected = cwd.join("Cargo.toml");

        // verify that it works with strings, Path and PathBuf
        assert_eq!(find_config_toml_from_path("src/").unwrap(), expected);
        assert_eq!(find_config_toml_from_path(Path::new("src/")).unwrap(), expected);
        assert_eq!(find_config_toml_from_path(PathBuf::from("src/")).unwrap(), expected);
    }

    #[test]
    /// look for `lib.rs` in `src/` dir.
    fn test_find_file_upwards() {
        let src = current_dir().expect("expected a working current_directory").join("src");
        let expected = src.join("lib.rs");

        assert_eq!(find_file_upwards(&src, "lib.rs").unwrap(), expected);

        //
        let expected = Path::new("src/lib.rs").canonicalize()
            .expect("relative path should turn into full path");
        assert_eq!(find_file_upwards("src/", "lib.rs").unwrap(), expected);

        assert!( match find_file_upwards("/tmp", "fakename.rs") {
            Err(ref e) if e.kind() == io::ErrorKind::NotFound => true,
            _ => false,
        }, "expected to reach the top of the root and fail");
    }
}
