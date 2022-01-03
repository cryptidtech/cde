pub enum CryptoClass<'a> {
    Aead {
        experimental: bool,
        subclass: AeadSubClass<'a>,
    },
    Claim {
        experimental: bool,
        subclass: ClaimSubClass<'a>,
    },
    Digest {
        experimental: bool,
        subclass: DigestSubClass<'a>,
    },
    Encryption {
        experimental: bool,
        subclass: EncryptionSubClass<'a>,
    },
    Hmac {
        experimental: bool,
        subclass: HmacSubClass<'a>,
    },
    Identifier {
        experimental: bool,
        subclass: IdentifierSubClass<'a>,
    },
    Key {
        experimental: bool,
        subclass: KeySubClass<'a>,
    },
    Nonce {
        experimental: bool,
        subclass: NonceSubClass<'a>,
    },
    Policy {
        experimental: bool,
        subclass: PolicySubClass<'a>,
    },
    Signature {
        experimental: bool,
        subclass: SignatureSubClass<'a>,
    },
    Timestamp {
        experimental: bool,
        subclass: TimestampSubClass<'a>,
    },
    List {
        subclass: ListSubClass,
    },
    Undefined {
        subclass: UndefinedSubClass,
    },
    Defined {
        name: &'a str,
        experimental: bool,
        subclass: DefinedSubClass<'a>,
    },
}

    pub enum AeadSubClass<'a> {
        Aes256Gcm {
            experimental: bool,
            subsubclass: SubSubClass,
        },
        ChaCha20Poly1305 {
            experimental: bool,
            subsubclass: ChaCha20Poly1305SubSubClass,
        },
        List {
            subsubclass: SubSubClass,
        },
        Undefined {
            subsubclass: SubSubClass,
        },
        Defined {
            name: &'a str,
            experimental: bool,
            subsubclass: SubSubClass,
        },
    }

        pub enum ChaCha20Poly1305SubSubClass {
            Plain   = 0,    // chacha20-poly1305
            Ietf    = 1,    // chacha20-poly1305-ietf
            Xietf   = 2,    // xchacha20-poly1305-ietf
        }

    pub enum ClaimSubClass<'a> {
        Oberon {
            experimental: bool,
            subsubclass: SubSubClass,
        },
        List {
            subsubclass: SubSubClass,
        },
        Undefined {
            subsubclass: SubSubClass,
        },
        Defined {
            name: &'a str,
            experimental: bool,
            subsubclass: SubSubClass,
        },
    }

    pub enum DigestSubClass<'a> {
        Blake2 {
            experimental: bool,
            subsubclass: Blake2SubSubClass,
        },
        Md {
            experimental: bool,
            subsubclass: MdSubSubClass,
        },
        Sha1 {
            experimental: bool,
            subsubclass: SubSubClass,
        },
        Sha2 {
            experimental: bool,
            subsubclass: Sha2SubSubClass,
        },
        Sha3 {
            experimental: bool,
            subsubclass: Sha3SubSubClass,
        },
        List {
            subsubclass: SubSubClass,
        },
        Undefined {
            subsubclass: SubSubClass,
        },
        Defined {
            name: &'a str,
            experimental: bool,
            subsubclass: SubSubClass,
        },
    }

        pub enum  Blake2SubSubClass {
            B       = 0,    // Blake2b
            S       = 1,    // Blake2s
        }

        pub enum MdSubSubClass {
            Two     = 2,    // MD2
            Four    = 4,    // MD4
            Five    = 5,    // MD5
            Six     = 6,    // MD6
        }

        pub enum Sha2SubSubClass {
            TwoTwoFour              = 0,    // SHA2-224
            TwoFiveSix              = 1,    // SHA2-256
            ThreeEightFour          = 2,    // SHA2-384
            FiveOneTwo              = 3,    // SHA2-512
            FiveOneTwoTwoTwoFour    = 4,    // SHA2-512/224
            FiveOneTwoTwoFiveSix    = 5,    // SHA2-512/256
        }

        pub enum Sha3SubSubClass {
            TwoTwoFour          = 0,    // SHA3-224
            TwoFiveSix          = 1,    // SHA3-256
            ThreeEightFour      = 2,    // SHA3-384
            FiveOneTwo          = 3,    // SHA3-512
            ShakeOneTwoEight    = 4,    // Shake-128
            ShakeTwoFiveSix     = 5,    // Shake-256
        }

    pub enum EncryptionSubClass<'a> {
        Aes {
            experimental: bool,
            subsubclass: AesSubSubClass,
        },
        XChaCha20 {
            experimental: bool,
            subsubclass: SubSubClass,
        },
        List {
            subsubclass: SubSubClass,
        },
        Undefined {
            subsubclass: SubSubClass,
        },
        Defined {
            name: &'a str,
            experimental: bool,
            subsubclass: SubSubClass,
        },
    }

        pub enum AesSubSubClass {
            OneTwoEight     = 0,    // AES-128
            OneNineTwo      = 1,    // AES-192
            TwoFiveSix      = 2,    // AES-256
        }

    pub enum HmacSubClass<'a> {
        List {
            subsubclass: SubSubClass,
        },
        Undefined {
            subsubclass: SubSubClass,
        },
        Defined {
            name: &'a str,
            experimental: bool,
            subsubclass: SubSubClass,
        },
    }

    pub enum IdentifierSubClass<'a> {
        Adi {
            experimental: bool,
            subsubclass: SubSubClass,
        },
        Did {
            experimental: bool,
            subsubclass: SubSubClass,
        },
        Email {
            experimental: bool,
            subsubclass: SubSubClass,
        },
        List {
            subsubclass: SubSubClass,
        },
        Undefined {
            subsubclass: SubSubClass,
        },
        Defined {
            name: &'a str,
            experimental: bool,
            subsubclass: SubSubClass,
        },
    }

    pub enum KeySubClass<'a> {
        Aes {
            experimental: bool,
            subsubclass:  AesSubSubClass,
        },
        Bls12381 {
            experimental: bool,
            subsubclass: AsymKeySubSubClass,
        },
        ChaCha20 {
            experimental: bool,
            subsubclass: SubSubClass,
        },
        Ed25519 {
            experimental: bool,
            subsubclass: AsymKeySubSubClass,
        },
        K256 {
            experimental: bool,
            subsubclass: AsymKeySubSubClass,
        },
        P256 {
            experimental: bool,
            subsubclass: AsymKeySubSubClass,
        },
        Rsa {
            experimental: bool,
            subsubclass: AsymKeySubSubClass,
        },
        X25519 {
            experimental: bool,
            subsubclass: AsymKeySubSubClass,
        },
        List {
            subsubclass: SubSubClass,
        },
        Undefined {
            subsubclass: SubSubClass,
        },
        Defined {
            name: &'a str,
            experimental: bool,
            subsubclass: SubSubClass,
        },
    }

        pub enum AsymKeySubSubClass {
            Public  = 0,    // Public Key
            Secret  = 1,    // Secret Key
        }

    pub enum NonceSubClass<'a> {
        List {
            subsubclass: SubSubClass,
        },
        Undefined {
            subsubclass: SubSubClass,
        },
        Defined {
            name: &'a str,
            experimental: bool,
            subsubclass: SubSubClass,
        },
    }

    pub enum PolicySubClass<'a> {
        List {
            subsubclass: SubSubClass,
        },
        Undefined {
            subsubclass: SubSubClass,
        },
        Defined {
            name: &'a str,
            experimental: bool,
            subsubclass: SubSubClass,
        },
    }

    pub enum SignatureSubClass<'a> {
        Minisign {
            experimental: bool,
            subsubclass: SubSubClass,
        },
        OpenSSL {
            experimental: bool,
            subsubclass: SubSubClass,
        },
        Pgp {
            experimental: bool,
            subsubclass: SubSubClass,
        },
        X509 {
            experimental: bool,
            subsubclass: SubSubClass,
        },
        List {
            subsubclass: SubSubClass,
        },
        Undefined {
            subsubclass: SubSubClass,
        },
        Defined {
            name: &'a str,
            experimental: bool,
            subsubclass: SubSubClass,
        },
    }

    pub enum TimestampSubClass<'a> {
        Bitcoin {
            experimental: bool,
            subsubclass: SubSubClass,
        },
        Iso8601 {
            experimental: bool,
            subsubclass: SubSubClass,
        },
        Unix {
            experimental: bool,
            subsubclass: SubSubClass,
        },
        List {
            subsubclass: SubSubClass,
        },
        Undefined {
            subsubclass: SubSubClass,
        },
        Defined {
            name: &'a str,
            experimental: bool,
            subsubclass: SubSubClass,
        },
    }

    pub enum ListSubClass {
        List {
            subsubclass: SubSubClass,
        },
    }

    pub enum UndefinedSubClass {
        List {
            subsubclass: SubSubClass,
        },
        Undefined {
            subsubclass: SubSubClass,
        },
    }

    pub enum DefinedSubClass<'a> {
        Defined {
            name: &'a str,
            experimental: bool,
            subsubclass: SubSubClass,
        },
    }

    pub struct SubSubClass(u8);


