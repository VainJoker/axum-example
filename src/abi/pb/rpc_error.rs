// all error related data structure

/// Application error definition
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(default)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RpcError {
    /// Error code, shall be 1:1 mapping with `error` crate
    #[prost(enumeration="RpcErrorCode", tag="1")]
    pub code: i32,
    /// Error message
    #[prost(string, tag="2")]
    pub message: ::prost::alloc::string::String,
}
/// error code
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum RpcErrorCode {
    Ok = 0,
    /// converted errors
    ProstDecodeError = 2000,
    ProstEncodeError = 2001,
    /// Others
    Other = 9999,
}
