// Copyright 2026 FastLabs Developers
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use core::fmt;

use serde_core::de;

use crate::ByteSize;

macro_rules! impl_serialize {
    ($($ty:ty),* $(,)?) => {
        $(
            impl serde_core::Serialize for ByteSize<$ty> {
                fn serialize<S>(&self, ser: S) -> Result<S::Ok, S::Error>
                where
                    S: serde_core::Serializer,
                {
                    if ser.is_human_readable() {
                        ser.collect_str(self)
                    } else {
                        self.bytes().serialize(ser)
                    }
                }
            }
        )*
    };
}

impl_serialize!(u8, u16, u32, u64, usize);

macro_rules! impl_deserialize {
    ($($ty:ty => $deserialize:ident),* $(,)?) => {
        $(
            impl<'de> serde_core::Deserialize<'de> for ByteSize<$ty> {
                fn deserialize<D>(de: D) -> Result<Self, D::Error>
                where
                    D: serde_core::Deserializer<'de>,
                {
                    struct Visitor;

                    impl de::Visitor<'_> for Visitor {
                        type Value = ByteSize<$ty>;

                        fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                            formatter.write_str("an integer or string")
                        }

                        fn visit_i64<E: de::Error>(self, value: i64) -> Result<Self::Value, E> {
                            if let Ok(val) = u64::try_from(value) {
                                if val <= <$ty>::MAX as u64 {
                                    Ok(ByteSize::b(val as $ty))
                                } else {
                                    Err(E::invalid_value(
                                        de::Unexpected::Signed(value),
                                        &"integer overflow",
                                    ))
                                }
                            } else {
                                Err(E::invalid_value(
                                    de::Unexpected::Signed(value),
                                    &"integer overflow",
                                ))
                            }
                        }

                        fn visit_u64<E: de::Error>(self, value: u64) -> Result<Self::Value, E> {
                            if value <= <$ty>::MAX as u64 {
                                Ok(ByteSize::b(value as $ty))
                            } else {
                                Err(E::invalid_value(
                                    de::Unexpected::Unsigned(value),
                                    &"integer overflow",
                                ))
                            }
                        }

                        fn visit_str<E: de::Error>(self, value: &str) -> Result<Self::Value, E> {
                            if let Ok(val) = value.parse() {
                                Ok(val)
                            } else {
                                Err(E::invalid_value(
                                    de::Unexpected::Str(value),
                                    &"parsable string",
                                ))
                            }
                        }
                    }

                    if de.is_human_readable() {
                        de.deserialize_any(Visitor)
                    } else {
                        de.$deserialize(Visitor)
                    }
                }
            }
        )*
    };
}

impl_deserialize!(
    u8 => deserialize_u8,
    u16 => deserialize_u16,
    u32 => deserialize_u32,
    u64 => deserialize_u64,
);

#[cfg(target_pointer_width = "16")]
impl_deserialize!(usize => deserialize_u16);

#[cfg(target_pointer_width = "32")]
impl_deserialize!(usize => deserialize_u32);

#[cfg(target_pointer_width = "64")]
impl_deserialize!(usize => deserialize_u64);

#[cfg(test)]
mod tests {
    use serde::Deserialize;
    use serde::Serialize;

    use crate::BSize;

    #[test]
    fn test_serde() {
        #[derive(Serialize, Deserialize)]
        struct S {
            x: BSize,
        }

        let s = serde_json::from_str::<S>(r#"{ "x": "5 B" }"#).unwrap();
        assert_eq!(s.x, BSize::b(5));

        let s = serde_json::from_str::<S>(r#"{ "x": 1048576 }"#).unwrap();
        assert_eq!(s.x, "1 MiB".parse::<BSize>().unwrap());

        let s = toml::from_str::<S>(r#"x = "2.5 MiB""#).unwrap();
        assert_eq!(s.x, "2.5 MiB".parse::<BSize>().unwrap());

        // i64 MAX
        let s = toml::from_str::<S>(r#"x = "9223372036854775807""#).unwrap();
        assert_eq!(s.x, "9223372036854775807".parse::<BSize>().unwrap());
    }

    #[test]
    fn test_serde_json() {
        let json = serde_json::to_string(&BSize::mib(1)).unwrap();
        assert_eq!(json, "\"1048576 B\"");

        let deserialized = serde_json::from_str::<BSize>(&json).unwrap();
        assert_eq!(deserialized.bytes(), 1048576);
    }
}
