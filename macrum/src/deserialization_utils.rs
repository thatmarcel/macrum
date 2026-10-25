use serde::de::Error;

pub fn bool_from_optional_integer<'de, D>(deserializer: D) -> Result<bool, D::Error>
where D: serde::Deserializer<'de> {
    let Some(value): Option<u8> = serde::Deserialize::deserialize(deserializer)? else { return Ok(false) };
    Ok(value == 1)
}

pub fn non_zero_u16<'de, D>(deserializer: D) -> Result<Option<u16>, D::Error>
where D: serde::Deserializer<'de> {
    let Some(value): Option<u16> = serde::Deserialize::deserialize(deserializer)? else { return Ok(None) };
    Ok(if value == 0 { None } else { Some(value) })
}

pub fn non_zero_u64<'de, D>(deserializer: D) -> Result<Option<u64>, D::Error>
where D: serde::Deserializer<'de> {
    let Some(value): Option<u64> = serde::Deserialize::deserialize(deserializer)? else { return Ok(None) };
    Ok(if value == 0 { None } else { Some(value) })
}

pub fn required_non_zero_u64<'de, D>(deserializer: D) -> Result<u64, D::Error>
where D: serde::Deserializer<'de> {
    let value: u64 = serde::Deserialize::deserialize(deserializer)?;
    if value == 0 {
        Err(D::Error::custom("u64 required to be non-zero was zero"))
    } else {
        Ok(value)
    }
}

pub fn non_empty_string<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where D: serde::Deserializer<'de> {
    let Some(value): Option<String> = serde::Deserialize::deserialize(deserializer)? else { return Ok(None) };
    Ok(if value.is_empty() { None } else { Some(value) })
}

pub fn required_non_empty_string<'de, D>(deserializer: D) -> Result<String, D::Error>
where D: serde::Deserializer<'de> {
    let value: String = serde::Deserialize::deserialize(deserializer)?;
    if value.is_empty() {
        Err(D::Error::custom("String required to be non-empty was empty"))
    } else {
        Ok(value)
    }
}