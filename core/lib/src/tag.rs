use crate::{ ch, idx, encoder, CdeError, CDE_ALPHABET };
use log::info;
use std::collections::HashMap;
use std::fmt::{ self, Debug, Display, Formatter };
use std::str::FromStr;
use std::string::ToString;

#[derive(Default)]
pub struct Tag {
    c: u8,
    sc: u8,
    ssc: u8,
    len: u32,
    c_name: Option<String>,
    sc_name: Option<String>,
    ssc_name: Option<String>,
}

impl<T: AsRef<[u8]>> From<T> for Tag {
    fn from(v: T) -> Self {
        // get the class value
        let c = ((v.as_ref()[0] & 0xFC) >> 2) & 0x3F;

        // get the sub-class value
        let sc = (((v.as_ref()[0] & 0x03) << 4) | ((v.as_ref()[1] & 0xF0) >> 4)) & 0x3F;

        // get the sub-sub-class value
        let ssc = v.as_ref()[1] & 0x07;

        // get the length from the stream
        let len = if v.as_ref().len() > 3 {
            info!("Tag 8-bit words: {:#02x} {:#02x} {:#02x} {:#02x} {:#02x} {:#02x}",
                  v.as_ref()[0], v.as_ref()[1], v.as_ref()[2], v.as_ref()[3],
                  v.as_ref()[4], v.as_ref()[5]);
            let b: [u8; 4] = [v.as_ref()[2], v.as_ref()[3], v.as_ref()[4], v.as_ref()[5]];
            u32::from_be_bytes(b)
        } else {
            info!("Tag 8-bit words: {:#02x} {:#02x} {:#02x}", v.as_ref()[0],
                  v.as_ref()[1], v.as_ref()[2]);
            v.as_ref()[2] as u32
        };

        Tag {
            c: c,
            sc: sc,
            ssc: ssc,
            len: len,
            c_name: None,
            sc_name: None,
            ssc_name: None
        }
    }
}

fn parse_type_name(name: &String, max: u8) -> Result<u8, CdeError> {
    if !name.is_ascii() {
        // raise an error if the type name is not ascii
        return Err(CdeError::InvalidTypeName(name.to_owned()));
    }

    // we try several things to convert the supplied type name string into an
    // appropriate type value. first we try to parse a u8 that has a value <64.
    // then we look at the first letter of the name and try to get the index
    // value for the character. we raise an error if neither works.
    match u8::from_str_radix(name.as_str(), 10) {
        Ok(i) if i < max => {
            return Ok(i);
        },
        Ok(i) => {
            // they used a number but it was >= max
            return Err(CdeError::InvalidTypeNumber(i));
        },
        Err(_) => {
            match CDE_ALPHABET.find(name.chars().nth(0).unwrap() as char) {
                Some(i) => {
                    return Ok(i as u8);
                },
                None => {
                    return Err(CdeError::InvalidTypeFirstLetter(name.to_owned()));
                }
            }
        }
    }
}

impl FromStr for Tag {
    type Err = CdeError;

    fn from_str(tag: &str) -> Result<Self, Self::Err> {
        let empty = HashMap::new();
        let mut i = tag.split('.');

        // try to look up the class value by name
        let c_name = match i.next() {
            Some(c) if c.len() == 0 => "_",
            Some(c) => c,
            None => "_"
        };

        let (c_value, sc_vals) = match CDE_VALUES.get(c_name) {
            Some((v, h)) => (*v, h),
            None => { // try parsing the name into a value
                let cval = parse_type_name(&c_name.to_string(), 64)?;
                let mut tmp = [0; 4];
                let cn = ch(cval).encode_utf8(&mut tmp);
                match CDE_VALUES.get(cn) {
                    Some((v, h)) => (*v, h),
                    None => { return Err(CdeError::InvalidClass(cn.to_string())); }
                }
            }
        };

        // try to look up the sub-class value by name
        let sc_name = match i.next() {
            Some(c) if c.len() == 0 => "_",
            Some(c) => c,
            None => "_"
        };
        
        let (sc_value, ssc_vals) = match sc_vals.get(sc_name) {
            Some((v, h)) => (*v, h),
            None => { // try parsing the name into a value
                let scval = parse_type_name(&sc_name.to_string(), 64)?;
                let mut tmp = [0; 4];
                let scn = ch(scval).encode_utf8(&mut tmp);
                match sc_vals.get(scn) {
                    Some((v, h)) => (*v, h),
                    None if c_value > 31 => (scval, &empty),
                    None => { return Err(CdeError::InvalidSubClass(scn.to_string())); }
                }
            }
        };

        // try to look up the sub-sub-class value by name
        let ssc_name = match i.next() {
            Some(c) if c.len() == 0 => "0",
            Some(c) => c,
            None => "0" // it's ok if this is missing
        };

        let ssc_value = match ssc_vals.get(ssc_name) {
            Some(v) => *v,
            None => { // try parsing the name into a value
                let sscval = parse_type_name(&ssc_name.to_string(), 64)?;
                let mut tmp = [0; 4];
                let sscn = ch(sscval).encode_utf8(&mut tmp);
                match ssc_vals.get(sscn) {
                    Some(v) => *v,
                    None => sscval
                }
            }
        };

        Ok(Tag {
            c: c_value,
            sc: sc_value,
            ssc: ssc_value,
            len: 0,
            c_name: Some(c_name.to_string()),
            sc_name: Some(sc_name.to_string()),
            ssc_name: Some(ssc_name.to_string())
        })
    }
}

impl Debug for Tag {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let s = self.encode();
        let (cn, scn, sscn) = self.name();
        if self.is_extended() {
            let c: Vec<char> = s.chars().take(8).collect();
            writeln!(f, "       encoding unit 1          optional encoding unit 2")?;
            writeln!(f, " /--------------------------/ /--------------------------/")?;
            writeln!(f, "/--{}--//--{}--//--{}--//--{}--/ /--{}--//--{}--//--{}--//--{}--/",
                        c[0], c[1], c[2], c[3], c[4], c[5], c[6], c[7])?;
            writeln!(f, "{:06b} {:06b} {:06b} {:06b}  {:06b} {:06b} {:06b} {:06b}",
                        idx(c[0]), idx(c[1]), idx(c[2]), idx(c[3]), idx(c[4]), idx(c[5]), idx(c[6]), idx(c[7]))?;
            writeln!(f, "||   | ||   | || ||                                    |")?;
            writeln!(f, "||   | ||   | || |+------------------------------------+.. len: {}", self.len)?;
            writeln!(f, "||   | ||   | |+-+........................................ sub-sub-class: {}", sscn)?;
            writeln!(f, "||   | ||   | +........................................... ext. length: {}", (idx(c[2]) > 31) as bool)?;
            writeln!(f, "||   | |+---+............................................. sub-class: {}", scn)?;
            writeln!(f, "||   | +.................................................. exp. sub-class: {}", (idx(c[1]) > 31) as bool)?;
            writeln!(f, "|+---+.................................................... class: {}", cn)?;
            writeln!(f, "+......................................................... exp. class: {}", (idx(c[0]) > 31) as bool)?;
        } else {
            let c: Vec<char> = s.chars().take(4).collect();
            writeln!(f, "       encoding unit 1")?;
            writeln!(f, " /--------------------------/")?;
            writeln!(f, "/--{}--//--{}--//--{}--//--{}--/",
                        c[0], c[1], c[2], c[3])?;
            writeln!(f, "{:06b} {:06b} {:06b} {:06b}",
                        idx(c[0]), idx(c[1]), idx(c[2]), idx(c[3]))?;
            writeln!(f, "||   | ||   | || ||       |")?;
            writeln!(f, "||   | ||   | || |+-------+.. len: {}", self.len)?;
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

impl Display for Tag {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let (c, sc, ssc) = self.name();
        let s = [c, sc, ssc].join(".");
        write!(f, "{}", s)
    }
}

impl Tag {

    pub fn is_extended(&self) -> bool {
        self.len > 255
    }

    pub(crate) fn set_length(&mut self, len: u32) {
        self.len = len;
    }

    pub fn encode(&self) -> String {
        let vec = if self.is_extended() {
            let mut v: Vec<u8> = Vec::new();
            v.push((self.c << 2) | ((self.sc & 0x30) >> 4));
            v.push(((self.sc & 0x0F) << 4) | 0x08 | self.ssc);
            v.extend_from_slice(&self.len.to_be_bytes());
            v
        } else {
            let mut v: Vec<u8> = Vec::new();
            v.push((self.c << 2) | ((self.sc & 0x30) >> 4));
            v.push(((self.sc & 0x0F) << 4) | self.ssc);
            v.push(self.len as u8);
            v
        };

        let enc = encoder().unwrap();
        enc.encode(&vec)
    }

    pub fn name(&self) -> (String, String, String) {
        let empty1 = HashMap::new();
        let empty2 = HashMap::new();

        let (c, sc_names) = match CDE_NAMES.get(&self.c) {
            Some((c, h)) => (c, h),
            None => (&"_", &empty1)
        };

        let c_name = match &self.c_name {
            Some(c) => c.clone(),
            None => c.to_string()
        };

        let (sc, ssc_names) = match sc_names.get(&self.sc) {
            Some((sc, h)) => (sc, h),
            None => (&"_", &empty2)
        };

        let sc_name = match &self.sc_name {
            Some(sc) => sc.clone(),
            None => sc.to_string()
        };

        let ssc = match ssc_names.get(&self.ssc) {
            Some(ssc) => ssc,
            None => &"0"
        };

        let ssc_name = match &self.ssc_name {
            Some(ssc) => ssc.clone(),
            None => ssc.to_string()
        };

        (c_name, sc_name, ssc_name)
    }

    pub fn class(&self) -> u8 {
        self.c
    }

    pub fn subclass(&self) -> u8 {
        self.sc
    }

    pub fn subsubclass(&self) -> u8 {
        self.ssc
    }
}

enum TagBuildFrom {
    Str,
    Byt
}

pub struct TagBuilder {
    how: TagBuildFrom,
    s: String,
    b: Vec<u8>,
    l: u32
}

impl TagBuilder {
    pub fn from_str(s: &str) -> Self {
        TagBuilder {
            how: TagBuildFrom::Str,
            s: String::from(s),
            b: Vec::new(),
            l: 0u32
        }
    }

    pub fn from_bytes<T: AsRef<[u8]>>(b: T) -> Self {
        let mut v = Vec::new();
        v.extend_from_slice(b.as_ref());
        TagBuilder {
            how: TagBuildFrom::Byt,
            s: String::new(),
            b: v,
            l: 0u32
        }
    }

    pub fn length(mut self, l: u32) -> Self {
        self.l = l;
        self
    }

    pub fn build(self) -> Tag {
        match self.how {
            TagBuildFrom::Str => {
                let mut tt = Tag::from_str(&self.s).unwrap();
                tt.set_length(self.l);
                tt
            },
            TagBuildFrom::Byt => {
                Tag::from(&self.b)
            }
        }
    }
}

lazy_static! {

    static ref CDE_VALUES: HashMap<&'static str, (u8, HashMap<&'static str, (u8, HashMap<&'static str, u8>)>)> = {
        let mut m = HashMap::new();

        /* aead sub-classes */
        let aead = {
            let mut m = HashMap::new();

            m.insert("a", (idx('a'), HashMap::new()));
            m.insert("c", (idx('c'), HashMap::new()));
            m.insert("i", (idx('i'), HashMap::new()));
            m.insert("x", (idx('x'), HashMap::new()));
            m.insert("-", (idx('-'), HashMap::new()));

            m.insert("aes256-gcm", (idx('a'), HashMap::new()));
            m.insert("chacha20-poly1305", (idx('c'), HashMap::new()));
            m.insert("chacha20-poly1305-ietf", (idx('i'), HashMap::new()));
            m.insert("xchacha20-poly1305-ietf", (idx('x'), HashMap::new()));
            m.insert("list", (idx('-'), HashMap::new()));

            m
        };

        /* cipher sub-classes */
        let cipher = {
            let mut m = HashMap::new();

            /* aes sub-sub-classes */
            let aes = {
                let mut m = HashMap::new();
                m.insert("256", 0);
                m.insert("128", 1);
                m.insert("192", 2);
                m
            };

            m.insert("a", (idx('a'), aes.clone()));
            m.insert("x", (idx('x'), HashMap::new()));
            m.insert("-", (idx('-'), HashMap::new()));

            m.insert("aes", (idx('a'), aes));
            m.insert("xchacha20", (idx('x'), HashMap::new()));
            m.insert("list", (idx('-'), HashMap::new()));

            m
        };

        /* digest sub-classes */
        let digest = {
            let mut m = HashMap::new();

            /* blake2 sub-sub-classes */
            let blake2 = {
                let mut m = HashMap::new();
                m.insert("b", 0);
                m.insert("s", 1);
                m
            };

            /* md sub-sub-classes */
            let md = {
                let mut m = HashMap::new();
                m.insert("5", 0);
                m.insert("4", 1);
                m.insert("2", 2);
                m.insert("6", 3);
                m
            };

            /* sha2 sub-sub-classes */
            let sha2 = {
                let mut m = HashMap::new();
                m.insert("256", 0);
                m.insert("512", 1);
                m.insert("224", 2);
                m.insert("384", 3);
                m.insert("512/224", 4);
                m.insert("512/256", 5);
                m
            };

            /* sha3 sub-sub-classes */
            let sha3 = {
                let mut m = HashMap::new();
                m.insert("256", 0);
                m.insert("512", 1);
                m.insert("224", 2);
                m.insert("384", 3);
                m
            };

            /* shake sub-sub-classes */
            let shake = {
                let mut m = HashMap::new();
                m.insert("128", 0);
                m.insert("256", 1);
                m
            };

            m.insert("b", (idx('b'), blake2.clone()));
            m.insert("m", (idx('m'), md.clone()));
            m.insert("1", (idx('1'), HashMap::new()));
            m.insert("2", (idx('2'), sha2.clone()));
            m.insert("3", (idx('3'), sha3.clone()));
            m.insert("k", (idx('k'), shake.clone()));
            m.insert("-", (idx('-'), HashMap::new()));

            m.insert("blake2", (idx('b'), blake2));
            m.insert("md", (idx('m'), md));
            m.insert("sha1", (idx('1'), HashMap::new()));
            m.insert("sha2", (idx('2'), sha2));
            m.insert("sha3", (idx('3'), sha3));
            m.insert("shake", (idx('k'), shake));
            m.insert("list", (idx('-'), HashMap::new()));
            
            m
        };

        /* identifier sub-classes */
        let identifier = {
            let mut m = HashMap::new();

            m.insert("a", (idx('a'), HashMap::new()));
            m.insert("-", (idx('-'), HashMap::new()));

            m.insert("author", (idx('a'), HashMap::new()));
            m.insert("list", (idx('-'), HashMap::new()));

            m
        };

        /* key sub-classes */
        let key = {
            let mut m = HashMap::new();

            /* ed25519 sub-sub-classes */
            let ed25519 = {
                let mut m = HashMap::new();
                m.insert("public", 0);
                m.insert("secret", 1);
                m
            };

            /* rsa sub-sub-classes */
            let rsa = {
                let mut m = HashMap::new();
                m.insert("public", 0);
                m.insert("secret", 1);
                m
            };

            m.insert("e", (idx('e'), ed25519.clone()));
            m.insert("r", (idx('r'), rsa.clone()));
            m.insert("-", (idx('-'), HashMap::new()));

            m.insert("ed25519", (idx('e'), ed25519));
            m.insert("rsa", (idx('r'), rsa));
            m.insert("list", (idx('-'), HashMap::new()));

            m
        };

        /* policy sub-classes */
        let policy = {
            let mut m = HashMap::new();

            m.insert("b", (idx('b'), HashMap::new()));
            m.insert("r", (idx('r'), HashMap::new()));
            m.insert("s", (idx('s'), HashMap::new()));
            m.insert("-", (idx('-'), HashMap::new()));

            m.insert("bitcoin", (idx('b'), HashMap::new()));
            m.insert("r1cs", (idx('r'), HashMap::new()));
            m.insert("solidity", (idx('s'), HashMap::new()));
            m.insert("list", (idx('-'), HashMap::new()));

            m
        };

        /* signature sub-classes */
        let signature = {
            let mut m = HashMap::new();

            m.insert("m", (idx('m'), HashMap::new()));
            m.insert("o", (idx('o'), HashMap::new()));
            m.insert("p", (idx('p'), HashMap::new()));
            m.insert("x", (idx('x'), HashMap::new()));
            m.insert("-", (idx('-'), HashMap::new()));

            m.insert("minisign", (idx('m'), HashMap::new()));
            m.insert("openssl", (idx('o'), HashMap::new()));
            m.insert("pgp", (idx('p'), HashMap::new()));
            m.insert("x509", (idx('x'), HashMap::new()));
            m.insert("list", (idx('-'), HashMap::new()));

            m
        };

        /* list sub-classes */
        let list = {
            let mut m = HashMap::new();
            m.insert("-", (idx('-'), HashMap::new()));
            m.insert("list", (idx('-'), HashMap::new()));

            m
        };

        /* any sub-classes */
        let any = {
            let mut m = HashMap::new();
            m.insert("-", (idx('-'), HashMap::new()));
            m.insert("_", (idx('_'), HashMap::new()));

            m.insert("list", (idx('-'), HashMap::new()));
            m.insert("any", (idx('_'), HashMap::new()));

            m
        };

        /* standard classes */
        m.insert("a", (idx('a'), aead.clone()));
        m.insert("c", (idx('c'), cipher.clone()));
        m.insert("d", (idx('d'), digest.clone()));
        m.insert("f", (idx('f'), HashMap::new()));
        m.insert("h", (idx('h'), HashMap::new()));
        m.insert("i", (idx('i'), identifier.clone()));
        m.insert("k", (idx('k'), key.clone()));
        m.insert("n", (idx('n'), HashMap::new()));
        m.insert("p", (idx('p'), policy.clone()));
        m.insert("s", (idx('s'), signature.clone()));
        m.insert("-", (idx('-'), list.clone()));
        m.insert("_", (idx('_'), any.clone()));

        m.insert("aead", (idx('a'), aead));
        m.insert("cipher", (idx('c'), cipher));
        m.insert("digest", (idx('d'), digest));
        m.insert("proof", (idx('f'), HashMap::new()));
        m.insert("hmac", (idx('h'), HashMap::new()));
        m.insert("identifier", (idx('i'), identifier));
        m.insert("key", (idx('k'), key));
        m.insert("nonce", (idx('n'), HashMap::new()));
        m.insert("policy", (idx('p'), policy));
        m.insert("signature", (idx('s'), signature));
        m.insert("list", (idx('-'), list.clone()));
        m.insert("any", (idx('_'), any.clone()));

        /* experimental classes */
        m.insert("A", (idx('A'), HashMap::new()));
        m.insert("C", (idx('C'), HashMap::new()));
        m.insert("D", (idx('D'), HashMap::new()));
        m.insert("F", (idx('F'), HashMap::new()));
        m.insert("H", (idx('H'), HashMap::new()));
        m.insert("I", (idx('I'), HashMap::new()));
        m.insert("K", (idx('K'), HashMap::new()));
        m.insert("N", (idx('N'), HashMap::new()));
        m.insert("P", (idx('P'), HashMap::new()));
        m.insert("S", (idx('S'), HashMap::new()));

        m.insert("Aead", (idx('A'), HashMap::new()));
        m.insert("Cipher", (idx('C'), HashMap::new()));
        m.insert("Digest", (idx('D'), HashMap::new()));
        m.insert("Proof", (idx('F'), HashMap::new()));
        m.insert("Hmac", (idx('H'), HashMap::new()));
        m.insert("Identifier", (idx('I'), HashMap::new()));
        m.insert("Key", (idx('K'), HashMap::new()));
        m.insert("Nonce", (idx('N'), HashMap::new()));
        m.insert("Policy", (idx('P'), HashMap::new()));
        m.insert("Signature", (idx('S'), HashMap::new()));
        m.insert("List", (idx('-'), list));
        m.insert("Any", (idx('_'), any));

        m
    };

    static ref CDE_NAMES: HashMap<u8, (&'static str, HashMap<u8, (&'static str, HashMap<u8, &'static str>)>)> = {
        let mut m = HashMap::new();

        /* aead sub-classes */
        let aead = {
            let mut m = HashMap::new();
            m.insert(idx('a'), ("aes256-gcm", HashMap::new()));
            m.insert(idx('c'), ("chacha20-poly1305", HashMap::new()));
            m.insert(idx('i'), ("chacha20-poly1305-ietf", HashMap::new()));
            m.insert(idx('x'), ("xchacha20-poly1305-ietf", HashMap::new()));
            m.insert(idx('-'), ("list", HashMap::new()));

            m
        };

        /* cipher sub-classes */
        let cipher = {
            let mut m = HashMap::new();

            /* aes sub-sub-classes */
            let aes = {
                let mut m = HashMap::new();
                m.insert(0, "256");
                m.insert(1, "128");
                m.insert(2, "192");
                m
            };

            m.insert(idx('a'), ("aes", aes));
            m.insert(idx('x'), ("xchacha20", HashMap::new()));
            m.insert(idx('-'), ("list", HashMap::new()));

            m
        };

        /* digest sub-classes */
        let digest = {
            let mut m = HashMap::new();

            /* blake2 sub-sub-classes */
            let blake2 = {
                let mut m = HashMap::new();
                m.insert(0, "b");
                m.insert(1, "s");
                m
            };

            /* md sub-sub-classes */
            let md = {
                let mut m = HashMap::new();
                m.insert(0, "5");
                m.insert(1, "4");
                m.insert(2, "2");
                m.insert(3, "6");
                m
            };

            /* sha2 sub-sub-classes */
            let sha2 = {
                let mut m = HashMap::new();
                m.insert(0, "256");
                m.insert(1, "512");
                m.insert(2, "224");
                m.insert(3, "384");
                m.insert(4, "512/224");
                m.insert(5, "512/256");
                m
            };

            /* sha3 sub-sub-classes */
            let sha3 = {
                let mut m = HashMap::new();
                m.insert(0, "256");
                m.insert(1, "512");
                m.insert(2, "224");
                m.insert(3, "384");
                m
            };

            /* shake sub-sub-classes */
            let shake = {
                let mut m = HashMap::new();
                m.insert(0, "128");
                m.insert(1, "256");
                m
            };

            m.insert(idx('b'), ("blake2", blake2));
            m.insert(idx('m'), ("md", md));
            m.insert(idx('1'), ("sha1", HashMap::new()));
            m.insert(idx('2'), ("sha2", sha2));
            m.insert(idx('3'), ("sha3", sha3));
            m.insert(idx('k'), ("shake", shake));
            m.insert(idx('-'), ("list", HashMap::new()));

            m
        };


        /* identifier sub-classes */
        let identifier = {
            let mut m = HashMap::new();
            m.insert(idx('a'), ("author", HashMap::new()));
            m.insert(idx('-'), ("list", HashMap::new()));
            
            m
        };

        /* key sub-classes */
        let key = {
            let mut m = HashMap::new();

            /* key_ed25519 sub-classes */
            let ed25519 = {
                let mut m = HashMap::new();
                m.insert(0, "public");
                m.insert(1, "secret");
                m
            };

            /* key_rsa sub-classes */
            let rsa = {
                let mut m = HashMap::new();
                m.insert(0, "public");
                m.insert(1, "secret");
                m
            };

            m.insert(idx('e'), ("ed25519", ed25519));
            m.insert(idx('r'), ("rsa", rsa));
            m.insert(idx('-'), ("list", HashMap::new()));

            m
        };


        /* policy sub-classes */
        let policy = {
            let mut m = HashMap::new();
            m.insert(idx('b'), ("bitcoin", HashMap::new()));
            m.insert(idx('r'), ("r1cs", HashMap::new()));
            m.insert(idx('s'), ("solidity", HashMap::new()));
            m.insert(idx('-'), ("list", HashMap::new()));

            m
        };

        /* signature sub-classes */
        let signature = {
            let mut m = HashMap::new();
            m.insert(idx('m'), ("minisign", HashMap::new()));
            m.insert(idx('o'), ("openssl", HashMap::new()));
            m.insert(idx('p'), ("pgp", HashMap::new()));
            m.insert(idx('x'), ("x509", HashMap::new()));
            m.insert(idx('-'), ("list", HashMap::new()));

            m
        };

        /* list sub-classes */
        let list = {
            let mut m = HashMap::new();
            m.insert(idx('-'), ("list", HashMap::new()));
            m
        };

        /* any sub-classes */
        let any = {
            let mut m = HashMap::new();
            m.insert(idx('-'), ("list", HashMap::new()));
            m.insert(idx('_'), ("any", HashMap::new()));
            m
        };


        /* standard classes */
        m.insert(idx('a'), ("aead", aead));
        m.insert(idx('c'), ("cipher", cipher));
        m.insert(idx('d'), ("digest", digest));
        m.insert(idx('f'), ("proof", HashMap::new()));
        m.insert(idx('h'), ("hmac", HashMap::new()));
        m.insert(idx('i'), ("identifier", identifier));
        m.insert(idx('k'), ("key", key));
        m.insert(idx('n'), ("nonce", HashMap::new()));
        m.insert(idx('p'), ("policy", policy));
        m.insert(idx('s'), ("signature", signature));
        m.insert(idx('-'), ("list", list));
        m.insert(idx('_'), ("any", any));

        /* experimental classes */
        m.insert(idx('A'), ("Aead", HashMap::new()));
        m.insert(idx('C'), ("Cipher", HashMap::new()));
        m.insert(idx('D'), ("Digest", HashMap::new()));
        m.insert(idx('F'), ("Proof", HashMap::new()));
        m.insert(idx('H'), ("Hmac", HashMap::new()));
        m.insert(idx('I'), ("Identifier", HashMap::new()));
        m.insert(idx('K'), ("Key", HashMap::new()));
        m.insert(idx('N'), ("Nonce", HashMap::new()));
        m.insert(idx('P'), ("Policy", HashMap::new()));
        m.insert(idx('S'), ("Signature", HashMap::new()));
        m
    };
}

/*
lazy_static! {

    static ref CDE_VALUES: HashMap<&'static str, (u8, HashMap<&'static str, (u8, HashMap<&'static str, u8>)>)> = {
        let mut m = HashMap::new();

        /* aead sub-classes */
        let aead = {
            let mut m = HashMap::new();

            m.insert("a", (idx('a'), HashMap::new()));
            m.insert("c", (idx('c'), HashMap::new()));
            m.insert("i", (idx('i'), HashMap::new()));
            m.insert("x", (idx('x'), HashMap::new()));
            m.insert("-", (idx('-'), HashMap::new()));

            m.insert("aes256-gcm", (idx('a'), HashMap::new()));
            m.insert("chacha20-poly1305", (idx('c'), HashMap::new()));
            m.insert("chacha20-poly1305-ietf", (idx('i'), HashMap::new()));
            m.insert("xchacha20-poly1305-ietf", (idx('x'), HashMap::new()));
            m.insert("list", (idx('-'), HashMap::new()));

            m.insert("A", (idx('A'), HashMap::new()));
            m.insert("C", (idx('C'), HashMap::new()));
            m.insert("I", (idx('I'), HashMap::new()));
            m.insert("X", (idx('X'), HashMap::new()));

            m.insert("Aes256-gcm", (idx('A'), HashMap::new()));
            m.insert("Chacha20-poly1305", (idx('C'), HashMap::new()));
            m.insert("Chacha20-poly1305-ietf", (idx('I'), HashMap::new()));
            m.insert("Xchacha20-poly1305-ietf", (idx('X'), HashMap::new()));

            m
        };

        /* cipher sub-classes */
        let cipher = {
            let mut m = HashMap::new();

            /* aes sub-sub-classes */
            let aes = {
                let mut m = HashMap::new();
                m.insert("256", 0);
                m.insert("128", 1);
                m.insert("192", 2);
                m
            };

            m.insert("a", (idx('a'), aes.clone()));
            m.insert("x", (idx('x'), HashMap::new()));
            m.insert("-", (idx('-'), HashMap::new()));

            m.insert("aes", (idx('a'), aes));
            m.insert("xchacha20", (idx('x'), HashMap::new()));
            m.insert("list", (idx('-'), HashMap::new()));

            m.insert("A", (idx('A'), HashMap::new()));
            m.insert("X", (idx('X'), HashMap::new()));

            m.insert("Aes", (idx('A'), HashMap::new()));
            m.insert("Xchacha20", (idx('X'), HashMap::new()));
            m
        };

        /* digest sub-classes */
        let digest = {
            let mut m = HashMap::new();

            /* blake2 sub-sub-classes */
            let blake2 = {
                let mut m = HashMap::new();
                m.insert("b", 0);
                m.insert("s", 1);
                m
            };

            /* md sub-sub-classes */
            let md = {
                let mut m = HashMap::new();
                m.insert("5", 0);
                m.insert("4", 1);
                m.insert("2", 2);
                m.insert("6", 3);
                m
            };

            /* sha2 sub-sub-classes */
            let sha2 = {
                let mut m = HashMap::new();
                m.insert("256", 0);
                m.insert("512", 1);
                m.insert("224", 2);
                m.insert("384", 3);
                m.insert("512/224", 4);
                m.insert("512/256", 5);
                m
            };

            /* sha3 sub-sub-classes */
            let sha3 = {
                let mut m = HashMap::new();
                m.insert("256", 0);
                m.insert("512", 1);
                m.insert("224", 2);
                m.insert("384", 3);
                m
            };

            /* shake sub-sub-classes */
            let shake = {
                let mut m = HashMap::new();
                m.insert("128", 0);
                m.insert("256", 1);
                m
            };

            m.insert("b", (idx('b'), blake2.clone()));
            m.insert("m", (idx('m'), md.clone()));
            m.insert("1", (idx('1'), HashMap::new()));
            m.insert("2", (idx('2'), sha2.clone()));
            m.insert("3", (idx('3'), sha3.clone()));
            m.insert("k", (idx('k'), shake.clone()));
            m.insert("-", (idx('-'), HashMap::new()));

            m.insert("blake2", (idx('b'), blake2));
            m.insert("md", (idx('m'), md));
            m.insert("sha1", (idx('1'), HashMap::new()));
            m.insert("sha2", (idx('2'), sha2));
            m.insert("sha3", (idx('3'), sha3));
            m.insert("shake", (idx('k'), shake));
            m.insert("list", (idx('-'), HashMap::new()));

            m.insert("B", (idx('B'), HashMap::new()));
            m.insert("M", (idx('M'), HashMap::new()));
            m.insert("6", (idx('6'), HashMap::new()));
            m.insert("7", (idx('7'), HashMap::new()));
            m.insert("8", (idx('8'), HashMap::new()));
            m.insert("K", (idx('K'), HashMap::new()));

            m.insert("Blake2", (idx('b'), HashMap::new()));
            m.insert("Md", (idx('m'), HashMap::new()));
            m.insert("Sha1", (idx('6'), HashMap::new()));
            m.insert("Sha2", (idx('7'), HashMap::new()));
            m.insert("Sha3", (idx('8'), HashMap::new()));
            m.insert("Shake", (idx('K'), HashMap::new()));
            m
        };

        /* identifier sub-classes */
        let identifier = {
            let mut m = HashMap::new();

            m.insert("a", (idx('a'), HashMap::new()));
            m.insert("-", (idx('-'), HashMap::new()));

            m.insert("author", (idx('a'), HashMap::new()));
            m.insert("list", (idx('-'), HashMap::new()));

            m.insert("A", (idx('A'), HashMap::new()));

            m.insert("Author", (idx('A'), HashMap::new()));
            m
        };

        /* key sub-classes */
        let key = {
            let mut m = HashMap::new();

            /* ed25519 sub-sub-classes */
            let ed25519 = {
                let mut m = HashMap::new();
                m.insert("public", 0);
                m.insert("secret", 1);
                m
            };

            /* rsa sub-sub-classes */
            let rsa = {
                let mut m = HashMap::new();
                m.insert("public", 0);
                m.insert("secret", 1);
                m
            };

            m.insert("e", (idx('e'), ed25519.clone()));
            m.insert("r", (idx('r'), rsa.clone()));
            m.insert("-", (idx('-'), HashMap::new()));

            m.insert("ed25519", (idx('e'), ed25519));
            m.insert("rsa", (idx('r'), rsa));
            m.insert("list", (idx('-'), HashMap::new()));

            m.insert("E", (idx('E'), HashMap::new()));
            m.insert("R", (idx('R'), HashMap::new()));

            m.insert("Ed25519", (idx('E'), HashMap::new()));
            m.insert("Rsa", (idx('R'), HashMap::new()));
            m
        };

        /* policy sub-classes */
        let policy = {
            let mut m = HashMap::new();

            m.insert("b", (idx('b'), HashMap::new()));
            m.insert("r", (idx('r'), HashMap::new()));
            m.insert("s", (idx('s'), HashMap::new()));
            m.insert("-", (idx('-'), HashMap::new()));

            m.insert("bitcoin", (idx('b'), HashMap::new()));
            m.insert("r1cs", (idx('r'), HashMap::new()));
            m.insert("solidity", (idx('s'), HashMap::new()));
            m.insert("list", (idx('-'), HashMap::new()));

            m.insert("B", (idx('B'), HashMap::new()));
            m.insert("R", (idx('R'), HashMap::new()));
            m.insert("S", (idx('S'), HashMap::new()));

            m.insert("Bitcoin", (idx('B'), HashMap::new()));
            m.insert("R1cs", (idx('R'), HashMap::new()));
            m.insert("Solidity", (idx('S'), HashMap::new()));
            m
        };

        /* signature sub-classes */
        let signature = {
            let mut m = HashMap::new();

            m.insert("m", (idx('m'), HashMap::new()));
            m.insert("o", (idx('o'), HashMap::new()));
            m.insert("p", (idx('p'), HashMap::new()));
            m.insert("x", (idx('x'), HashMap::new()));
            m.insert("-", (idx('-'), HashMap::new()));

            m.insert("minisign", (idx('m'), HashMap::new()));
            m.insert("openssl", (idx('o'), HashMap::new()));
            m.insert("pgp", (idx('p'), HashMap::new()));
            m.insert("x509", (idx('x'), HashMap::new()));
            m.insert("list", (idx('-'), HashMap::new()));

            m.insert("M", (idx('M'), HashMap::new()));
            m.insert("O", (idx('O'), HashMap::new()));
            m.insert("P", (idx('P'), HashMap::new()));
            m.insert("X", (idx('X'), HashMap::new()));

            m.insert("Minisign", (idx('M'), HashMap::new()));
            m.insert("Openssl", (idx('O'), HashMap::new()));
            m.insert("Pgp", (idx('P'), HashMap::new()));
            m.insert("X509", (idx('X'), HashMap::new()));
            m
        };

        /* list sub-classes */
        let list = {
            let mut m = HashMap::new();
            m.insert("-", (idx('-'), HashMap::new()));
            m.insert("list", (idx('-'), HashMap::new()));

            m.insert("List", (idx('-'), HashMap::new()));
            m
        };

        /* any sub-classes */
        let any = {
            let mut m = HashMap::new();
            m.insert("-", (idx('-'), HashMap::new()));
            m.insert("_", (idx('_'), HashMap::new()));

            m.insert("list", (idx('-'), HashMap::new()));
            m.insert("any", (idx('_'), HashMap::new()));

            m.insert("List", (idx('-'), HashMap::new()));
            m.insert("Any", (idx('_'), HashMap::new()));
            m
        };

        /* standard classes */
        m.insert("a", (idx('a'), aead.clone()));
        m.insert("c", (idx('c'), cipher.clone()));
        m.insert("d", (idx('d'), digest.clone()));
        m.insert("f", (idx('f'), HashMap::new()));
        m.insert("h", (idx('h'), HashMap::new()));
        m.insert("i", (idx('i'), identifier.clone()));
        m.insert("k", (idx('k'), key.clone()));
        m.insert("n", (idx('n'), HashMap::new()));
        m.insert("p", (idx('p'), policy.clone()));
        m.insert("s", (idx('s'), signature.clone()));
        m.insert("-", (idx('-'), list.clone()));

        m.insert("aead", (idx('a'), aead));
        m.insert("cipher", (idx('c'), cipher));
        m.insert("digest", (idx('d'), digest));
        m.insert("proof", (idx('f'), HashMap::new()));
        m.insert("hmac", (idx('h'), HashMap::new()));
        m.insert("identifier", (idx('i'), identifier));
        m.insert("key", (idx('k'), key));
        m.insert("nonce", (idx('n'), HashMap::new()));
        m.insert("policy", (idx('p'), policy));
        m.insert("signature", (idx('s'), signature));
        m.insert("list", (idx('-'), list.clone()));
        m.insert("any", (idx('_'), any.clone()));

        /* experimental classes */
        m.insert("A", (idx('A'), HashMap::new()));
        m.insert("C", (idx('C'), HashMap::new()));
        m.insert("D", (idx('D'), HashMap::new()));
        m.insert("F", (idx('F'), HashMap::new()));
        m.insert("H", (idx('H'), HashMap::new()));
        m.insert("I", (idx('I'), HashMap::new()));
        m.insert("K", (idx('K'), HashMap::new()));
        m.insert("N", (idx('N'), HashMap::new()));
        m.insert("P", (idx('P'), HashMap::new()));
        m.insert("S", (idx('S'), HashMap::new()));
        m.insert("_", (idx('_'), any.clone()));

        m.insert("Aead", (idx('A'), HashMap::new()));
        m.insert("Cipher", (idx('C'), HashMap::new()));
        m.insert("Digest", (idx('D'), HashMap::new()));
        m.insert("Proof", (idx('F'), HashMap::new()));
        m.insert("Hmac", (idx('H'), HashMap::new()));
        m.insert("Identifier", (idx('I'), HashMap::new()));
        m.insert("Key", (idx('K'), HashMap::new()));
        m.insert("Nonce", (idx('N'), HashMap::new()));
        m.insert("Policy", (idx('P'), HashMap::new()));
        m.insert("Signature", (idx('S'), HashMap::new()));
        m.insert("List", (idx('-'), list));
        m.insert("Any", (idx('_'), any));

        m
    };

    static ref CDE_NAMES: HashMap<u8, (&'static str, HashMap<u8, (&'static str, HashMap<u8, &'static str>)>)> = {
        let mut m = HashMap::new();

        /* aead sub-classes */
        let aead = {
            let mut m = HashMap::new();
            m.insert(idx('a'), ("aes256-gcm", HashMap::new()));
            m.insert(idx('c'), ("chacha20-poly1305", HashMap::new()));
            m.insert(idx('i'), ("chacha20-poly1305-ietf", HashMap::new()));
            m.insert(idx('x'), ("xchacha20-poly1305-ietf", HashMap::new()));
            m.insert(idx('-'), ("list", HashMap::new()));

            m.insert(idx('A'), ("Aes256-gcm", HashMap::new()));
            m.insert(idx('C'), ("Chacha20-poly1305", HashMap::new()));
            m.insert(idx('I'), ("Chacha20-poly1305-ietf", HashMap::new()));
            m.insert(idx('X'), ("Xchacha20-poly1305-ietf", HashMap::new()));
            m
        };

        /* cipher sub-classes */
        let cipher = {
            let mut m = HashMap::new();

            /* aes sub-sub-classes */
            let aes = {
                let mut m = HashMap::new();
                m.insert(0, "256");
                m.insert(1, "128");
                m.insert(2, "192");
                m
            };

            m.insert(idx('a'), ("aes", aes));
            m.insert(idx('x'), ("xchacha20", HashMap::new()));
            m.insert(idx('-'), ("list", HashMap::new()));

            m.insert(idx('A'), ("Aes", HashMap::new()));
            m.insert(idx('X'), ("Xchacha20", HashMap::new()));
            m
        };

        /* digest sub-classes */
        let digest = {
            let mut m = HashMap::new();

            /* blake2 sub-sub-classes */
            let blake2 = {
                let mut m = HashMap::new();
                m.insert(0, "b");
                m.insert(1, "s");
                m
            };

            /* md sub-sub-classes */
            let md = {
                let mut m = HashMap::new();
                m.insert(0, "5");
                m.insert(1, "4");
                m.insert(2, "2");
                m.insert(3, "6");
                m
            };

            /* sha2 sub-sub-classes */
            let sha2 = {
                let mut m = HashMap::new();
                m.insert(0, "256");
                m.insert(1, "512");
                m.insert(2, "224");
                m.insert(3, "384");
                m.insert(4, "512/224");
                m.insert(5, "512/256");
                m
            };

            /* sha3 sub-sub-classes */
            let sha3 = {
                let mut m = HashMap::new();
                m.insert(0, "256");
                m.insert(1, "512");
                m.insert(2, "224");
                m.insert(3, "384");
                m
            };

            /* shake sub-sub-classes */
            let shake = {
                let mut m = HashMap::new();
                m.insert(0, "128");
                m.insert(1, "256");
                m
            };

            m.insert(idx('b'), ("blake2", blake2));
            m.insert(idx('m'), ("md", md));
            m.insert(idx('1'), ("sha1", HashMap::new()));
            m.insert(idx('2'), ("sha2", sha2));
            m.insert(idx('3'), ("sha3", sha3));
            m.insert(idx('k'), ("shake", shake));
            m.insert(idx('-'), ("list", HashMap::new()));

            m.insert(idx('B'), ("Blake2", HashMap::new()));
            m.insert(idx('M'), ("Md", HashMap::new()));
            m.insert(idx('6'), ("Sha1", HashMap::new()));
            m.insert(idx('7'), ("Sha2", HashMap::new()));
            m.insert(idx('8'), ("Sha3", HashMap::new()));
            m.insert(idx('K'), ("Shake", HashMap::new()));
            m
        };


        /* identifier sub-classes */
        let identifier = {
            let mut m = HashMap::new();
            m.insert(idx('a'), ("author", HashMap::new()));
            m.insert(idx('-'), ("list", HashMap::new()));
            
            m.insert(idx('A'), ("Author", HashMap::new()));
            m
        };

        /* key sub-classes */
        let key = {
            let mut m = HashMap::new();

            /* key_ed25519 sub-classes */
            let ed25519 = {
                let mut m = HashMap::new();
                m.insert(0, "public");
                m.insert(1, "secret");
                m
            };

            /* key_rsa sub-classes */
            let rsa = {
                let mut m = HashMap::new();
                m.insert(0, "public");
                m.insert(1, "secret");
                m
            };

            m.insert(idx('e'), ("ed25519", ed25519));
            m.insert(idx('r'), ("rsa", rsa));
            m.insert(idx('-'), ("list", HashMap::new()));

            m.insert(idx('E'), ("Ed25519", HashMap::new()));
            m.insert(idx('R'), ("Rsa", HashMap::new()));
            m
        };


        /* policy sub-classes */
        let policy = {
            let mut m = HashMap::new();
            m.insert(idx('b'), ("bitcoin", HashMap::new()));
            m.insert(idx('r'), ("r1cs", HashMap::new()));
            m.insert(idx('s'), ("solidity", HashMap::new()));
            m.insert(idx('-'), ("list", HashMap::new()));

            m.insert(idx('B'), ("Bitcoin", HashMap::new()));
            m.insert(idx('R'), ("R1cs", HashMap::new()));
            m.insert(idx('S'), ("Solidity", HashMap::new()));
            m
        };

        /* signature sub-classes */
        let signature = {
            let mut m = HashMap::new();
            m.insert(idx('m'), ("minisign", HashMap::new()));
            m.insert(idx('o'), ("openssl", HashMap::new()));
            m.insert(idx('p'), ("pgp", HashMap::new()));
            m.insert(idx('x'), ("x509", HashMap::new()));
            m.insert(idx('-'), ("list", HashMap::new()));

            m.insert(idx('M'), ("Minisign", HashMap::new()));
            m.insert(idx('O'), ("Openssl", HashMap::new()));
            m.insert(idx('P'), ("Pgp", HashMap::new()));
            m.insert(idx('X'), ("X509", HashMap::new()));
            m
        };

        /* list sub-classes */
        let list = {
            let mut m = HashMap::new();
            m.insert(idx('-'), ("list", HashMap::new()));
            m
        };

        /* any sub-classes */
        let any = {
            let mut m = HashMap::new();
            m.insert(idx('-'), ("list", HashMap::new()));
            m.insert(idx('_'), ("any", HashMap::new()));
            m
        };


        /* standard classes */
        m.insert(idx('a'), ("aead", aead));
        m.insert(idx('c'), ("cipher", cipher));
        m.insert(idx('d'), ("digest", digest));
        m.insert(idx('f'), ("proof", HashMap::new()));
        m.insert(idx('h'), ("hmac", HashMap::new()));
        m.insert(idx('i'), ("identifier", identifier));
        m.insert(idx('k'), ("key", key));
        m.insert(idx('n'), ("nonce", HashMap::new()));
        m.insert(idx('p'), ("policy", policy));
        m.insert(idx('s'), ("signature", signature));
        m.insert(idx('-'), ("list", list));

        /* experimental classes */
        m.insert(idx('A'), ("Aead", HashMap::new()));
        m.insert(idx('C'), ("Cipher", HashMap::new()));
        m.insert(idx('D'), ("Digest", HashMap::new()));
        m.insert(idx('F'), ("Proof", HashMap::new()));
        m.insert(idx('H'), ("Hmac", HashMap::new()));
        m.insert(idx('I'), ("Identifier", HashMap::new()));
        m.insert(idx('K'), ("Key", HashMap::new()));
        m.insert(idx('N'), ("Nonce", HashMap::new()));
        m.insert(idx('P'), ("Policy", HashMap::new()));
        m.insert(idx('S'), ("Signature", HashMap::new()));
        m.insert(idx('_'), ("Any", any));
        m
    };
}*/
