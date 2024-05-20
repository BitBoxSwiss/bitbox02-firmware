/// The encoding to use for structs and enum variants.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum Encoding {
    #[default]
    Array,
    Map
}
