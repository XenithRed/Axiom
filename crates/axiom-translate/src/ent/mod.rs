pub mod kind;
pub mod meta;

pub use kind::{bedrock_to_java, init_default, java_to_bedrock};
pub use meta::{decode_java, encode_bedrock, java_to_bedrock as remap_meta, Entry as MetaEntry, Val as MetaVal};
