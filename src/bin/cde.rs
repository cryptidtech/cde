extern crate structopt;
#[macro_use]
extern crate vlog;

use cde::*;
use anyhow::Result;
use std::ffi::OsString;
use std::fs::File;
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use structopt::StructOpt;
use vlog::set_verbosity_level;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "cde",
    version = "0.1",
    author = "David Huseby <dwh@linuxprogrammer.org>",
    about = "Encode/Decode cryptographic data in CDE format",
)]
struct Opt {
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

        /// The class of the encoded object
        #[structopt(short="c", long = "class")]
        class: Class,

        /// The sub-class of the encoded object
        #[structopt(short="s", long = "sub-class")]
        sub_class: SubClass,

        /// The sub-sub-class of the encoded object
        #[structopt(short="v", long = "sub-sub-class", default_value = "0")]
        sub_sub_class: SubSubClass,

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
    /// Display the type tag information for an encoded object
    Info {
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

    // set the verbosity level
    set_verbosity_level(opt.verbosity);
    v3!("cde: verbosity set to: {}", opt.verbosity);

    match opt.cmd {
        Command::Encode { output, class, sub_class, sub_sub_class, input } => {
            ve1!("cde: encoding from {} to {}",
                reader_name(&input)?.to_string_lossy(),
                writer_name(&output)?.to_string_lossy());

            let mut r = reader(&input)?;
            let mut w = writer(&output)?;

            // read in the data to encode
            let mut data = Vec::<u8>::new();
            let len = r.read_to_end(&mut data)?;

            // generate a type tag from the command line options
            let tt = TypeTag::new(&class, &sub_class, &sub_sub_class, len as u32);
            ve1!("{}", tt);

            // write the encoded type tag
            w.write_all(tt.encode().as_bytes())?;

            // write the encoded data
            let enc = encoder()?;
            w.write_all(enc.encode(&data).as_bytes())?;
        },
        Command::Decode { output, input } => {
            ve1!("cde: decoding from {} to {}",
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

            let tt = TypeTag::from(tag);
            ve1!("{}", tt);

            w.write_all(&data)?;
        }
        Command::Info { input } => {
            ve1!("cde: reading info from {}",
                reader_name(&input)?.to_string_lossy());

            let mut r = reader(&input)?;

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

            let tt = TypeTag::from(tag);
            ve0!("{}", tt);
        }
    }

    Ok(())
}
