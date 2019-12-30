use find_cargo_toml::{read_and_deserialize, read_and_deserialize_from_path, Result};

pub fn main() -> Result<()> {
    let conf: Result<toml::Value> = {
        let mut buffer = String::new();
        read_and_deserialize(&mut buffer)
    };
    println!("conf: {:?}", conf);
    let conf: Result<toml::Value> = {
        let path = "src/";
        let mut buffer = String::new();
        read_and_deserialize_from_path(path, &mut buffer)
    };
    println!("conf: {:?}", conf);
    Ok(())
}
