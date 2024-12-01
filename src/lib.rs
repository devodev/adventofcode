use anyhow::{anyhow, Context, Result};

pub fn split2<T>(val: &str) -> Result<(T, T)>
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::error::Error + Send + Sync + 'static,
{
    let mut split = val.split_whitespace();
    let left = split
        .next()
        .ok_or(anyhow!("invalid input formatting: no value for left column"))?;
    let right = split
        .next()
        .ok_or(anyhow!("invalid input formatting: no value for right column"))?;
    if split.next().is_some() {
        return Err(anyhow!("invalid input formatting: more than 1 column"));
    }

    Ok((
        left.parse::<T>().context("parsing left")?,
        right.parse::<T>().context("parsing right")?,
    ))
}
