use anyhow::{Context, Result};
use std::str::FromStr;
pub fn delim_parse<'a, T: FromStr>(line: impl Iterator<Item = &'a str>) -> Result<Vec<T>> {
    let b = line
        .filter(|v| !v.as_bytes().is_empty())
        .map(|v| {
            v.parse::<T>()
                .ok()
                .with_context(|| format!("could not parse **{}**", v))
        })
        .collect::<Result<Vec<T>>>();
    return b;
}
