use crossterm::style::{Color, SetForegroundColor, ResetColor, Attribute, SetAttribute};
use std::io::Write;
use crate::cap::Dir;
use crate::parse::ParsedPkt;

const COL_SEQ:   Color = Color::DarkGrey;
const COL_TS:    Color = Color::DarkGrey;
const COL_C2S:   Color = Color::Cyan;
const COL_S2C:   Color = Color::Green;
const COL_ID:    Color = Color::Yellow;
const COL_NAME:  Color = Color::White;
const COL_BYTES: Color = Color::DarkGrey;
const COL_FIELD: Color = Color::DarkCyan;
const COL_VAL:   Color = Color::Reset;

pub struct Fmt {
    pub a: bool,
    pub b: bool,
    pub c: bool,
}

impl Fmt {
    pub fn new(color: bool, verbose: bool, hex: bool) -> Self {
        Self { a: color, b: verbose, c: hex }
    }

    pub fn print(&self, p: &ParsedPkt, raw: &[u8]) {
        let mut out = std::io::stdout();
        let dir_str = match p.b {
            Dir::C2S => "▶  C→S",
            Dir::S2C => "◀  S→C",
        };
        let dir_col = match p.b {
            Dir::C2S => COL_C2S,
            Dir::S2C => COL_S2C,
        };

        if self.a {
            let _ = crossterm::execute!(out, SetForegroundColor(COL_SEQ));
        }
        print!("#{:<6} ", p.a);

        if self.a {
            let _ = crossterm::execute!(out, SetForegroundColor(COL_TS));
        }

        if self.a {
            let _ = crossterm::execute!(out, SetForegroundColor(dir_col), SetAttribute(Attribute::Bold));
        }
        print!("{dir_str}  ");

        if self.a {
            let _ = crossterm::execute!(out, SetAttribute(Attribute::Reset), SetForegroundColor(COL_ID));
        }
        print!("{:<6}", p.id_hex());

        if self.a {
            let _ = crossterm::execute!(out, SetForegroundColor(COL_NAME), SetAttribute(Attribute::Bold));
        }
        print!("{:<36}", p.d);

        if self.a {
            let _ = crossterm::execute!(out, SetAttribute(Attribute::Reset), SetForegroundColor(COL_BYTES));
        }
        print!("{:>6}B", p.e);

        if self.a {
            let _ = crossterm::execute!(out, ResetColor);
        }

        if self.b && !p.f.is_empty() {
            println!();
            for field in &p.f {
                if self.a {
                    let _ = crossterm::execute!(out, SetForegroundColor(COL_FIELD));
                }
                print!("         ├─ {:<20}", field.a);
                if self.a {
                    let _ = crossterm::execute!(out, SetForegroundColor(COL_VAL));
                }
                print!("{}", field.b);
                if self.a {
                    let _ = crossterm::execute!(out, ResetColor);
                }
                println!();
            }
        } else {
            println!();
        }

        if self.c && self.b {
            println!("{}", hex_dump(raw, 16));
        }
    }

    pub fn print_stats(&self, total: u64, c2s: u64, s2c: u64, bytes: u64) {
        println!();
        println!("─────────────────────────────────");
        println!("  total packets : {total}");
        println!("  client → server: {c2s}");
        println!("  server → client: {s2c}");
        println!("  total bytes   : {}", human_bytes(bytes));
        println!("─────────────────────────────────");
    }
}

pub fn hex_dump(b: &[u8], width: usize) -> String {
    let mut out = String::new();
    for (i, chunk) in b.chunks(width).enumerate() {
        out.push_str(&format!("         {:04x}  ", i * width));
        for byte in chunk { out.push_str(&format!("{byte:02x} ")); }
        let pad = width.saturating_sub(chunk.len());
        for _ in 0..pad { out.push_str("   "); }
        out.push_str(" │ ");
        for &byte in chunk {
            out.push(if byte.is_ascii_graphic() { byte as char } else { '.' });
        }
        out.push('\n');
    }
    out
}

fn human_bytes(n: u64) -> String {
    match n {
        0..=1023 => format!("{n} B"),
        1024..=1_048_575 => format!("{:.1} KB", n as f64 / 1024.0),
        1_048_576..=1_073_741_823 => format!("{:.1} MB", n as f64 / 1_048_576.0),
        _ => format!("{:.2} GB", n as f64 / 1_073_741_824.0),
    }
}
