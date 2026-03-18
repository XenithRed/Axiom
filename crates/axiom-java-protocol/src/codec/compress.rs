use bytes::Bytes;
use flate2::{read::ZlibDecoder, write::ZlibEncoder, Compression};
use std::io::{Read, Write};
use crate::error::{Err, R};

pub fn compress_zlib(data: &[u8]) -> R<Bytes> {
    let mut enc = ZlibEncoder::new(Vec::new(), Compression::default());
    enc.write_all(data).map_err(|e| Err::Compress(e.to_string()))?;
    Ok(Bytes::from(enc.finish().map_err(|e| Err::Compress(e.to_string()))?))
}

pub fn decompress_zlib(data: &[u8]) -> R<Bytes> {
    let mut dec = ZlibDecoder::new(data);
    let mut out = Vec::new();
    dec.read_to_end(&mut out).map_err(|e| Err::Compress(e.to_string()))?;
    Ok(Bytes::from(out))
}
