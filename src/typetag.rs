use crate::{
    idx,
    encoder,
    CdeError,
    Class,
    SubClass,
    SubSubClass
};
use log::info;
use std::fmt::{ self, Display, Formatter };

#[derive(Debug, Default)]
pub struct TypeTag {
    c: Class,
    sc: SubClass,
    ssc: SubSubClass,
    len: u32
}

impl<T: AsRef<[u8]>> From<T> for TypeTag {
    fn from(v: T) -> Self {
        let c = Class::from(&v);
        let sc = SubClass::from(&v);
        let ssc = SubSubClass::from(&v);
        let len = if v.as_ref().len() > 3 {
            info!("TypeTag 8-bit words: {:#02x} {:#02x} {:#02x} {:#02x} {:#02x} {:#02x}",
                  v.as_ref()[0], v.as_ref()[1], v.as_ref()[2], v.as_ref()[3],
                  v.as_ref()[4], v.as_ref()[5]);
            let b: [u8; 4] = [v.as_ref()[2], v.as_ref()[3], v.as_ref()[4], v.as_ref()[5]];
            u32::from_be_bytes(b)
        } else {
            info!("TypeTag 8-bit words: {:#02x} {:#02x} {:#02x}", v.as_ref()[0],
                  v.as_ref()[1], v.as_ref()[2]);
            v.as_ref()[2] as u32
        };
        TypeTag { c: c, sc: sc, ssc: ssc, len: len }
    }
}

impl Display for TypeTag {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let s = self.encode();
        let (cn, scn, sscn) = self.type_name().unwrap();
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

impl TypeTag {
    pub fn new(c: &Class, sc: &SubClass, ssc: &SubSubClass, len: u32) -> Self {
        TypeTag { c: *c, sc: *sc, ssc: *ssc, len: len }
    }

    pub fn is_extended(&self) -> bool {
        self.len > 255
    }

    pub fn encode(&self) -> String {
        let c = self.c.value();
        let sc = self.sc.value();
        let ssc = self.ssc.value();

        let vec = if self.is_extended() {
            let mut v: Vec<u8> = Vec::new();
            v.push((c << 2) | ((sc & 0x30) >> 4));
            v.push(((sc & 0x0F) << 4) | 0x08 | ssc);
            v.extend_from_slice(&self.len.to_be_bytes());
            v
        } else {
            let mut v: Vec<u8> = Vec::new();
            v.push((c << 2) | ((sc & 0x30) >> 4));
            v.push(((sc & 0x0F) << 4) | ssc);
            v.push(self.len as u8);
            v
        };

        let enc = encoder().unwrap();
        enc.encode(&vec)
    }

    pub fn type_name(&self) -> Result<(String, String, String), CdeError> {
        let s = self.encode().to_lowercase();
        let mut chars = s.chars();
        let c = chars.next().ok_or(CdeError::InvalidClass(s.clone()))?;
        let sc = chars.next().ok_or(CdeError::InvalidSubClass(s.clone()))?;
        let ssc = self.ssc.value();
        match c {
            'a' => {
                let cn = String::from("AEAD");
                let sscn = ssc.to_string();
                match sc {
                    'a' => Ok((cn, String::from("AES256-GCM"), sscn)),
                    'c' => Ok((cn, String::from("ChaCha20-Poly1305"), sscn)),
                    'i' => Ok((cn, String::from("ChaCha20-Poly1305-IETF"), sscn)),
                    'x' => Ok((cn, String::from("XChaCha20-Poly1305-IETF"), sscn)),
                    _ => Err(CdeError::InvalidSubClass(String::from(sc)))
                }
            },
            'c' => {
                let cn = String::from("Cipher");
                let sscn = ssc.to_string();
                match sc {
                    'a' => Ok((cn, String::from("AES"), sscn)),
                    'x' => Ok((cn, String::from("XChaCha20"), sscn)),
                    _ => Err(CdeError::InvalidSubClass(String::from(sc)))
                }
            },
            'd' => {
                let cn = String::from("Digest");
                match sc {
                    'b' => {
                        let scn = String::from("Blake2");
                        match ssc {
                            0 => Ok((cn, scn, String::from("Blake2b"))),
                            1 => Ok((cn, scn, String::from("Blake2s"))),
                            _ => Err(CdeError::InvalidSubSubClass(ssc.to_string()))
                        }
                    },
                    'm' => {
                        let scn = String::from("MD");
                        match ssc {
                            0 => Ok((cn, scn, String::from("MD5"))),
                            1 => Ok((cn, scn, String::from("MD4"))),
                            2 => Ok((cn, scn, String::from("MD2"))),
                            3 => Ok((cn, scn, String::from("MD6"))),
                            _ => Err(CdeError::InvalidSubSubClass(ssc.to_string()))
                        }
                    },
                    '1' => Ok((cn, String::from("SHA1"), String::default())),
                    '2' => {
                        let scn = String::from("SHA2");
                        match ssc {
                            0 => Ok((cn, scn, String::from("SHA2-256"))),
                            1 => Ok((cn, scn, String::from("SHA2-512"))),
                            2 => Ok((cn, scn, String::from("SHA2-224"))),
                            3 => Ok((cn, scn, String::from("SHA2-384"))),
                            4 => Ok((cn, scn, String::from("SHA2-512/224"))),
                            5 => Ok((cn, scn, String::from("SHA2-512/256"))),
                            _ => Err(CdeError::InvalidSubSubClass(ssc.to_string()))
                        }
                    },
                    '3' => {
                        let scn = String::from("SHA3");
                        match ssc {
                            0 => Ok((cn, scn, String::from("SHA3-256"))),
                            1 => Ok((cn, scn, String::from("SHA3-512"))),
                            2 => Ok((cn, scn, String::from("SHA3-224"))),
                            3 => Ok((cn, scn, String::from("SHA3-384"))),
                            4 => Ok((cn, scn, String::from("SHAKE128"))),
                            5 => Ok((cn, scn, String::from("SHAKE256"))),
                            _ => Err(CdeError::InvalidSubSubClass(ssc.to_string()))
                        }
                    },
                    _ => Err(CdeError::InvalidSubClass(String::from(sc)))
                }
            },
            'f' => {
                let cn = String::from("Proof");
                Ok((cn, sc.to_string(), ssc.to_string()))
            },
            'h' => {
                let cn = String::from("HMAC");
                Ok((cn, sc.to_string(), ssc.to_string()))
            },
            'i' => {
                let cn = String::from("Identifier");
                let sscn = ssc.to_string();
                match sc {
                    'a' => Ok((cn, String::from("ADI"), sscn)),
                    'd' => Ok((cn, String::from("DID"), sscn)),
                    'e' => Ok((cn, String::from("Email"), sscn)),
                    _ => Err(CdeError::InvalidSubClass(String::from(sc)))
                }
            },
            'k' => {
                let cn = String::from("Key");
                match sc {
                    'e' => {
                        let scn = String::from("Ed25519");
                        match ssc {
                            0 => Ok((cn, scn, String::from("Public"))),
                            1 => Ok((cn, scn, String::from("Secret"))),
                            _ => Err(CdeError::InvalidSubSubClass(ssc.to_string()))
                        }
                    },
                    'r' => {
                        let scn = String::from("RSA");
                        match ssc {
                            0 => Ok((cn, scn, String::from("Public"))),
                            1 => Ok((cn, scn, String::from("Secret"))),
                            _ => Err(CdeError::InvalidSubSubClass(ssc.to_string()))
                        }
                    },
                    _ => Err(CdeError::InvalidSubClass(String::from(sc)))
                }
            },
            'n' => {
                let cn = String::from("Nonce");
                Ok((cn, sc.to_string(), ssc.to_string()))
            },
            'p' => {
                let cn = String::from("Policy");
                let sscn = ssc.to_string();
                match sc {
                    'b' => Ok((cn, String::from("Bitcoin"), sscn)),
                    'c' => Ok((cn, String::from("Policy-as-Code"), sscn)),
                    'r' => Ok((cn, String::from("R1CS Gadget"), sscn)),
                    's' => Ok((cn, String::from("Solidity"), sscn)),
                    _ => Err(CdeError::InvalidSubClass(String::from(sc)))
                }
            },
            's' => {
                let cn = String::from("Signature");
                let sscn = ssc.to_string();
                match sc {
                    'm' => Ok((cn, String::from("Minisign"), sscn)),
                    'o' => Ok((cn, String::from("OpenSSL"), sscn)),
                    'p' => Ok((cn, String::from("PGP"), sscn)),
                    'x' => Ok((cn, String::from("X.509"), sscn)),
                    _ => Err(CdeError::InvalidSubClass(String::from(sc)))
                }
            },
            '-' => {
                let cn = String::from("List");
                let sscn = ssc.to_string();
                match sc {
                    '-' => Ok((cn, String::from("List"), sscn)),
                    _ => Err(CdeError::InvalidSubClass(String::from(sc)))
                }
            },
            '_' => {
                let cn = String::from("Non-Typed");
                let sscn = ssc.to_string();
                match sc {
                    '-' => Ok((cn, String::from("List"), sscn)),
                    '_' => Ok((cn, String::from("Non-Typed"), sscn)),
                    _ => Err(CdeError::InvalidSubClass(String::from(sc)))
                }
            },
            _ => Err(CdeError::InvalidClass(String::from(c)))
        }
    }
}


