use crate::{idx, Error, CDE_ALPHABET, ENCODER, CryptoData, Result};
use std::fmt::{self, Debug, Display, Formatter};
use std::str::FromStr;

// include the generated hashmaps
include!(concat!(env!("OUT_DIR"), "/hashmaps.rs"));

static NUMBERS: &'static str = "0123456789";
static UNDEFINED: &'static str = "undefined";

#[derive(Default)]
pub struct Tag {
    b: [u8; 6],
    e: [u8; 8],
}

impl Tag {

    pub(crate) fn update_encoding(&mut self) {
        if self.is_extended() {
            // encode all 6 bytes
            ENCODER.encode_mut(&self.b, &mut self.e)
        } else {
            // only encode the first 3 bytes
            ENCODER.encode_mut(&self.b[..3], &mut self.e[..4])
        }
    }

    pub fn is_extended(&self) -> bool {
        (self.b[1] & 0x08) != 0
    }

    pub fn set_length(&mut self, len: u32) {
        if len > 255 {
            // store the length
            self.b[2..].copy_from_slice(&len.to_be_bytes());

            // set the extended length bit
            self.b[1] |= 0x08;
        } else {
            // store the length
            self.b[2] = len as u8;

            // clear the extended length bit
            self.b[1] &= 0xF7;
        }
        self.update_encoding();
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
        self.update_encoding();
    }

    pub fn encode_len(&self) -> usize {
        if self.is_extended() {
            8
        } else {
            4
        }
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.b
    }

    pub fn as_str(&self) -> &str {
        if self.is_extended() {
            // return the full 8 bytes as &str
            core::str::from_utf8(&self.e).unwrap()
        } else {
            // only the first 4 bytes as &str
            core::str::from_utf8(&self.e[..4]).unwrap()
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
                        println!("found sub-sub-class: {}", ssc);
                        return Ok((c, sc, Some(*ssc)));
                    } else {
                        println!("using sub-sub-class number instead");
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
        self.b[1] & 0x07
    }
}

impl CryptoData for Tag {
    pub fn len(&self) -> usize {
        if self.is_extended() {
            let mut buf: [u8; 4] = [0; 4];
            buf.copy_from_slice(&self.b[2..]);
            u32::from_be_bytes(buf) as usize
        } else {
            self.b[2] as usize
        }
    }

    pub fn encode_len(&self) -> usize {
        ENCODER.encode_len(self.len())
    }

    pub fn encode(&self, encoded: &mut [u8]) {
        if self.is_extended() {
            // encode all 6 bytes
            ENCODER.encode_mut(&self.b, encoded)
        } else {
            // only encode the first 3 bytes
            ENCODER.encode_mut(&self.b[..3], encoded)
        }
    }
}

enum TagBuildFrom {
    Str,
    Byt
}

pub struct TagBuilder<'a> {
    how: TagBuildFrom,
    s: Option<&'a str>,
    b: Option<&'a [u8]>,
    l: u32
}

impl Default for TagBuilder<'_> {
    fn default() -> Self {
        TagBuilder {
            how: TagBuildFrom::Str,
            s: None,
            b: None,
            l: 0u32,
        }
    }
}

impl<'a> TagBuilder<'a> {
    pub fn from_str(s: &'a str) -> Self {
        TagBuilder {
            how: TagBuildFrom::Str,
            s: Some(s),
            b: None,
            l: 0u32,
        }
    }

    pub fn from_bytes(b: &'a [u8]) -> Self {
        TagBuilder {
            how: TagBuildFrom::Byt,
            s: None,
            b: Some(b),
            l: 0u32,
        }
    }

    pub fn length(mut self, l: u32) -> Self {
        self.l = l;
        self
    }

    pub fn build(self) -> Result<Tag>  {
        let mut tag = match self.how {
            TagBuildFrom::Str => {
                match self.s {
                    None => return Err(Error::TagFromStr),
                    Some(s) =>  {
                        Tag::from_str(s)?
                    }
                }
            },
            TagBuildFrom::Byt => {
                match self.b {
                    None => return Err(Error::TagFromBytes),
                    Some(s) => {
                        Tag::from(s)
                    }
                }
            }
        };

        tag.set_length(self.l);
        Ok(tag)
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

impl<T: AsRef<[u8]>> From<T> for Tag {
    fn from(v: T) -> Self {
        let mut tag = Tag::default();
        if v.as_ref().len() >= 6 {
            tag.b.copy_from_slice(&v.as_ref()[..6]);
        } else if v.as_ref().len() >= 3 {
            tag.b[..3].copy_from_slice(&v.as_ref()[..3]);
        }

        // encode the bytes
        tag.update_encoding();

        //println!("{:08b} {:08b} {:08b} {:08b} {:08b} {:08b}",
        //         tag.t[0], tag.t[1], tag.t[2], tag.t[3], tag.t[4], tag.t[5]);

        tag
    }
}

impl FromStr for Tag {
    type Err = Error;

    /// This takes a tag string name like "key.ed25519.public" and parses it
    /// into a Tag containing the correct class, sub-class, and sub-sub-class
    /// values. The length is initiatlized to zero.
    fn from_str(tag: &str) -> Result<Self> {

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
                //println!("have {}, {}, {}...", c_name, sc_name, ssc_name);
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
                //println!("have {}, {}...", c_name, sc_name);
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
            _ => return Err(Error::TagFromStr),
        };

        //println!("have values: {}, {}, {}...", c, sc, ssc);

        // pack the type values into a byte buffer
        let buf: [u8; 3] = [
            ((((c & 0x3f) << 2) & 0xfc) | (((sc & 0x30) >> 4) & 0x03)),
            ((((sc & 0x0f) << 4) & 0xf0) | (ssc & 0x07)),
            0
        ];

        //println!("{:08b} {:08b} {:08b}", buf[0], buf[1], buf[2]);

        // construct the tag from the bytes
        Ok(Tag::from(buf))
    }
}

impl Debug for Tag {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut b = [0u8; 8];
        self.encode(&mut b);
        let s = core::str::from_utf8(&mut b).unwrap();
        let mut i = s.chars();
        let (cn, scn, sscn) = self.name().unwrap();
        let sscn = match sscn {
            None => "None",
            Some(sscn) => sscn
        };
        if self.is_extended() {
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
            writeln!(f, "||   | ||   | || ||                                    |")?;
            writeln!(f, "||   | ||   | || |+------------------------------------+.. len: {}", self.get_length())?;
            writeln!(f, "||   | ||   | |+-+........................................ sub-sub-class: {}", sscn)?;
            writeln!(f, "||   | ||   | +........................................... ext. length: {}", (idx(c[2]) > 31) as bool)?;
            writeln!(f, "||   | |+---+............................................. sub-class: {}", scn)?;
            writeln!(f, "||   | +.................................................. exp. sub-class: {}", (idx(c[1]) > 31) as bool)?;
            writeln!(f, "|+---+.................................................... class: {}", cn)?;
            writeln!(f, "+......................................................... exp. class: {}", (idx(c[0]) > 31) as bool)?;
        } else {
            let c: [char; 4] = [i.next().unwrap(), i.next().unwrap(),
                                i.next().unwrap(), i.next().unwrap()];

            writeln!(f, "       encoding unit 1")?;
            writeln!(f, " /--------------------------/")?;
            writeln!(f, "/--{}--//--{}--//--{}--//--{}--/",
                        c[0], c[1], c[2], c[3])?;
            writeln!(f, "{:06b} {:06b} {:06b} {:06b}",
                        idx(c[0]), idx(c[1]), idx(c[2]), idx(c[3]))?;
            writeln!(f, "||   | ||   | || ||       |")?;
            writeln!(f, "||   | ||   | || |+-------+.. len: {}", self.get_length())?;
            writeln!(f, "||   | ||   | |+-+........... sub-sub-class: {}", sscn)?;
            writeln!(f, "||   | ||   | +.............. ext. length: {}", (idx(c[2]) > 31) as bool)?;
            writeln!(f, "||   | |+---+................ sub-class: {}", scn)?;
            writeln!(f, "||   | +..................... exp. sub-class: {}", (idx(c[1]) > 31) as bool)?;
            writeln!(f, "|+---+....................... class: {}", cn)?;
            writeln!(f, "+............................ exp. class: {}", (idx(c[0]) > 31) as bool)?;
        }
        Ok(())
    }
}


mod tests {
}
