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
        let value = String::deserialize(deserializer)?;
        if value.starts_with("0x") {
            Ok(OidOrCloid::Cloid(value))
        } else {
            Ok(OidOrCloid::Oid(value.parse().unwrap()))
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
