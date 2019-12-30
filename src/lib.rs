use std::env::current_dir;
use std::io;
//use std::io::prelude::*;
//use std::fs::File;
use std::fs;
use std::path::{Path, PathBuf};
use std::fmt::Display;
//use std::error::Error;
use serde::Deserialize;
use toml;
// use toml::de;

#[macro_use]
extern crate error_chain;

mod error {
    error_chain! {
        foreign_links {
            Io(::std::io::Error);
            Toml(::toml::de::Error);
        }

    }
}

pub use error::*;

/////

pub fn bar<'a, T, P>(path: P, buffer: &'a mut String) -> Result<T>
where
    T: Deserialize<'a> + std::fmt::Debug,
    P: AsRef<Path> + std::fmt::Debug,
{
        // We store the result into *buffer to ensure the lifetime checker knows
        // it lives long enough for the toml decode.
        *buffer = conf_from_path_direct(path)?;
        config_toml_string(buffer)
}

pub fn baz<'a, T>(buffer: &'a mut String) -> Result<T>
where
    T: Deserialize<'a> + std::fmt::Debug,
{
        let path = "Cargo.toml";

        // We store the result into *buffer to ensure the lifetime checker knows
        // it lives long enough for the toml decode.
        *buffer = conf_from_path_direct(path)?;
        config_toml_string(buffer)
}

/// Deserialize a string of toml into type T
///
/// Returned T will have the lifetime of the input string
///
/// ```
/// let mut buffer = r#"
///    title = 'TOML Example'
///
///    [owner]
///    name = 'Lisa'
/// "#.to_owned();
///
///  /// unwrap to type toml::Value
///  let toml_value = config_toml_string::<toml::Value>(&mut buffer).unwrap();
///  println!("toml_value: {:?}", toml_value);
///  // toml_value: Table({"owner": Table({"name": String("Lisa")}), "title": String("TOML Example")})
/// ```
pub fn config_toml_string<'a, T>(buffer: &'a str) -> Result<T>
where
    T: Deserialize<'a>,
{
    toml::from_str::<T>(&buffer).map_err(|e| e.into())
}

pub fn conf_from_path_direct<P>(path: P) -> Result<(String)>
where
    P: AsRef<Path> + std::fmt::Debug,
{
    fs::read_to_string(path).map_err(|e| e.into())
}

pub fn read_and_deserialize_from_path<'a, P, T>(path: P, buffer: &'a mut String ) -> Result<T>
where P: AsRef<Path> + std::fmt::Debug,
      T: Deserialize<'a>
{
    *buffer = fs::read_to_string(path)?;
    toml::from_str::<T>(buffer).map_err(|e| e.into())
}

pub fn read_and_deserialize<'a, T>(buffer: &'a mut String ) -> Result<T>
where T: Deserialize<'a>
{
    let path = "./Cargo.toml";
    *buffer = fs::read_to_string(path)?;
    toml::from_str::<T>(buffer).map_err(|e| e.into())
}

pub fn toml_from_path<'a, P>(path: P, buffer: &mut String) -> Result<String>
where
    P: AsRef<Path>,
{
    let tmp: Result<String> = fs::read_to_string(path).map_err(|e| e.into());
    let b = tmp?;
    *buffer = b;
    Ok(r#"
        title = 'TOML Example'

        [owner]
        name = 'Lisa'
    "#
    .to_owned())
}

/*
pub fn foo<'a, T>(mut buffer: &'a mut String) -> Result<T>
where
    T: Deserialize<'a> + std::fmt::Debug {


    let path = "Cargo.toml";
    conf_from_path(path, &'a mut buffer)?;
    let conf: Result<T>;
    //conf = config_toml_string(&conf_str);
    conf = config_toml_string(&mut buffer);

    println!("conf: {:?}", conf);
    conf
}
*/

/*
pub fn config_toml_from_path<'a, T, P>(path: P, buffer: &'a str) -> Result<T>
where T: Deserialize<'a>,
      P: AsRef<Path>
{
    //let cfg_path = find_config_toml()?;

    let raw = fs::read_to_string(path)?;

    let config = toml::from_str::<T>(&raw);

    //let config_toml: T;
    //config_toml = toml::from_str(&buffer)?;

    //config_toml = toml::from_str(&buffer)
    Config {
        raw,
        config?
    }
}
*/

/*
pub fn foo<'a, T> (serialized:&'a str ) -> Result<T, toml::de::Error>
where    T: Deserialize<'a>
{
    toml::from_str::<T>(&serialized)
}
*/

pub fn find_config_toml() -> Result<PathBuf> {
    //! Walks upward from `current_dir()` looking for the first `Cargo.toml` file.
    find_file_upwards(current_dir()?, "Cargo.toml")
}

pub fn find_config_toml_from_path<P>(path: P) -> Result<PathBuf>
where P: AsRef<Path>,
{
    //! Walks upward from the specified `path` looking for the first `Cargo.toml` file.
    find_file_upwards(path, "Cargo.toml")
}

pub fn find_file_upwards<P, S>(path: P, file_name: S,) -> Result<PathBuf>
where P: AsRef<Path>,
      S: AsRef<Path> + Display  {
    //! Walks upward from `path` looking for the first file named `file_name`.
    let path = path.as_ref().canonicalize()?;

    for current in path.ancestors() {
        let manifest = current.join(&file_name);
        if manifest.is_file() {
            return Ok(manifest);
        }
    }
    Err(io::Error::new(
        io::ErrorKind::NotFound,
        format!("File Not Found in any parent: '{}'", file_name),
    ).into())
}

#[cfg(test)]
mod tests {
    use super::{config_toml_string, find_config_toml, find_config_toml_from_path, find_file_upwards};
    use std::env::current_dir;
    use std::path::{Path, PathBuf};
    use toml;

    #[test]
    fn test_config_toml_string() {
        let mut buffer = r#"
        title = 'TOML Example'

        [owner]
        name = 'Lisa'
        "#.to_owned();

        let foo = config_toml_string::<toml::Value>(&mut buffer).unwrap();
        println!("foo: {:?}", foo);
        assert!(false, "bad joss")
    }

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
        assert_eq!(
            find_config_toml_from_path(Path::new("src/")).unwrap(),
            expected
        );
        assert_eq!(
            find_config_toml_from_path(PathBuf::from("src/")).unwrap(),
            expected
        );
    }

    #[test]
    /// look for `lib.rs` in `src/` dir.
    fn test_find_file_upwards() {
        let src = current_dir()
            .expect("expected a working current_directory")
            .join("src");
        let expected = src.join("lib.rs");

        assert_eq!(find_file_upwards(&src, "lib.rs").unwrap(), expected);

        //
        let expected = Path::new("src/lib.rs")
            .canonicalize()
            .expect("relative path should turn into full path");
        assert_eq!(find_file_upwards("src/", "lib.rs").unwrap(), expected);

        /*
        assert!(
            match find_file_upwards("/tmp", "fakename.rs") {
                Err(ref e) if e == io::ErrorKind::NotFound => true,
                _ => false,
            },
            "expected to reach the top of the root and fail"
        );
        */

        /*
        assert!(
            match find_file_upwards("nonexistantpath", "fakename.rs") {
                Err(ref e) if e.kind() == io::ErrorKind::NotFound => true,
                _ => false,
            },
            "expect non-existant path to return a NotFound error."
        );
        */
    }
}
