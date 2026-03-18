pub mod bedrock;
pub mod java;
pub mod prop;
pub mod table;

pub use bedrock::BedrockState;
pub use java::JavaState;
pub use table::{bedrock_to_java, init_default, init_from_json, java_to_bedrock};
