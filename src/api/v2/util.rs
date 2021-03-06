// Copyright (C) 2020 Daniel Mueller <deso@posteo.net>
// SPDX-License-Identifier: GPL-3.0-or-later

use serde::de::Error as SerdeError;
use serde::de::Unexpected;
use serde::Deserialize;
use serde::Deserializer;
use serde::Serializer;


/// Parse a `i64` from a string.
fn parse_i64<'de, D>(s: &str) -> Result<i64, D::Error>
where
  D: Deserializer<'de>,
{
  i64::from_str_radix(&s, 10)
    .map_err(|_| SerdeError::invalid_value(Unexpected::Str(&s), &"an integer"))
}

/// Parse a `u64` from a string.
fn parse_u64<'de, D>(s: &str) -> Result<u64, D::Error>
where
  D: Deserializer<'de>,
{
  u64::from_str_radix(&s, 10)
    .map_err(|_| SerdeError::invalid_value(Unexpected::Str(&s), &"an unsigned integer"))
}

/// Deserialize a string encoded `u64`, parsing the value as signed
/// first and then dropping the sign.
pub fn u64_from_i64_from_str<'de, D>(deserializer: D) -> Result<u64, D::Error>
where
  D: Deserializer<'de>,
{
  let value = parse_i64::<D>(&String::deserialize(deserializer)?)?;
  Ok(value.abs() as u64)
}

/// Deserialize a string encoded `u64`.
pub fn u64_from_str<'de, D>(deserializer: D) -> Result<u64, D::Error>
where
  D: Deserializer<'de>,
{
  parse_u64::<D>(&String::deserialize(deserializer)?)
}

/// Serialize a `u64` value as a string.
#[allow(clippy::trivially_copy_pass_by_ref)]
pub fn u64_to_str<S>(value: &u64, serializer: S) -> Result<S::Ok, S::Error>
where
  S: Serializer,
{
  serializer.serialize_str(&value.to_string())
}
