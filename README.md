# hexdumpr
Hexdump tool written in Rust, available for all operating systems Rust supports.

# Building
You need Rust installed and Cargo available from the terminal.

1. Download this repo as a zip.
2. Extract the zip into an empty directory.
3. Open a shell and run `cargo build --release`. The executable will be in `./target/release/`.

Alternatively, you can send the compiled executable to `~/.cargo/bin` (hopefully in PATH), by executing `cargo install --path .`.

# What Changed Since Forking
* Replaced `getopts` with `structopt` (clap) argument parsing.
* Added ANSI color support and printing (can be disabled.)
* Performance improvements

# Usage
## Options
| Flag (short, long) | Input | Description |
| ------------------ | ----- | ----------- |
| `-n`, `--length` | length: `usize` | Interpret only length bytes of input. |
| `-s`, `--offset` | offset: `usize` | Skip offset bytes from the beginning of the input. |
| `-b`, `--one-byte-octal` | N/A | One-byte octal display. |
| `-c`, `--one-byte-char` | N/A | One-byte character display. |
| `-C`, `--canonical` | N/A | Canonical hex display. |
| `-d`, `--two-byte-dec` | N/A | Two-byte decimal display. |
| `-o`, `--two-byte-octal` | N/A | Two-byte octal display. |
| `-x`, `--two-byte-hex` | N/A | Two-byte hexadecimal display. |

## Help
```
$ hexdumpr -h
hexdumpr 0.3.0
kovrik <kovrik0@gmail.com>

USAGE:
    hexdumpr.exe [FLAGS] [OPTIONS] <FILE>

FLAGS:
    -C, --canonical-hex     Canonical hexadecimal display
    -h, --help              Prints help information
        --no-color          Disable colored ANSI output
    -c, --one-byte-char     One-byte character display
    -b, --one-byte-octal    One-byte octal display
    -d, --two-byte-dec      Two-byte decimal display
    -x, --two-byte-hex      Two-byte hexadecimal display (default)
    -o, --two-byte-octal    Two-byte octal display
    -V, --version           Prints version information

OPTIONS:
    -n, --length <length>    Interpret only length bytes of input
    -s, --offset <offset>    Skip offset bytes from the beginning of the input

ARGS:
    <FILE>
```

## Example
```
$ hexdumpr .\src\main.rs
.\src\main.rs:
00000000  5b23 616d 7263 5f6f 7375 5d65 650a 7478  #[macro_use].ext
00000010  7265 206e 7263 7461 2065 7473 7572 7463  ern crate struct
00000020  706f 3b74 650a 7478 7265 206e 7263 7461  opt;.extern crat
00000030  2065 6e61 6973 745f 7265 3b6d 0a0a 7375  e ansi_term;..us
00000040  2065 6e61 6973 745f 7265 3a6d 433a 6c6f  e ansi_term::Col
00000050  726f 3a3a 467b 7869 6465 202c 6c42 6575  or::{Fixed, Blue
00000060  3b7d 750a 6573 7320 6474 3a3a 6d63 3b70  };.use std::cmp;
00000070  750a 6573 7320 6474 3a3a 7366 3a3a 6946  .use std::fs::Fi
00000080  656c 0a3b 7375 2065 7473 3a64 693a 3a6f  le;.use std::io:
00000090  7b3a 7473 6f64 7475 202c 6552 6461 202c  :{stdout, Read,
000000a0  7257 7469 7d65 0a3b 7375 2065 7473 3a64  Write};.use std:
000000b0  703a 7461 3a68 503a 7461 4268 6675 0a3b  :path::PathBuf;.
000000c0  7375 2065 7473 7572 7463 706f 3a74 533a  use structopt::S
000000d0  7274 6375 4f74 7470 0a3b 230a 645b 7265  tructOpt;..#[der
000000e0  7669 2865 7453 7572 7463 704f 2974 0a5d  ive(StructOpt)].
000000f0  5b23 7473 7572 7463 706f 2874 5d29 730a  #[structopt()].s
00000100  7274 6375 2074 704f 2074 0a7b 2020 2020  truct Opt {.
00000110  5b23 7473 7572 7463 706f 2874 616e 656d  #[structopt(name
00000120  3d20 2220 4946 454c 2c22 7020 7261 6573   = "FILE", parse
00000130  6628 6f72 5f6d 736f 735f 7274 2929 0a5d  (from_os_str))].
00000140  2020 2020 6966 656c 6e5f 6d61 3a65 5020      file_name: P
00000150  7461 4268 6675 0a2c 200a 2020 2320 735b  athBuf,..    #[s
```
