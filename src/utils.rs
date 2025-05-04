use serde::{Deserialize, Deserializer};

pub fn empty_string_as_none<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de> + std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Display,
{
    let opt = Option::<String>::deserialize(deserializer)?;
    match opt {
        Some(s) if s.is_empty() => Ok(None),
        Some(s) => match s.parse::<T>() {
            Ok(val) => Ok(Some(val)),
            Err(err) => Err(serde::de::Error::custom(err.to_string())),
        },
        None => Ok(None),
    }
}
