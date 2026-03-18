use bytes::{BufMut, Bytes, BytesMut};
use crate::{
    blk::{BedrockState, JavaState, java_to_bedrock},
    chk::light::{full_bright_sky, zero_light},
    error::{Err, R},
};

const VOL: usize = 4096;
const STORAGE_VER: u8 = 8;

pub struct BedrockSubChunk {
    pub a: i8,
    pub b: Bytes,
}

pub fn java_section_to_bedrock(
    section_y: i8,
    block_ids: &[JavaState; 4096],
    sky_light: Option<&[u8; 2048]>,
    blk_light: Option<&[u8; 2048]>,
) -> R<BedrockSubChunk> {
    let mut out = BytesMut::new();

    out.put_u8(STORAGE_VER);
    out.put_u8(1);

    let (bits, palette) = build_palette(block_ids)?;
    write_block_storage(&mut out, block_ids, bits, &palette)?;

    let sky = sky_light.copied().unwrap_or_else(full_bright_sky);
    let blk = blk_light.copied().unwrap_or_else(zero_light);
    out.extend_from_slice(&sky);
    out.extend_from_slice(&blk);

    Ok(BedrockSubChunk { a: section_y, b: out.freeze() })
}

fn build_palette(blocks: &[JavaState; VOL]) -> R<(u8, Vec<BedrockState>)> {
    let mut palette: Vec<BedrockState> = Vec::new();
    let mut seen = ahash::AHashMap::new();

    for &js in blocks {
        let bs = java_to_bedrock(js).unwrap_or(BedrockState::AIR);
        seen.entry(bs).or_insert_with(|| {
            let idx = palette.len();
            palette.push(bs);
            idx
        });
    }

    let bits = needed_bits(palette.len());
    Ok((bits, palette))
}

fn needed_bits(n: usize) -> u8 {
    match n {
        0..=1   => 1,
        2..=4   => 2,
        5..=16  => 4,
        17..=256 => 8,
        _        => 16,
    }
}

fn write_block_storage(
    out: &mut BytesMut,
    blocks: &[JavaState; VOL],
    bits: u8,
    palette: &[BedrockState],
) -> R<()> {
    let flags: u8 = (bits << 1) | 1;
    out.put_u8(flags);

    let per_word = 32usize / bits as usize;
    let words = (VOL + per_word - 1) / per_word;

    let idx_map: ahash::AHashMap<BedrockState, u32> = palette.iter()
        .enumerate()
        .map(|(i, &s)| (s, i as u32))
        .collect();

    let mut word = 0u32;
    let mut bit_pos = 0usize;
    let mut word_count = 0usize;

    for &js in blocks {
        let bs = java_to_bedrock(js).unwrap_or(BedrockState::AIR);
        let idx = *idx_map.get(&bs).unwrap_or(&0);
        word |= idx << bit_pos;
        bit_pos += bits as usize;
        if bit_pos >= 32 {
            out.extend_from_slice(&word.to_le_bytes());
            word = 0;
            bit_pos = 0;
            word_count += 1;
        }
    }
    if bit_pos > 0 {
        out.extend_from_slice(&word.to_le_bytes());
    }

    write_palette_nbt(out, palette);
    Ok(())
}

fn write_palette_nbt(out: &mut BytesMut, palette: &[BedrockState]) {
    write_var32(out, palette.len() as u32);
    for bs in palette {
        write_var32(out, bs.runtime_id());
    }
}

fn write_var32(out: &mut BytesMut, mut v: u32) {
    loop {
        let x = (v & 0x7F) as u8;
        v >>= 7;
        out.put_u8(if v != 0 { x | 0x80 } else { x });
        if v == 0 { break; }
    }
}
