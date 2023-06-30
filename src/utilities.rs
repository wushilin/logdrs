use std::collections::HashMap;
use regex::Regex;

pub fn parse_size(arg:&str) -> Result<i64, Box<dyn std::error::Error>> {
    let unit_lookup: HashMap<&str, i64> = [
        ("k", 1000),
        ("K", 1024),
        ("m", 1000000),
        ("M", 1048576),
        ("g", 1000000000),
        ("G", 1073741824),
        ("kB", 1000),
        ("KB", 1024),
        ("KiB", 1024),
        ("mB", 1000000),
        ("MB", 1048576),
        ("MiB", 1048576),
        ("gB", 1000000000),
        ("GB", 1073741824),
        ("GiB", 1073741824),
    ]
    .iter()
    .cloned()
    .collect();
    let r = Regex::new(r"^([\d_,]+)(\D+)?$")?;
    let captures = r.captures(arg).ok_or(format!("invalid size specification: {arg}"));
    match captures {
        Err(cause) => {
            return  Err(cause.into());
        },
        _=>{}
    }
    let matches = captures.unwrap();

    let count_str = matches.get(1).unwrap().as_str().replace("_", "").replace(",", "");
    let count = count_str.parse::<i64>()?;

    let unit = matches.get(2).map(|m| m.as_str()).unwrap_or("");
    if unit.is_empty() {
        return Ok(count);
    }

    let lookup = unit_lookup.get(unit).ok_or(format!(
        "invalid unit {}. Only support {}",
        unit,
        unit_lookup.keys().map(|&s| s).collect::<Vec<&str>>().join(",")
    ))?;

    Ok(count * lookup)
}