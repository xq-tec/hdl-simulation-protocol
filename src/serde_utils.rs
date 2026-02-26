//! Utility functions for serializing and deserializing types which can be efficiently encoded in
//! binary formats, but which require a string representation for certain human-readable formats.
//! The primary use case is for `i64` and `u64` in JSON, because precision can be lost when
//! deserializing numbers from JSON in JavaScript.

use std::fmt;
use std::marker::PhantomData;
use std::str::FromStr;

use serde::Deserialize;
use serde::Serialize;

/// Helper type to deserialize a string representing a number,
/// without allocating a String when it isn't necessary.
struct Helper<'msg, T>(&'msg str, PhantomData<T>);

impl<'de, 'msg, T> serde::de::Visitor<'de> for Helper<'msg, T>
where
    T: FromStr,
    <T as FromStr>::Err: fmt::Display,
{
    type Value = T;

    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.0)
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        v.parse().map_err(serde::de::Error::custom)
    }
}

/// Deserializes a value of type `T`, directly encoded for binary formats,
/// or encoded as a string for human-readable formats.
pub fn deserialize<'de, 'msg, T, D>(deserializer: D, expecting: &'msg str) -> Result<T, D::Error>
where
    D: serde::Deserializer<'de>,
    T: FromStr,
    <T as FromStr>::Err: fmt::Display,
    T: Deserialize<'de>,
{
    if deserializer.is_human_readable() {
        deserializer.deserialize_str(Helper(expecting, PhantomData))
    } else {
        T::deserialize(deserializer)
    }
}

/// Serializes a value of type `T`, directly encoded for binary formats,
/// or encoded as a string for human-readable formats.
pub fn serialize<T, S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
where
    T: Serialize,
    T: fmt::Display,
    S: serde::Serializer,
{
    if serializer.is_human_readable() {
        serializer.collect_str(value)
    } else {
        value.serialize(serializer)
    }
}
