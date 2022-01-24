use crate::{CDE_ALPHABET, CryptoData, ENCODER, Error, idx, Result, VarUInt};
use std::fmt::{self, Debug, Display, Formatter};

// include the generated hashmaps
include!(concat!(env!("OUT_DIR"), "/hashmaps.rs"));

static NUMBERS: &'static str = "0123456789";
static UNDEFINED: &'static str = "undefined";

#[derive(Clone, Copy, Default, PartialEq)]
pub struct Tag {
    b: [u8; 2],
    l: VarUInt,
}

impl Tag {

    pub(crate) fn new(b: &[u8]) -> Self {
        let mut t = Tag::default();
        t.b.copy_from_slice(&b[0..2]);
        t.l = VarUInt::from(&b[2..]);
        t
    }

    pub fn set_data_length(&mut self, len: u64) {
        self.l = VarUInt::from(len);
    }

    pub fn get_data_length(&self) -> u64 {
        self.l.into()
    }

    pub fn is_exp_class(&self) -> bool {
        (self.b[0] & 0x80) != 0
    }

    pub fn is_exp_sub_class(&self) -> bool {
        (self.b[0] & 0x02) != 0
    }

    pub fn set_exp_class(&mut self, exp: bool) {
        if exp {
            self.b[0] |= 0x80;
        } else {
            self.b[0] &= 0xf7;
        }
    }

    pub fn set_exp_sub_class(&mut self, exp: bool) {
        if exp {
            self.b[0] |= 0x02;
        } else {
            self.b[0] &= 0xfd;
        }
    }

    pub fn name(&self) -> Result<(&str, &str, Option<&str>)> {
        let n: [u8; 3] = [self.class(), self.subclass(), self.subsubclass()];
        let i: [usize; 3] = [n[0] as usize, n[1] as usize, n[2] as usize];

        if let Some((c, sc_map)) = NAMES.get(&n[0]) {
            if let Some((sc, ssc_map)) = sc_map.get(&n[1]) {
                if let Some(ssc_map) = ssc_map {
                    //NOTE: due to a bug in the phf maps, we cannot use 0-indexing
                    //so we use 1-indexing instead as a work-around
                    if let Some(ssc) = ssc_map.get(&(n[2] + 1)) {
                        return Ok((c, sc, Some(*ssc)));
                    } else {
                        return Ok((c, sc, NUMBERS.get(i[2]..i[2]+1)));
                    }
                } else {
                    return Ok((c, sc, NUMBERS.get(i[2]..i[2]+1)));
                } 
            } else if let Some(sc) = CDE_ALPHABET.get(i[1]..i[1]+1) {
                return Ok((c, sc, None));
            } else {
                return Ok((c, UNDEFINED, None));
            }
        } else if let Some(c) = CDE_ALPHABET.get(i[0]..i[0]+1) {
            if let Some(sc) = CDE_ALPHABET.get(i[1]..i[1]+1) {
                return Ok((c, sc, None));
            } else {
                return Ok((c, UNDEFINED, None));
            }
        } else {
            return Ok((UNDEFINED, UNDEFINED, None));
        }
    }

    pub fn class(&self) -> u8 {
        ((self.b[0] & 0xfc) >> 2) & 0x3f
    }

    pub fn subclass(&self) -> u8 {
        (((self.b[0] & 0x03) << 4) | ((self.b[1] & 0xf0) >> 4)) & 0x3f
    }

    pub fn subsubclass(&self) -> u8 {
        self.b[1] & 0x0f
    }

}

impl CryptoData for Tag {
    fn len(&self) -> usize {
        2 + self.l.len()
    }

    fn bytes(&self, buf: &mut [u8]) -> usize {
        buf[0..2].copy_from_slice(&self.b);
        self.l.bytes(&mut buf[2..self.len()]);
        self.len()
    }

    fn encode_len(&self) -> usize {
        ENCODER.encode_len(self.len())
    }

    fn encode(&self, buf: &mut [u8]) -> usize {
        let mut b = [0u8; 9];
        let len = self.bytes(&mut b);
        ENCODER.encode_mut(&b[0..len], &mut buf[0..self.encode_len()]);
        self.encode_len()
    }
}

enum TagBuildFrom {
    Tag,
    Bytes,
    Encoded,
}

pub struct TagBuilder<'a> {
    how: TagBuildFrom,
    tag: Option<&'a str>,
    bytes: Option<&'a [u8]>,
}

// create a tag in the provided buffer copying from the bytes slice
// let tt = TagBuilder::from_bytes(&bytes).build().unwrap();
//
// create a tag in the provided buffer from the type string
// let tt = TagBuilder::from_tag("key.p256.secret").build().unwrap();

impl<'a> TagBuilder<'a> {
    pub fn from_tag(s: &'a str) -> Self {
        TagBuilder {
            how: TagBuildFrom::Tag,
            tag: Some(s),
            bytes: None,
        }
    }

    pub fn from_bytes(b: &'a [u8]) -> Self {
        TagBuilder {
            how: TagBuildFrom::Bytes,
            tag: None,
            bytes: Some(b),
        }
    }

    pub fn from_encoded(e: &'a [u8]) -> Self {
        TagBuilder {
            how: TagBuildFrom::Encoded,
            tag: None,
            bytes: Some(e),
        }
    }

    pub fn build(&self) -> Result<Tag>  {
        let mut buf = [0u8; 9];
        let tag = match self.how {
            TagBuildFrom::Tag => {
                if let Some(tag) = self.tag {
                    TagBuilder::decode_str(tag, &mut buf)?;
                    Tag::new(&buf)
                } else {
                    return Err(Error::FromStr);
                }
            },
            TagBuildFrom::Bytes => {
                if let Some(bytes) = self.bytes {
                    Tag::new(&bytes)
                } else {
                    return Err(Error::FromBytes);
                }
            },
            TagBuildFrom::Encoded => {
                if let Some(bytes) = self.bytes {
                    ENCODER.decode_mut(&bytes[0..4], &mut buf[0..3]).map_err(|_| Error::DecodeError)?;
                    if (buf[2] & 0x80) != 0 {
                        ENCODER.decode_mut(&bytes[4..8], &mut buf[3..6]).map_err(|_| Error::DecodeError)?;
                    }
                    if (buf[5] & 0x80) != 0 {
                        ENCODER.decode_mut(&bytes[8..12], &mut buf[6..9]).map_err(|_| Error::DecodeError)?;
                    }
                    Tag::new(&buf)
                } else {
                    return Err(Error::DecodeError);
                }
            }
        };

        Ok(tag)
    }

    /// This takes a tag string name like "key.ed25519.public" and parses it
    /// into a Tag containing the correct class, sub-class, and sub-sub-class
    /// values. The length is initiatlized to zero.
    fn decode_str(tag: &str, buf: &mut [u8]) -> Result<()> {

        // checks if the value is experimental
        fn experimental(v: u8) -> bool {
            (v > 31) && (v != 63)
        }

        /// If the str is a single character
        fn name_or_char(v: &str) -> Option<u8> {
            if v.len() >= 1 {
                if let Some(c) = v.chars().next() {
                    if let Some(c) = CDE_ALPHABET.find(c) {
                        return Some(c as u8);
                    }
                }
            }
            None
        }

        let parts = match tag.len() {
            0 => (None, None, None),
            _ => {
                let mut s = tag.split('.');
                (s.next(), s.next(), s.next())
            },
        };

        let (c, sc, ssc) = match parts {
            (Some(c_name), Some(sc_name), Some(ssc_name)) => {
                match VALUES.get(c_name) {
                    None => {
                        if let Some(c) = name_or_char(c_name) {
                            // if we get here they specified a non-standard...
                            if !experimental(c) {
                                // ...it must be experimental or it is an error...
                                return Err(Error::InvalidClass);
                            } else if let Some(sc) = name_or_char(sc_name) {
                                if !experimental(sc) {
                                    // ...the sub-class must be experimental or it is an error
                                    return Err(Error::InvalidSubClass);
                                } else if let Ok(ssc) = u8::from_str_radix(ssc_name, 10) {
                                    // ...both class and sub-class are experimental so 
                                    // also return the sub-sub-class number
                                    (c, sc, ssc)
                                } else {
                                    // ...the sub-sub-class was not a base 10 number
                                    return Err(Error::InvalidSubSubClass);
                                }
                            } else {
                                // ...the sub-class value wasn't a string
                                return Err(Error::InvalidSubClass);
                            }
                        } else {
                            // .. the class value wasn't a string
                            return Err(Error::InvalidClass);
                        }
                    },
                    Some((c, sc_map)) => {
                        // the class name was a standard class name
                        match sc_map.get(sc_name) {
                            None => {
                                // the sub-class was not a standard sub-class name
                                if !experimental(*c) {
                                    // ...the class must be experimental or it is an error
                                    return Err(Error::InvalidClass);
                                } else if let Some(sc) = name_or_char(sc_name) {
                                    if !experimental(sc) {
                                        // ...the sub-class must be experimental or it is an error
                                        return Err(Error::InvalidSubClass)
                                    } else if let Ok(ssc) = u8::from_str_radix(ssc_name, 10) {
                                        // ...both class and sub-class are experimental so
                                        // also return the sub-sub-class number
                                        (*c, sc, ssc)
                                    } else {
                                        // ...the sub-sub-class was not a base 10 number
                                        return Err(Error::InvalidSubSubClass);
                                    }
                                } else {
                                    // ...the sub-class value wasn't a string
                                    return Err(Error::InvalidSubClass);
                                }
                            },
                            Some((sc, ssc_map)) => {
                                // the sub-class name was a standard class name
                                match ssc_map {
                                    None => {
                                        // there are no sub-sub-classes for this class and
                                        // sub-class combination
                                        if experimental(*c) {
                                            if experimental(*sc) {
                                                // both the class and sub-class are standard
                                                // and experimental so just return them with
                                                // the experimental sub-sub-class
                                                if let Ok(ssc) = u8::from_str_radix(ssc_name, 10) {
                                                    (*c, *sc, ssc)
                                                } else {
                                                    // the sub-sub-class was not a base 10 number
                                                    return Err(Error::InvalidSubSubClass);
                                                }
                                            } else {
                                                // an experimental class with a non-experimental
                                                // sub-class is an error
                                                return Err(Error::InvalidSubClass);
                                            }
                                        } else {
                                            // this is a standard class and standard sub-class
                                            // without any standard sub-sub-classes so the
                                            // sub-class must be experimental
                                            if !experimental(*sc) {
                                                // there is a special corner case to take into
                                                // account here... both "undefined" ('_') and list
                                                // ('-') are not considered experimental but we
                                                // allow list.list, undefined.list, and
                                                // undefined.undefined to have sub-sub-classes set
                                                // so that user can have different kinds of these
                                                // types
                                                if (*c == idx('_') && (*sc == idx('_') || *sc == idx('-'))) ||
                                                   (*c == idx('-') && *sc == idx('-')) {
                                                       if let Ok(ssc) = u8::from_str_radix(ssc_name, 10) {
                                                           (*c, *sc, ssc)
                                                       } else {
                                                           return Err(Error::InvalidSubSubClass);
                                                       }
                                                } else {
                                                    return Err(Error::InvalidSubClass);
                                                }
                                            } else if let Ok(ssc) = u8::from_str_radix(ssc_name, 10) {
                                                // the sub-class is experimental so just get the
                                                // sub-sub-class number and return all three
                                                (*c, *sc, ssc)
                                            } else {
                                                // the sub-sub-class was not a base 10 number
                                                return Err(Error::InvalidSubSubClass);
                                            }
                                        }
                                    },
                                    Some(ssc_map) => {
                                        // there are sub-sub-classes for this class and
                                        // sub-class combination
                                        if experimental(*c)  {
                                            if experimental(*sc) {
                                                // both the class and sub-class are standard
                                                // and experimental so just return them with
                                                // the experimental sub-sub-class
                                                match ssc_map.get(ssc_name) {
                                                    None => {
                                                        if let Ok(ssc) = u8::from_str_radix(ssc_name, 10) {
                                                            (*c, *sc, ssc)
                                                        } else {
                                                            return Err(Error::InvalidSubSubClass);
                                                        }
                                                    },
                                                    // subtract 1 from the sub-sub-class number
                                                    // because of a bug in the phf map we had to
                                                    // use 1-indexed maps instead of 0-indexed maps
                                                    Some(ssc) => (*c, *sc, *ssc-1),
                                                }
                                            } else {
                                                // an experimental class with a non-experimental
                                                // sub-class is an error
                                                return Err(Error::InvalidSubClass);
                                            }
                                        } else {
                                            // the class is not experimental so it doesn't matter
                                            // if the sub-class is experimental or not...just get
                                            // the sub-sub-class number and return all three
                                            match ssc_map.get(ssc_name) {
                                                None => {
                                                    if let Ok(ssc) = u8::from_str_radix(ssc_name, 10) {
                                                        (*c, *sc, ssc)
                                                    } else {
                                                        return Err(Error::InvalidSubSubClass);
                                                    }
                                                },
                                                // subtract 1 from the sub-sub-class number
                                                // because of a bug in the phf map we had to
                                                // use 1-indexed maps instead of 0-indexed maps
                                                Some(ssc) => (*c, *sc, *ssc-1)
                                            }
                                        }
                                    },
                                }
                            },
                        }
                    },
                }
            },

            (Some(c_name), Some(sc_name), None) => {
                match VALUES.get(c_name) {
                    None => {
                        // the class is non-standard...
                        if let Some(c) = name_or_char(c_name) {
                            if !experimental(c) {
                                // ...it must be experimental or it is an error
                                return Err(Error::InvalidClass);
                            } else if let Some(sc) = name_or_char(sc_name) {
                                if !experimental(sc) {
                                    // ...and therefore the sub-class must also be
                                    // experimental or it is an error
                                    return Err(Error::InvalidSubClass);
                                } else {
                                    // it is OK to specify an experimental non-standard class and
                                    // experimental non-standard sub-class without a sub-sub-class.
                                    // the sub-sub-class just defaults to 0
                                    (c, sc, 0)
                                }
                            } else {
                                // the sub-class name wasn't a string
                                return Err(Error::InvalidSubClass);
                            }
                        } else {
                            // the class name wasn't a string
                            return Err(Error::InvalidClass);
                        }
                    },
                    Some((c, sc_map)) => {
                        // the class is standard...
                        match sc_map.get(sc_name) {
                            None => {
                                // ...the sub-class name is non-standard...
                                if !experimental(*c) {
                                    // ...the class must be experimental or it is an error...
                                    return Err(Error::InvalidClass);
                                } else if let Some(sc) = name_or_char(sc_name) {
                                    if !experimental(sc) {
                                        // ...the sub-class must also be experimental
                                        // or it is an error
                                        return Err(Error::InvalidSubClass);
                                    } else {
                                        // it is OK to specify an experimental standard class and
                                        // an experimental non-standard sub-class without
                                        // specifying a sub-sub-class. the sub-sub-class just
                                        // defaults to 0
                                        (*c, sc, 0)
                                    }
                                } else {
                                    // ...the sub-class name wasn't a string
                                    return Err(Error::InvalidSubClass);
                                }
                            },
                            Some((sc, ssc_map)) => {
                                // ...the sub-class name is standard...
                                match ssc_map {
                                    None => {
                                        // standard class and standard sub-class without any
                                        // standard sub-sub-classes are allowed
                                        (*c, *sc, 0)
                                    },
                                    Some(_) => {
                                        // if we get here, there is a sub-sub-class
                                        // map and they didn't specify which sub-sub-class
                                        // this is an error
                                        return Err(Error::InvalidSubSubClass);
                                    },
                                }
                            },
                        }
                    },
                }
            },

            (Some(_), None, None) => {
                // there is no valid case where just the class is specified
                return Err(Error::InvalidSubClass);
            },

            (None, None, None) => {
                // there is no valid case where nothing is specified...
                // this case is triggered if the string is the empty string
                return Err(Error::InvalidClass);
            },

            // everything other combination is invalid...
            _ => return Err(Error::FromStr),
        };

        buf[0] = (((c & 0x3f) << 2) & 0xfc) | (((sc & 0x30) >> 4) & 0x03);
        buf[1] = (((sc & 0x0f) << 4) & 0xf0) | (ssc & 0x07);
        buf[2] = 0;

        Ok(())
    }
}

impl Display for Tag {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if let Ok((c, sc, ssc)) = self.name() {
            if let Some(ssc) = ssc {
                write!(f, "{}.{}.{}", c, sc, ssc)
            } else {
                write!(f, "{}.{}", c, sc)
            }
        } else {
            Err(core::fmt::Error)
        }
    }
}

impl Debug for Tag {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut b = [0u8; 12];
        self.encode(&mut b);
        let s = core::str::from_utf8(&mut b).unwrap();
        let mut i = s.chars();
        let (cn, scn, sscn) = self.name().unwrap();
        let sscn = match sscn {
            None => "None",
            Some(sscn) => sscn
        };
        match self.len() {
            3 => {
                let c: [char; 4] = [i.next().unwrap(), i.next().unwrap(),
                                    i.next().unwrap(), i.next().unwrap()];

                writeln!(f, "       encoding unit 1")?;
                writeln!(f, " /--------------------------/")?;
                writeln!(f, "/--{}--//--{}--//--{}--//--{}--/",
                            c[0], c[1], c[2], c[3])?;
                writeln!(f, "{:06b} {:06b} {:06b} {:06b}",
                            idx(c[0]), idx(c[1]), idx(c[2]), idx(c[3]))?;
                writeln!(f, "||   | ||   | |  ||       |")?;
                writeln!(f, "||   | ||   | |  |+-------+.. len: {}", self.get_data_length())?;
                writeln!(f, "||   | ||   | +--+........... sub-sub-class: {}", sscn)?;
                writeln!(f, "||   | |+---+................ sub-class: {}", scn)?;
                writeln!(f, "||   | +..................... exp. sub-class: {}", self.is_exp_sub_class())?;
                writeln!(f, "|+---+....................... class: {}", cn)?;
                writeln!(f, "+............................ exp. class: {}", self.is_exp_class())?;
            },
            6 => {
                let c: [char; 8] = [i.next().unwrap(), i.next().unwrap(),
                                    i.next().unwrap(), i.next().unwrap(),
                                    i.next().unwrap(), i.next().unwrap(),
                                    i.next().unwrap(), i.next().unwrap()];

                writeln!(f, "       encoding unit 1          optional encoding unit 2")?;
                writeln!(f, " /--------------------------/ /--------------------------/")?;
                writeln!(f, "/--{}--//--{}--//--{}--//--{}--/ /--{}--//--{}--//--{}--//--{}--/",
                            c[0], c[1], c[2], c[3], c[4], c[5], c[6], c[7])?;
                writeln!(f, "{:06b} {:06b} {:06b} {:06b}  {:06b} {:06b} {:06b} {:06b}",
                            idx(c[0]), idx(c[1]), idx(c[2]), idx(c[3]), idx(c[4]), idx(c[5]), idx(c[6]), idx(c[7]))?;
                writeln!(f, "||   | ||   | |  ||                                    |")?;
                writeln!(f, "||   | ||   | |  |+-------+--+-------++-------++-------+")?;
                writeln!(f, "||   | ||   | |  |        |.. len: {}", self.get_data_length())?;
                writeln!(f, "||   | ||   | +--+........... sub-sub-class: {}", sscn)?;
                writeln!(f, "||   | |+---+................ sub-class: {}", scn)?;
                writeln!(f, "||   | +..................... exp. sub-class: {}", self.is_exp_sub_class())?;
                writeln!(f, "|+---+....................... class: {}", cn)?;
                writeln!(f, "+............................ exp. class: {}", self.is_exp_class())?;
            },
            9 => {
                let c: [char; 12] = [i.next().unwrap(), i.next().unwrap(),
                                     i.next().unwrap(), i.next().unwrap(),
                                     i.next().unwrap(), i.next().unwrap(),
                                     i.next().unwrap(), i.next().unwrap(),
                                     i.next().unwrap(), i.next().unwrap(),
                                     i.next().unwrap(), i.next().unwrap()];

                writeln!(f, "       encoding unit 1          optional encoding unit 2     optional encoding unit 3")?;
                writeln!(f, " /--------------------------/ /--------------------------/ /--------------------------/")?;
                writeln!(f, "/--{}--//--{}--//--{}--//--{}--/ /--{}--//--{}--//--{}--//--{}--/ /--{}--//--{}--//--{}--//--{}--/",
                            c[0], c[1], c[2], c[3], c[4], c[5], c[6], c[7], c[8], c[9], c[10], c[11])?;
                writeln!(f, "{:06b} {:06b} {:06b} {:06b}  {:06b} {:06b} {:06b} {:06b}  {:06b} {:06b} {:06b} {:06b}",
                            idx(c[0]), idx(c[1]), idx(c[2]), idx(c[3]),
                            idx(c[4]), idx(c[5]), idx(c[6]), idx(c[7]),
                            idx(c[8]), idx(c[9]), idx(c[10]), idx(c[11]))?;
                writeln!(f, "||   | ||   | |  ||                                                                  |")?;
                writeln!(f, "||   | ||   | |  |+-------+--+-------++-------++-------++--------++--------++--------+")?;
                writeln!(f, "||   | ||   | |  |        |.. len: {}", self.get_data_length())?;
                writeln!(f, "||   | ||   | +--+........... sub-sub-class: {}", sscn)?;
                writeln!(f, "||   | |+---+................ sub-class: {}", scn)?;
                writeln!(f, "||   | +..................... exp. sub-class: {}", self.is_exp_sub_class())?;
                writeln!(f, "|+---+....................... class: {}", cn)?;
                writeln!(f, "+............................ exp. class: {}", self.is_exp_class())?;
            },
            _ => { return Err(fmt::Error); },
        }
        Ok(())
    }
}
