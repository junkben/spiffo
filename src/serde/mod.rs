macro_rules! serde_string {
    ($type:ident) => {
        pub mod $type {
            use serde::{Deserialize, Serialize};
            use std::str::FromStr;

            /// Deserialize from String
            pub fn deserialize<'de, D>(deserializer: D) -> Result<$type, D::Error>
            where
                D: serde::de::Deserializer<'de>,
            {
                let string = String::deserialize(deserializer)?;
                let value = $type::from_str(&string).unwrap();
                Ok(value)
            }

            /// Serialize to String
            pub fn serialize<S>(value: &$type, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::ser::Serializer,
            {
                format!("{}", value).serialize(serializer)
            }
        }
    };
}

serde_string!(bool);
serde_string!(u32);
serde_string!(u16);

pub mod f32 {
    use serde::{Deserialize, Serialize};
    use std::str::FromStr;

    /// Deserialize from String
    pub fn deserialize<'de, D>(deserializer: D) -> Result<f32, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        let string = String::deserialize(deserializer)?;
        let value = f32::from_str(&string).unwrap();
        Ok(value)
    }

    /// Serialize to String
    pub fn serialize<S>(value: &f32, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        format!("{:#?}", value).serialize(serializer)
    }
}
