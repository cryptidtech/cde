extern crate structopt;

use cde::{ CryptoData, Error, ENCODER, Result, TagBuilder };
use log::*;
use std::ffi::OsString;
use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "cde",
    version = "0.3.0",
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
        .init()
        .map_err(|_| Error::GeneralError)?;

    debug!("cde: verbosity set to: {}", opt.verbosity);

    match opt.cmd {
        Command::Encode { output, tt, input } => {
            info!("cde: encoding from {} to {}",
                reader_name(&input)?.to_string_lossy(),
                writer_name(&output)?.to_string_lossy());

            let mut r = reader(&input)?;
            let mut w = writer(&output)?;
            let mut tmp = tempfile::tempfile()?;

            let mut decoded = [0u8; 3 * 1024];
            let mut encoded = [0u8; 4 * 1024];
            let mut len = 0;
            loop {
                let n = r.read(&mut decoded)?;
                if n == 0 {
                    break;
                } else {
                    len += n;
                    ENCODER.encode_mut(&decoded[0..n], &mut encoded[0..ENCODER.encode_len(n)]);
                }
                tmp.write_all(&encoded[0..ENCODER.encode_len(n)])?;
            }
            tmp.seek(SeekFrom::Start(0))?;

            // generate a type tag from the command line options
            let mut tt = TagBuilder::from_tag(&tt).build()?;
            debug!("\n{:?}", tt);

            // set the data length on the tag
            tt.set_data_length(len);

            // write the encoded type tag
            let mut b = [0u8; 12];
            let len = tt.encode(&mut b);
            w.write_all(&b[0..len])?;

            // copy the encoded data from the tmpfile to the output
            io::copy(&mut tmp, &mut w)?;
        },
        Command::Decode { output, input } => {
            info!("cde: decoding from {} to {}",
                reader_name(&input)?.to_string_lossy(),
                writer_name(&output)?.to_string_lossy());

            let mut r = reader(&input)?;
            let mut w = writer(&output)?;

            // read in the encoded tag
            let mut encoded = [0u8; 12];
            let mut decoded = [0u8; 9];
            r.read_exact(&mut encoded[0..4])?;
            ENCODER.decode_mut(&encoded[0..4], &mut decoded[0..3]).map_err(|_| Error::DecodeError)?;
            if decoded[2] & 0x80 != 0 {
                r.read_exact(&mut encoded[4..8])?;
                ENCODER.decode_mut(&encoded[4..8], &mut decoded[3..6]).map_err(|_| Error::DecodeError)?;
            }
            if decoded[5] & 0x80 != 0 {
                r.read_exact(&mut encoded[8..12])?;
                ENCODER.decode_mut(&encoded[8..12], &mut decoded[6..9]).map_err(|_| Error::DecodeError)?;
            }

            // decode the tag first...
            let tag = TagBuilder::from_bytes(&decoded).build()?;

            // debug output the tag
            debug!("\n{:?}", tag);

            // decode the file
            let mut left = tag.get_data_length() as usize;
            let mut buf = [0u8; 4 * 1024];
            while left > 0  {
                // read in up to 4KB of encoded data
                let len = r.read(&mut buf)?;

                // decode it to 3KB of decoded data
                let data = ENCODER.decode(&buf).map_err(|_| Error::DecodeError)?;

                // write 3KB to the output
                w.write_all(&data)?;

                // go until we are out of data to read
                left -= len;
            }

            // read in everything and decode the tag
            let mut s = Vec::<u8>::new();
            let _ = r.read_to_end(&mut s)?;

            // decode the tag first...
            let tag = TagBuilder::from_encoded(&s).build()?;

            // remove the encoded tag
            let s = s.split_off(tag.encode_len());

            // decode the data portion
            let data = ENCODER.decode(&s).map_err(|_| Error::DecodeError)?;

            // debug output the tag
            debug!("\n{:?}", tag);

            // write out the decoded data
            w.write_all(&data)?;
        }
        Command::Info { input } => {
            debug!("cde: reading info from {}",
                reader_name(&input)?.to_string_lossy());

            let mut r = reader(&input)?;
            let mut w = writer(&None)?;

            // read in the encoded tag
            let mut encoded = [0u8; 12];
            let mut decoded = [0u8; 9];
            r.read_exact(&mut encoded[0..4])?;
            ENCODER.decode_mut(&encoded[0..4], &mut decoded[0..3]).map_err(|_| Error::DecodeError)?;
            if decoded[2] & 0x80 != 0 {
                r.read_exact(&mut encoded[4..8])?;
                ENCODER.decode_mut(&encoded[4..8], &mut decoded[3..6]).map_err(|_| Error::DecodeError)?;
            }
            if decoded[5] & 0x80 != 0 {
                r.read_exact(&mut encoded[8..12])?;
                ENCODER.decode_mut(&encoded[8..12], &mut decoded[6..9]).map_err(|_| Error::DecodeError)?;
            }

            // decode the tag first...
            let tag = TagBuilder::from_bytes(&decoded).build()?;

            debug!("\n{:?}", tag);

            // write the tag out
            w.write_all(format!("\n{:?}", tag).as_bytes())?;
        }
        Command::Tt { input } => {
            debug!("cde: reading info from {}",
                reader_name(&input)?.to_string_lossy());

            let mut r = reader(&input)?;
            let mut w = writer(&None)?;

            // read in the encoded tag
            let mut encoded = [0u8; 12];
            let mut decoded = [0u8; 9];
            r.read_exact(&mut encoded[0..4])?;
            ENCODER.decode_mut(&encoded[0..4], &mut decoded[0..3]).map_err(|_| Error::DecodeError)?;
            if decoded[2] & 0x80 != 0 {
                r.read_exact(&mut encoded[4..8])?;
                ENCODER.decode_mut(&encoded[4..8], &mut decoded[3..6]).map_err(|_| Error::DecodeError)?;
            }
            if decoded[5] & 0x80 != 0 {
                r.read_exact(&mut encoded[8..12])?;
                ENCODER.decode_mut(&encoded[8..12], &mut decoded[6..9]).map_err(|_| Error::DecodeError)?;
            }

            // decode the tag first...
            let tag = TagBuilder::from_bytes(&decoded).build()?;

            debug!("\n{:?}", tag);

            // write the tag out
            w.write_all(format!("\n{}\n", tag).as_bytes())?;
        }
    }

    Ok(())
}
