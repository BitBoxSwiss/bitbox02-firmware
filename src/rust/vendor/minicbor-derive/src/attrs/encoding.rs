/// The encoding to use for structs and enum variants.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Encoding {
    Array,
    Map
}

impl Default for Encoding {
    fn default() -> Self {
        Encoding::Array
    }
}

