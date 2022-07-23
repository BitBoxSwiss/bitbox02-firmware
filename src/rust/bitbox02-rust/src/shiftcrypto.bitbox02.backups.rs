#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BackupMetaData {
    #[prost(uint32, tag="1")]
    pub timestamp: u32,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    #[prost(enumeration="BackupMode", tag="3")]
    pub mode: i32,
}
///*
/// BackupData is encoded in the data field of the BackupContent
/// and depends on the BackupMode.
/// Defining it as a protobuf message allows language/architecture independent
/// encoding/decoding.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BackupData {
    #[prost(uint32, tag="1")]
    pub seed_length: u32,
    #[prost(bytes="vec", tag="2")]
    pub seed: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint32, tag="3")]
    pub birthdate: u32,
    #[prost(string, tag="4")]
    pub generator: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BackupContent {
    #[prost(bytes="vec", tag="1")]
    pub checksum: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag="2")]
    pub metadata: ::core::option::Option<BackupMetaData>,
    /// This field is obsolete and from v9.13.0, it is set to 0.
    ///
    /// It used to be the length of the `data` field, serialized as protobuf, prefixed with the
    /// serialized field tag of the `data` field. Counting the prefix in the length is a historical
    /// accident. This field was also technically redundant, as protobuf already encodes the length
    /// when serializing the data field.
    ///
    /// Since this field is part of the checksum computation, we keep it so that existing backups can
    /// be loaded and the checksum verified. Other than that, it serves no purpose, as it is not
    /// needed to deserialize or interpret the data.
    #[prost(uint32, tag="3")]
    pub length: u32,
    #[prost(bytes="vec", tag="4")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
// NOTE! Once the firmware is released to the general public and there are actual backups it is
// strictly forbidden to modify BackupV1 and any types contained within BackupV1 because the
// checksum covers all fields. 

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BackupV1 {
    #[prost(message, optional, tag="1")]
    pub content: ::core::option::Option<BackupContent>,
}
//message Backup_V2 {
//RSBackupContent rs_content = 1;
//}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Backup {
    #[prost(oneof="backup::BackupVersion", tags="1")]
    pub backup_version: ::core::option::Option<backup::BackupVersion>,
}
/// Nested message and enum types in `Backup`.
pub mod backup {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum BackupVersion {
        ///        Backup_V2 backup_V2 = 2;
        #[prost(message, tag="1")]
        BackupV1(super::BackupV1),
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum BackupMode {
    Plaintext = 0,
}
