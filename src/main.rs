use find_cargo_toml::{bar, baz, Result};

pub fn main() -> Result<()> {
    let path = "/home/andrew/src/github/spazm/find-cargo-toml/Cargo.toml";
    let conf: Result<toml::Value> = {
        let mut buffer = String::new();
        bar(path, &mut buffer)
    };
    println!("conf: {:?}", conf);
    let conf: Result<toml::Value> = {
        let mut buffer = String::new();
        baz(&mut buffer)
    };
    println!("conf: {:?}", conf);
    Ok(())
}
