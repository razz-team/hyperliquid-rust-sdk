use serde::{Serialize, Serializer, Deserialize, Deserializer};
use uuid::Uuid;

use crate::helpers::uuid_to_hex_string;

#[derive(Debug, Clone)]
pub enum OidOrCloid {
    Oid(u64),
    Cloid(String),
}

impl Serialize for OidOrCloid {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            OidOrCloid::Oid(oid) => serializer.serialize_u64(*oid),
            OidOrCloid::Cloid(cloid) => serializer.serialize_str(cloid),
        }
    }
}

impl<'de> Deserialize<'de> for OidOrCloid {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = serde_json::Value::deserialize(deserializer).map_err(serde::de::Error::custom)?;
        match value {
            serde_json::Value::Number(n) => n
                .as_u64()
                .map(OidOrCloid::Oid)
                .ok_or_else(|| serde::de::Error::custom("oid must be a non-negative integer")),
            serde_json::Value::String(s) if s.starts_with("0x") => Ok(OidOrCloid::Cloid(s)),
            serde_json::Value::String(s) => s
                .parse::<u64>()
                .map(OidOrCloid::Oid)
                .map_err(|_| serde::de::Error::custom("oid string must be a valid u64 or 0x-prefixed cloid")),
            _ => Err(serde::de::Error::custom("oid must be a number or string")),
        }
    }
}

pub trait OidOrCloidTrait {
    fn into(self) -> OidOrCloid;
}

impl OidOrCloidTrait for u64 {
    fn into(self) -> OidOrCloid {
        OidOrCloid::Oid(self)
    }
}
impl OidOrCloidTrait for String {
    fn into(self) -> OidOrCloid {
        OidOrCloid::Cloid(self)
    }
}
impl OidOrCloidTrait for Uuid {
    fn into(self) -> OidOrCloid {
        OidOrCloid::Cloid(uuid_to_hex_string(self))
    }
}
impl OidOrCloidTrait for OidOrCloid {
    fn into(self) -> OidOrCloid {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_oid_from_number() {
        let oid: OidOrCloid = serde_json::from_str("12345").unwrap();
        assert!(matches!(oid, OidOrCloid::Oid(12345)));
    }

    #[test]
    fn deserialize_oid_from_string() {
        let oid: OidOrCloid = serde_json::from_str("\"67890\"").unwrap();
        assert!(matches!(oid, OidOrCloid::Oid(67890)));
    }

    #[test]
    fn deserialize_cloid_from_hex_string() {
        let oid: OidOrCloid = serde_json::from_str("\"0xabcdef\"").unwrap();
        assert!(matches!(oid, OidOrCloid::Cloid(ref s) if s == "0xabcdef"));
    }

    #[test]
    fn deserialize_rejects_invalid_string() {
        let result: Result<OidOrCloid, _> = serde_json::from_str("\"not_a_number\"");
        assert!(result.is_err());
    }

    #[test]
    fn deserialize_rejects_negative_number() {
        let result: Result<OidOrCloid, _> = serde_json::from_str("-1");
        assert!(result.is_err());
    }
}
