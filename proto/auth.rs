#[derive(Clone, PartialEq, ::prost::Message)]
pub struct JsonWebToken {
    #[prost(string, tag="1")]
    pub jwt: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct User {
    #[prost(string, tag="1")]
    pub id: std::string::String,
    #[prost(string, tag="2")]
    pub first_name: std::string::String,
    #[prost(string, tag="3")]
    pub last_name: std::string::String,
    #[prost(string, tag="4")]
    pub email: std::string::String,
    #[prost(int32, tag="5")]
    pub access_level: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Resource {
    #[prost(enumeration="resource::Method", tag="1")]
    pub method: i32,
    #[prost(string, tag="2")]
    pub path: std::string::String,
    #[prost(string, tag="3")]
    pub jwt: std::string::String,
}
pub mod resource {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Method {
        Invalid = 0,
        Get = 1,
        Post = 2,
        Put = 3,
        Delete = 4,
        Head = 5,
        Connect = 6,
        Options = 7,
        Trace = 8,
        Patch = 9,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Access {
    #[prost(bool, tag="1")]
    pub has_access: bool,
}
