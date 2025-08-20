/// Implement serde serialization based on the
/// fmt::Display and std::FromStr traits.
macro_rules! serde_string_impl {
	($name:ident, $expecting:expr) => {
		#[cfg(feature = "serde")]
		impl<'de> $crate::serde::Deserialize<'de> for $name {
			fn deserialize<D>(deserializer: D) -> Result<$name, D::Error>
			where
				D: $crate::serde::de::Deserializer<'de>,
			{
				use core::fmt::{self, Formatter};
				use core::str::FromStr;
				use alloc::string::String;

				struct Visitor;
				impl<'de> $crate::serde::de::Visitor<'de> for Visitor {
					type Value = $name;

					fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
						formatter.write_str($expecting)
					}

					fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
					where
						E: $crate::serde::de::Error,
					{
						$name::from_str(v).map_err(E::custom)
					}

					fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E>
					where
						E: $crate::serde::de::Error,
					{
						self.visit_str(v)
					}

					fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
					where
						E: $crate::serde::de::Error,
					{
						self.visit_str(&v)
					}
				}

				deserializer.deserialize_str(Visitor)
			}
		}

		#[cfg(feature = "serde")]
		impl<'de> $crate::serde::Serialize for $name {
			fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
			where
				S: $crate::serde::Serializer,
			{
				serializer.collect_str(&self)
			}
		}
	};
}
