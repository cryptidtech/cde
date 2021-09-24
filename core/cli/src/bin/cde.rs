extern crate structopt;

use cde::{ encoder, TagBuilder };
use anyhow::Result;
use log::*;
use std::ffi::OsString;
use std::fs::File;
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "cde",
    version = "0.2.0",
    author = "David Huseby <dwh@linuxprogrammer.org>",
    about = "Encode/Decode cryptographic data in CDE format",
)]
struct Opt {

    /// silence all output
    #[structopt(short = "q", long = "quiet")]
    quiet: bool,

    /// verbose output
    #[structopt(long = "verbose", short = "v", parse(from_occurrences))]
    verbosity: usize,

    /// the subcommand operation
    #[structopt(subcommand)]
    cmd: Command
}

#[derive(Debug, StructOpt)]
enum Command {

    #[structopt(name = "encode")]
    /// Encode the given file or data from stdin.
    Encode {
        /// The file to save the encoded file to, otherwise stdout.
        #[structopt(short = "o", parse(from_os_str))]
        output: Option<PathBuf>,

        /// The type string for the encoded object
        #[structopt(short = "t", long = "tt")]
        tt: String,

        /// Path of file to encode or '-' if data passed through stdin.
        #[structopt(name = "FILE", parse(from_os_str))]
        input: Option<PathBuf>
    },

    #[structopt(name = "decode")]
    /// Decode the given file or data from stdin.
    Decode {
        /// The file to save the decoded file to, otherwise stdout.
        #[structopt(short = "o", parse(from_os_str))]
        output: Option<PathBuf>,

        /// Path of file to decode or '-' if data is passed through stdin.
        #[structopt(name = "FILE", parse(from_os_str))]
        input: Option<PathBuf>
    },

    #[structopt(name = "info")]
    /// Display the type tag debug information for an encoded object
    Info {
        /// Path of file to decode or '-' if data is passed through stdin.
        #[structopt(name = "FILE", parse(from_os_str))]
        input: Option<PathBuf>
    },

    #[structopt(name = "tt")]
    /// Display the type tag string for an encoded object
    Tt {
        /// Path of file to decode or '-' if data is passed through stdin.
        #[structopt(name = "FILE", parse(from_os_str))]
        input: Option<PathBuf>
    }
}

fn writer(path: &Option<PathBuf>) -> Result<Box<dyn Write>> {
    match path {
        Some(p) => {
            let path = Path::new(&p);
            Ok(Box::new(File::create(&path)?) as Box<dyn Write>)
        }
        None => Ok(Box::new(io::stdout()) as Box<dyn Write>)
    }
}

fn writer_name(path: &Option<PathBuf>) -> Result<OsString> {
    match path {
        Some(p) => {
            Ok(p.clone().into_os_string())
        }
        None => Ok(OsString::from("stdout"))
    }
}

fn reader(path: &Option<PathBuf>) -> Result<Box<dyn Read>> {
    match path {
        Some(p) => {
            if p.to_string_lossy() == "-" {
                Ok(Box::new(io::stdin()) as Box<dyn Read>)
            } else {
                let path = Path::new(&p);
                Ok(Box::new(File::open(&path)?) as Box<dyn Read>)
            }
        }
        None => Ok(Box::new(io::stdin()) as Box<dyn Read>)
    }
}

fn reader_name(path: &Option<PathBuf>) -> Result<OsString> {
    match path {
        Some(p) => {
            if p.to_string_lossy() == "-" {
                Ok(OsString::from("stdin"))
            } else {
                Ok(p.clone().into_os_string())
            }
        }
        None => Ok(OsString::from("stdin"))
    }
}

fn main() -> Result<()> {

    // parse the command line flags
    let opt = Opt::from_args();

    // set up logger
    stderrlog::new()
        .module(module_path!())
        .quiet(opt.quiet)
        .verbosity(opt.verbosity)
        .init()?;

    debug!("cde: verbosity set to: {}", opt.verbosity);

    match opt.cmd {
        Command::Encode { output, tt, input } => {
            info!("cde: encoding from {} to {}",
                reader_name(&input)?.to_string_lossy(),
                writer_name(&output)?.to_string_lossy());

            let mut r = reader(&input)?;
            let mut w = writer(&output)?;

            // read in the data to encode
            let mut data = Vec::<u8>::new();
            let len = r.read_to_end(&mut data)?;

            // generate a type tag from the command line options
            let tt = TagBuilder::from_str(&tt).length(len as u32).build();
            debug!("\n{:?}", tt);

            // write the encoded type tag
            w.write_all(tt.encode().as_bytes())?;

            // write the encoded data
            let enc = encoder()?;
            w.write_all(enc.encode(&data).as_bytes())?;
        },
        Command::Decode { output, input } => {
            info!("cde: decoding from {} to {}",
                reader_name(&input)?.to_string_lossy(),
                writer_name(&output)?.to_string_lossy());

            let mut r = reader(&input)?;
            let mut w = writer(&output)?;

            // read in everything and split the tag from the data
            let mut s = Vec::<u8>::new();
            let _ = r.read_to_end(&mut s)?;
            let dec = encoder()?;
            let mut tag = dec.decode(&s)?;
            let data = if tag[1] & 0x08 != 0 {
                tag.split_off(6)
            } else {
                tag.split_off(3)
            };

            let tt = TagBuilder::from_bytes(tag).build();
            debug!("\n{:?}", tt);

            w.write_all(&data)?;
        }
        Command::Info { input } => {
            debug!("cde: reading info from {}",
                reader_name(&input)?.to_string_lossy());

            let mut r = reader(&input)?;
            let mut w = writer(&None)?;

            // read in everything and split the tag from the data
            let mut s = Vec::<u8>::new();
            let _ = r.read_to_end(&mut s)?;
            let dec = encoder()?;
            let mut tag = dec.decode(&s)?;
            let _ = if tag[1] & 0x08 != 0 {
                tag.split_off(6)
            } else {
                tag.split_off(3)
            };

            let tt = TagBuilder::from_bytes(tag).build();
            debug!("\n{:?}", tt);

            w.write_all(format!("\n{:?}", tt).as_bytes())?;
        }
        Command::Tt { input } => {
            debug!("cde: reading info from {}",
                reader_name(&input)?.to_string_lossy());

            let mut r = reader(&input)?;
            let mut w = writer(&None)?;

            // read in everything and split the tag from the data
            let mut s = Vec::<u8>::new();
            let _ = r.read_to_end(&mut s)?;
            let dec = encoder()?;
            let mut tag = dec.decode(&s)?;
            let _ = if tag[1] & 0x08 != 0 {
                tag.split_off(6)
            } else {
                tag.split_off(3)
            };

            let tt = TagBuilder::from_bytes(tag).build();
            debug!("\n{:?}", tt);

            w.write_all(format!("\n{}\n", tt).as_bytes())?;
        }
    }

    Ok(())
}
