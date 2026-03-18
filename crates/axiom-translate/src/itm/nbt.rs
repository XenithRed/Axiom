use bytes::Bytes;
use crate::error::{Err, R};

pub struct JavaItem {
    pub a: i32,
    pub b: i8,
    pub c: Option<Bytes>,
}

pub struct BedrockItem {
    pub a: i32,
    pub b: i16,
    pub c: i8,
    pub d: Option<Bytes>,
}

pub fn java_to_bedrock_item(java: &JavaItem) -> R<BedrockItem> {
    let (bid, dmg) = super::table::java_to_bedrock(java.a)?;
    Ok(BedrockItem {
        a: bid,
        b: dmg,
        c: java.b,
        d: java.c.as_ref().map(|n| bridge_nbt(n)),
    })
}

pub fn bedrock_to_java_item(bedrock: &BedrockItem) -> R<JavaItem> {
    let jid = super::table::bedrock_to_java(bedrock.a)?;
    Ok(JavaItem {
        a: jid,
        b: bedrock.c,
        c: bedrock.d.as_ref().map(|n| bridge_nbt_rev(n)),
    })
}

fn bridge_nbt(src: &Bytes) -> Bytes {
    src.clone()
}

fn bridge_nbt_rev(src: &Bytes) -> Bytes {
    src.clone()
}
