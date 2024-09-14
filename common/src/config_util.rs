use serde::de::DeserializeOwned;

pub trait TomlConfig: DeserializeOwned {
    const DEFAULT_TOML: &str;
}

pub fn load_or_create<C>(path: &str) -> C
where
    C: DeserializeOwned + TomlConfig,
{
    std::fs::read_to_string(path).map_or_else(
        |_| {
            std::fs::write(path, C::DEFAULT_TOML).unwrap();
            toml::from_str(C::DEFAULT_TOML).unwrap()
        },
        |data| toml::from_str(&data).unwrap(),
    )
}
