#[macro_use]
extern crate structopt;
extern crate ansi_term;

use structopt::StructOpt;
use std::fs::File;
use std::io::prelude::*;
use std::cmp;
use std::path::PathBuf;

#[derive(StructOpt, Debug)]
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
    #[structopt(short = "c", long = "one-byte-char")]
    one_byte_char: bool,
    #[structopt(short = "C", long = "canonical-hex")]
    canonical_hex: bool,
    #[structopt(short = "x", long = "two-byte-hex")]
    two_byte_hex: bool,
    #[structopt(short = "d", long = "two-byte-dec")]
    two_byte_dec: bool,
    #[structopt(short = "o", long = "two-byte-octal")]
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
    let filename = opt.file_name;

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
    let bytes;
    let display;
    match () {
        _ if opt.one_byte_octal => {
            display = 'b';
            bytes = 1;
        }
        _ if opt.one_byte_char => {
            display = 'c';
            bytes = 1;
        }
        _ if opt.canonical_hex => {
            display = 'C';
            bytes = 1;
        }
        _ if opt.two_byte_dec => {
            display = 'd';
            bytes = 2;
        }
        _ if opt.two_byte_octal => {
            display = 'o';
            bytes = 2;
        }
        _ => {
            display = 'x';
            bytes = 2;
        }
    }
    print_hexdump(&data[offset..end], offset, display, bytes);
}

pub fn print_hexdump(data: &[u8], offset: usize, display: char, bytes: usize) {
    let mut address = 0;
    while address <= data.len() {
        let end = cmp::min(address + 16, data.len());
        print_line(&data[address..end], address + offset, display, bytes);
        address = address + 16;
    }
}

fn print_line(line: &[u8], address: usize, display: char, bytes: usize) {
    // print address
    print!("\n{:08x}:", address);
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
            'b' => print!(" {:03o}", word),
            'c' => match ((word as u8) as char).is_control() {
                true => print!(" "),
                false => print!(" {:03}", (word as u8) as char),
            },
            'C' => print!(" {:02x}", word),
            'x' => print!(" {:04x}", word),
            'o' => print!(" {:06o} ", word),
            'd' => print!("  {:05} ", word),
            _ => print!(" {:04x}", word),
        }
    }

    if display != 'c' {
        if (line.len() % 16) > 0 {
            // align
            let words_left = (16 - line.len()) / bytes;
            let word_size = match display {
                'b' => 4,
                'c' => 4,
                'C' => 3,
                'x' => 5,
                'o' => 8,
                'd' => 8,
                _ => 5,
            };
            for _ in 0..word_size * words_left {
                print!(" ");
            }
        }

        print!("  ");
        for c in line {
            // replace all control chars with dots
            match (*c as char).is_control() {
                true => print!("."),
                false => print!("{}", (*c as char)),
            }
        }
    }
}
