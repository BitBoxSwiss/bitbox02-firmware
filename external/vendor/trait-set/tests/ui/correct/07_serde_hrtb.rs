//! Checks the complex serde-inspired example with
//! higher-ranked trait bounds.

use trait_set::trait_set;

pub trait Serializer {
    type Ok;
    type Error;

    fn ok_value() -> Self::Ok;
}
pub trait Deserializer<'de> {
    type Error;
}

pub trait Serialize {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer;
}

pub trait Deserialize<'de>: Sized {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>;
}

impl Serializer for u8 {
    type Ok = ();
    type Error = ();

    fn ok_value() -> Self::Ok {
        ()
    }
}

impl<'de> Deserializer<'de> for u8 {
    type Error = ();
}

impl Serialize for u8 {
    fn serialize<S>(&self, _serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        Ok(S::ok_value())
    }
}

impl<'de> Deserialize<'de> for u8 {
    fn deserialize<D>(_deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(0u8)
    }
}

trait_set! {
    pub trait Serde = Serialize + for<'de> Deserialize<'de>;
    pub trait SerdeLifetimeTemplate<'de> = Serialize + Deserialize<'de>;
}

fn test_set<T: Serde>(_arg: T) {}
fn test_template<'de, T: SerdeLifetimeTemplate<'de>>(_arg: T) {}

fn main() {
    test_set(0u8);
    test_template(0u8);
}
