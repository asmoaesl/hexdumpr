#[macro_use]
extern crate structopt;
extern crate ansi_term;

use ansi_term::Color::{Blue, Fixed};
use std::cmp;
use std::fs::File;
use std::io::{stdout, Read, Write};
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt()]
struct Opt {
    #[structopt(name = "FILE", parse(from_os_str))]
    file_name: PathBuf,

    #[structopt(short = "s", long = "offset",
                help = "Skip offset bytes from the beginning of the input")]
    offset: Option<usize>,
    #[structopt(short = "n", long = "length", help = "Interpret only length bytes of input")]
    length: Option<usize>,

    #[structopt(short = "b", long = "one-byte-octal", help = "One-byte octal display")]
    one_byte_octal: bool,
    #[structopt(short = "c", long = "one-byte-char", help = "One-byte character display")]
    one_byte_char: bool,
    #[structopt(short = "C", long = "canonical-hex", help = "Canonical hexadecimal display")]
    canonical_hex: bool,
    #[structopt(short = "x", long = "two-byte-hex",
                help = "Two-byte hexadecimal display (default)")]
    two_byte_hex: bool,
    #[structopt(short = "d", long = "two-byte-dec", help = "Two-byte decimal display")]
    two_byte_dec: bool,
    #[structopt(short = "o", long = "two-byte-octal", help = "Two-byte octal display")]
    two_byte_octal: bool,
    #[structopt(long = "no-color", help = "Disable colored ANSI output")]
    no_color: bool,
}

fn main() {
    // parse command line arguments
    let opt = Opt::from_args();

    // offset in bytes
    let offset = opt.offset.unwrap_or(0);

    // get filename
    let filename = &opt.file_name;

    print!("{}:", filename.to_str().unwrap());

    let mut f = File::open(&filename).expect("Unable to open file");
    let mut data = Vec::new();
    f.read_to_end(&mut data).expect("Unable to read data");
    if data.len() < offset {
        return;
    }

    // length in bytes
    let mut end = data.len();
    if opt.length.is_some() {
        let length = opt.length.unwrap();
        if length < data.len() - offset {
            end = offset + length;
        }
    }
    if end == 0 {
        return;
    }

    // display mode
    #[cfg_attr(rustfmt, rustfmt_skip)]
    let display = if opt.one_byte_octal { Display::OneByteOctal }
    else if opt.one_byte_char { Display::OneByteChar }
    else if opt.canonical_hex { Display::CanonicalHex }
    else if opt.two_byte_dec { Display::TwoByteDecimal }
    else if opt.two_byte_octal { Display::TwoByteOctal }
    else if opt.two_byte_hex { Display::TwoByteHex }
    else { Display::TwoByteHex };

    if !opt.no_color && cfg!(windows) {
        ansi_term::enable_ansi_support().unwrap();
    }

    print_hexdump(&data[offset..end], offset, display, &opt);
}

#[derive(Clone, Copy, PartialEq)]
enum Display {
    OneByteOctal,   // -b
    OneByteChar,    // -c
    CanonicalHex,   // -C
    TwoByteDecimal, // -d
    TwoByteOctal,   // -o
    TwoByteHex,     // -x, default
}

fn print_hexdump(data: &[u8], offset: usize, display: Display, opt: &Opt) {
    let no_color = opt.no_color;
    let stdout = stdout();
    let mut handle = stdout.lock();

    let bytes = match display {
        Display::OneByteOctal => 1,
        Display::OneByteChar => 1,
        Display::CanonicalHex => 1,
        Display::TwoByteDecimal => 2,
        Display::TwoByteOctal => 2,
        Display::TwoByteHex => 2,
    };

    let mut address = 0;
    while address <= data.len() {
        let end = cmp::min(address + 16, data.len());
        print_line(
            &data[address..end],
            address + offset,
            display,
            bytes,
            no_color,
            &mut handle,
        );
        address = address + 16;
    }
}

fn print_line(
    line: &[u8],
    address: usize,
    display: Display,
    bytes: usize,
    no_color: bool,
    handle: &mut ::std::io::StdoutLock,
) {
    use Display::*;

    // print address
    if no_color {
        write!(handle, "\n{:08x}:", address).unwrap();
    } else {
        write!(handle, "\n{} ", Blue.paint(format!("{:08x}", address))).unwrap();
    }

    let words = match (line.len() % bytes) == 0 {
        true => line.len() / bytes,
        false => (line.len() / bytes) + 1,
    };

    for b in 0..words {
        let word = match bytes {
            1 => line[b] as u16,
            _ => match line.len() == bytes * b + 1 {
                true => u16::from_be(((line[bytes * b] as u16) << 8) + 0),
                false => {
                    u16::from_be(((line[bytes * b] as u16) << 8) + (line[bytes * b + 1] as u16))
                }
            },
        };

        match display {
            OneByteOctal => write!(handle, " {:03o}", word).unwrap(),
            OneByteChar => match ((word as u8) as char).is_control() {
                true => write!(handle, " ").unwrap(),
                false => write!(handle, " {:03}", (word as u8) as char).unwrap(),
            },
            CanonicalHex => write!(handle, " {:02x}", word).unwrap(),
            TwoByteDecimal => write!(handle, " {:05} ", word).unwrap(),
            TwoByteOctal => write!(handle, " {:06o} ", word).unwrap(),
            TwoByteHex => write!(handle, " {:04x}", word).unwrap(),
        }
    }

    if display != Display::OneByteChar {
        if (line.len() % 16) > 0 {
            // align
            let words_left = (16 - line.len()) / bytes;
            let word_size = match display {
                OneByteOctal => 4,
                OneByteChar => 4,
                CanonicalHex => 3,
                TwoByteDecimal => 8,
                TwoByteOctal => 8,
                TwoByteHex => 5,
            };
            for _ in 0..word_size * words_left {
                write!(handle, " ").unwrap();
            }
        }

        write!(handle, "  ").unwrap();
        for c in line {
            // replace all control chars with dots
            match (*c as char).is_control() {
                true => write!(handle, ".").unwrap(),
                false => {
                    if no_color {
                        write!(handle, "{}", (*c as char)).unwrap();
                    } else {
                        write!(handle, "{}", Fixed(202).paint((*c as char).to_string())).unwrap();
                    }
                }
            }
        }
    }
}
