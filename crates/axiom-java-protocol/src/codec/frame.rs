use bytes::{Buf, BufMut, Bytes, BytesMut};
use tokio_util::codec::{Decoder, Encoder};
use crate::error::{Err, R};
use crate::varint::{read_var32_bytes, write_var32, var32_len};

pub const MAX_FRAME: usize = 2_097_152;

pub struct FrameCodec {
    pub a: Option<i32>,
}

impl FrameCodec {
    pub fn new() -> Self { Self { a: None } }
    pub fn with_threshold(t: i32) -> Self { Self { a: Some(t) } }
}

impl Decoder for FrameCodec {
    type Item  = Bytes;
    type Error = Err;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Bytes>, Err> {
        let mut tmp = &src[..];
        let orig = tmp.len();

        let frame_len = match read_var32(&mut tmp) {
            Ok(n)  => n as usize,
            Err(Err::Eof) => return Ok(None),
            Err(e) => return Err(e),
        };

        if frame_len > MAX_FRAME { return Err(Err::FrameTooLarge(frame_len)); }

        let hdr_len = orig - tmp.len();

        if tmp.len() < frame_len { return Ok(None); }

        src.advance(hdr_len);
        let raw = src.split_to(frame_len).freeze();
        Ok(Some(raw))
    }
}

impl Encoder<Bytes> for FrameCodec {
    type Error = Err;

    fn encode(&mut self, item: Bytes, dst: &mut BytesMut) -> Result<(), Err> {
        match self.a {
            None => {
                write_var32(dst, item.len() as i32);
                dst.extend_from_slice(&item);
            }
            Some(threshold) => {
                encode_compressed(item, threshold, dst)?;
            }
        }
        Ok(())
    }
}

fn encode_compressed(item: Bytes, threshold: i32, dst: &mut BytesMut) -> R<()> {
    use super::compress::{compress_zlib, decompress_zlib};

    if (item.len() as i32) >= threshold {
        let compressed = compress_zlib(&item)?;
        let data_len_bytes = {
            let mut tmp = BytesMut::new();
            write_var32(&mut tmp, item.len() as i32);
            tmp.freeze()
        };
        let total = data_len_bytes.len() + compressed.len();
        write_var32(dst, total as i32);
        dst.extend_from_slice(&data_len_bytes);
        dst.extend_from_slice(&compressed);
    } else {
        let total = 1 + item.len();
        write_var32(dst, total as i32);
        dst.put_u8(0x00);
        dst.extend_from_slice(&item);
    }
    Ok(())
}

pub fn decode_compressed(raw: Bytes) -> R<Bytes> {
    use super::compress::decompress_zlib;
    let mut b = raw;
    let data_len = read_var32_bytes(&mut b)?;
    if data_len == 0 { return Ok(b); }
    decompress_zlib(&b)
}

fn read_var32(src: &mut &[u8]) -> R<i32> {
    crate::varint::read_var32(src)
}

fn read_var32_advance(src: &mut &[u8]) -> R<i32> {
    crate::varint::read_var32(src)
}
