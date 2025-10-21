use serde::{Deserialize, Deserializer, de};

pub mod image;
pub mod track;
pub mod user;

pub fn bool_from_strnum<'de, D: Deserializer<'de>>(d: D) -> Result<bool, D::Error> {
    let s = String::deserialize(d)?;
    match s.trim().to_ascii_lowercase().as_str() {
        "1" | "true" | "t" | "yes" | "y" => Ok(true),
        "0" | "false" | "f" | "no" | "n" => Ok(false),
        _ => Err(de::Error::custom(format!("invalid boolean: {s}"))),
    }
}
