#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RpcRequest {
    #[prost(string, tag = "1")]
    pub service_path: ::prost::alloc::string::String,
    #[prost(map = "string, string", tag = "2")]
    pub metadata: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    #[prost(bytes = "bytes", tag = "3")]
    pub payload: ::prost::bytes::Bytes,
    #[prost(int64, tag = "4")]
    pub seq_id: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RpcResponse {
    #[prost(uint32, tag = "1")]
    pub code: u32,
    #[prost(string, tag = "2")]
    pub message: ::prost::alloc::string::String,
    #[prost(map = "string, string", tag = "3")]
    pub metadata: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    #[prost(bytes = "bytes", tag = "4")]
    pub payload: ::prost::bytes::Bytes,
    #[prost(int64, tag = "5")]
    pub seq_id: i64,
}
