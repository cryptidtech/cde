use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

static CDE_ALPHABET: &'static str = "abcdefghijklmnopqrstuvwxyz01234-ABCDEFGHIJKLMNOPQRSTUVWXYZ56789_";

fn idx(c: char) -> u8 {                                                        
    if let Some(i) = CDE_ALPHABET.find(c) {                                        
        i as u8                                                                    
    } else {                                                                       
        63                                                                         
    }                                                                              
}

fn main() -> Result<(), Box<(dyn Error + 'static)>> {

    let path = Path::new(&env::var("OUT_DIR").unwrap()).join("hashmaps.rs");
    let mut file = BufWriter::new(File::create(&path)?);

    writeln!(&mut file, "type SubSubNamesMap = phf::OrderedMap<u8, &'static str>;")?;
    writeln!(&mut file, "type SubNamesMap<'a> = phf::OrderedMap<u8, (&'static str, Option<&'a SubSubNamesMap>)>;")?;
    writeln!(&mut file, "type NamesMap<'a> = phf::OrderedMap<u8, (&'static str, &'a SubNamesMap<'a>)>;")?;

    writeln!(
        &mut file,
        "static NAMES: NamesMap = \n{};\n\n",
        phf_codegen::OrderedMap::new()
            .entry(idx('a'), "(\"aead\", &NAMES_AEAD)")
            .entry(idx('c'), "(\"claim\", &NAMES_CLAIM)")
            .entry(idx('d'), "(\"digest\", &NAMES_DIGEST)")
            .entry(idx('e'), "(\"encryption\", &NAMES_ENCRYPTION)")
            .entry(idx('h'), "(\"hmac\", &NAMES_HMAC)")
            .entry(idx('i'), "(\"identifier\", &NAMES_IDENTIFIER)")
            .entry(idx('k'), "(\"key\", &NAMES_KEY)")
            .entry(idx('n'), "(\"nonce\", &NAMES_NONCE)")
            .entry(idx('p'), "(\"policy\", &NAMES_POLICY)")
            .entry(idx('s'), "(\"signature\", &NAMES_SIGNATURE)")
            .entry(idx('t'), "(\"timestamp\", &NAMES_TIMESTAMP)")
            .entry(idx('-'), "(\"list\", &NAMES_LIST)")
            .entry(idx('_'), "(\"undefined\", &NAMES_UNDEFINED)")
            .entry(idx('A'), "(\"Aead\", &NAMES_AEAD)")
            .entry(idx('C'), "(\"Claim\", &NAMES_CLAIM)")
            .entry(idx('D'), "(\"Digest\", &NAMES_DIGEST)")
            .entry(idx('E'), "(\"Encryption\", &NAMES_ENCRYPTION)")
            .entry(idx('H'), "(\"Hmac\", &NAMES_HMAC)")
            .entry(idx('I'), "(\"Identifier\", &NAMES_IDENTIFIER)")
            .entry(idx('K'), "(\"Key\", &NAMES_KEY)")
            .entry(idx('N'), "(\"Nonce\", &NAMES_NONCE)")
            .entry(idx('P'), "(\"Policy\", &NAMES_POLICY)")
            .entry(idx('S'), "(\"Signature\", &NAMES_SIGNATURE)")
            .entry(idx('T'), "(\"Timestamp\", &NAMES_TIMESTAMP)")
            .build()
    )?;

        writeln!(
            &mut file,
            "static NAMES_AEAD: SubNamesMap = \n{};\n\n",
            phf_codegen::OrderedMap::new()
                .entry(idx('_'), "(\"undefined\", None)")
                .entry(idx('-'), "(\"list\", None)")
                .entry(idx('a'), "(\"aes256-gcm\", None)")
                .entry(idx('c'), "(\"chacha20-poly1305\", None)")
                .entry(idx('i'), "(\"chacha20-poly1305-ietf\", None)")
                .entry(idx('x'), "(\"xchacha20-poly1305-ietf\", None)")
                .entry(idx('A'), "(\"Aes256-gcm\", None)")
                .entry(idx('C'), "(\"Chacha20-poly1305\", None)")
                .entry(idx('I'), "(\"Chacha20-poly1305-ietf\", None)")
                .entry(idx('X'), "(\"Xchacha20-poly1305-ietf\", None)")
                .build()
        )?;

        writeln!(
            &mut file,
            "static NAMES_CLAIM: SubNamesMap = \n{};\n\n",
            phf_codegen::OrderedMap::new()
                .entry(idx('_'), "(\"undefined\", None)")
                .entry(idx('-'), "(\"list\", None)")
                .entry(idx('o'), "(\"oberon\", None)")
                .entry(idx('O'), "(\"oberon\", None)")
                .build()
        )?;

        writeln!(
            &mut file,
            "static NAMES_DIGEST: SubNamesMap = \n{};\n\n",
            phf_codegen::OrderedMap::new()
                .entry(idx('_'), "(\"undefined\", None)")
                .entry(idx('-'), "(\"list\", None)")
                .entry(idx('b'), "(\"blake2\", Some(&NAMES_DIGEST_BLAKE2))")
                .entry(idx('m'), "(\"md\", Some(&NAMES_DIGEST_MD))")
                .entry(idx('s'), "(\"sha1\", None)")
                .entry(idx('h'), "(\"sha2\", Some(&NAMES_DIGEST_SHA2))")
                .entry(idx('a'), "(\"sha3\", Some(&NAMES_DIGEST_SHA3))")
                .entry(idx('B'), "(\"Blake2\", Some(&NAMES_DIGEST_BLAKE2))")
                .entry(idx('M'), "(\"Md\", Some(&NAMES_DIGEST_MD))")
                .entry(idx('S'), "(\"Sha1\", None)")
                .entry(idx('H'), "(\"Sha2\", Some(&NAMES_DIGEST_SHA2))")
                .entry(idx('A'), "(\"Sha3\", Some(&NAMES_DIGEST_SHA3))")
                .build()
        )?;

            writeln!(
                &mut file,
                "static NAMES_DIGEST_BLAKE2: SubSubNamesMap = \n{};\n\n",
                phf_codegen::OrderedMap::new()
                    .entry(1, "\"b\"")
                    .entry(2, "\"s\"")
                    .build()
            )?;

            writeln!(
                &mut file,
                "static NAMES_DIGEST_MD: SubSubNamesMap = \n{};\n\n",
                phf_codegen::OrderedMap::new()
                    .entry(1, "\"5\"")
                    .entry(2, "\"4\"")
                    .entry(3, "\"2\"")
                    .entry(4, "\"6\"")
                    .build()
            )?;

            writeln!(
                &mut file,
                "static NAMES_DIGEST_SHA2: SubSubNamesMap = \n{};\n\n",
                phf_codegen::OrderedMap::new()
                    .entry(1, "\"256\"")
                    .entry(2, "\"512\"")
                    .entry(3, "\"224\"")
                    .entry(4, "\"384\"")
                    .entry(5, "\"512/224\"")
                    .entry(6, "\"512/256\"")
                    .build()
            )?;

            writeln!(
                &mut file,
                "static NAMES_DIGEST_SHA3: SubSubNamesMap = \n{};\n\n",
                phf_codegen::OrderedMap::new()
                    .entry(1, "\"256\"")
                    .entry(2, "\"512\"")
                    .entry(3, "\"224\"")
                    .entry(4, "\"384\"")
                    .entry(5, "\"shake128\"")
                    .entry(6, "\"shake256\"")
                    .build()
            )?;

        writeln!(
            &mut file,
            "static NAMES_ENCRYPTION: SubNamesMap = \n{};\n\n",
            phf_codegen::OrderedMap::new()
                .entry(idx('_'), "(\"undefined\", None)")
                .entry(idx('-'), "(\"list\", None)")
                .entry(idx('a'), "(\"aes\", Some(&NAMES_CIPHER_AES))")
                .entry(idx('x'), "(\"xchacha20\", None)")
                .entry(idx('A'), "(\"Aes\", Some(&NAMES_CIPHER_AES))")
                .entry(idx('X'), "(\"Xchacha20\", None)")
                .build()
        )?;

            writeln!(
                &mut file,
                "static NAMES_CIPHER_AES: SubSubNamesMap = \n{};\n\n",
                phf_codegen::OrderedMap::new()
                    .entry(1, "\"256\"")
                    .entry(2, "\"128\"")
                    .entry(3, "\"192\"")
                    .build()
            )?;

        writeln!(
            &mut file,
            "static NAMES_HMAC: SubNamesMap = \n{};\n\n",
            phf_codegen::OrderedMap::new()
                .entry(idx('_'), "(\"undefined\", None)")
                .entry(idx('-'), "(\"list\", None)")
                .build()
        )?;

        writeln!(
            &mut file,
            "static NAMES_IDENTIFIER: SubNamesMap = \n{};\n\n",
            phf_codegen::OrderedMap::new()
                .entry(idx('_'), "(\"undefined\", None)")
                .entry(idx('-'), "(\"list\", None)")
                .entry(idx('a'), "(\"adi\", None)")
                .entry(idx('d'), "(\"did\", None)")
                .entry(idx('e'), "(\"email\", None)")
                .entry(idx('A'), "(\"Adi\", None)")
                .entry(idx('D'), "(\"Did\", None)")
                .entry(idx('E'), "(\"Email\", None)")
                .build()
        )?;

        writeln!(
            &mut file,
            "static NAMES_KEY: SubNamesMap = \n{};\n\n",
            phf_codegen::OrderedMap::new()
                .entry(idx('_'), "(\"undefined\", None)")
                .entry(idx('-'), "(\"list\", None)")
                .entry(idx('e'), "(\"ed25519\", Some(&NAMES_KEY_ED25519))")
                .entry(idx('x'), "(\"x25519\", Some(&NAMES_KEY_X25519))")
                .entry(idx('r'), "(\"rsa\", Some(&NAMES_KEY_RSA))")
                .entry(idx('b'), "(\"bls12381\", Some(&NAMES_KEY_BLS12381))")
                .entry(idx('k'), "(\"k256\", Some(&NAMES_KEY_K256))")
                .entry(idx('p'), "(\"p256\", Some(&NAMES_KEY_P256))")
                .entry(idx('c'), "(\"chacha20\", None)")
                .entry(idx('a'), "(\"aes\", Some(&NAMES_KEY_AES))")
                .entry(idx('E'), "(\"Ed25519\", Some(&NAMES_KEY_ED25519))")
                .entry(idx('X'), "(\"X25519\", Some(&NAMES_KEY_X25519))")
                .entry(idx('R'), "(\"Rsa\", Some(&NAMES_KEY_RSA))")
                .entry(idx('B'), "(\"Bls12381\", Some(&NAMES_KEY_BLS12381))")
                .entry(idx('K'), "(\"K256\", Some(&NAMES_KEY_K256))")
                .entry(idx('P'), "(\"P256\", Some(&NAMES_KEY_P256))")
                .entry(idx('C'), "(\"Chacha20\", None)")
                .entry(idx('A'), "(\"Aes\", Some(&NAMES_KEY_AES))")
                .build()
        )?;

            writeln!(
                &mut file,
                "static NAMES_KEY_ED25519: SubSubNamesMap = \n{};\n\n",
                phf_codegen::OrderedMap::new()
                    .entry(1, "\"public\"")
                    .entry(2, "\"secret\"")
                    .build()
            )?;

            writeln!(
                &mut file,
                "static NAMES_KEY_X25519: SubSubNamesMap = \n{};\n\n",
                phf_codegen::OrderedMap::new()
                    .entry(1, "\"public\"")
                    .entry(2, "\"secret\"")
                    .build()
            )?;

            writeln!(
                &mut file,
                "static NAMES_KEY_RSA: SubSubNamesMap = \n{};\n\n",
                phf_codegen::OrderedMap::new()
                    .entry(1, "\"public\"")
                    .entry(2, "\"secret\"")
                    .build()
            )?;

            writeln!(
                &mut file,
                "static NAMES_KEY_BLS12381: SubSubNamesMap = \n{};\n\n",
                phf_codegen::OrderedMap::new()
                    .entry(1, "\"public\"")
                    .entry(2, "\"secret\"")
                    .build()
            )?;

            writeln!(
                &mut file,
                "static NAMES_KEY_K256: SubSubNamesMap = \n{};\n\n",
                phf_codegen::OrderedMap::new()
                    .entry(1, "\"public\"")
                    .entry(2, "\"secret\"")
                    .build()
            )?;

            writeln!(
                &mut file,
                "static NAMES_KEY_P256: SubSubNamesMap = \n{};\n\n",
                phf_codegen::OrderedMap::new()
                    .entry(1, "\"public\"")
                    .entry(2, "\"secret\"")
                    .build()
            )?;

            writeln!(
                &mut file,
                "static NAMES_KEY_AES: SubSubNamesMap = \n{};\n\n",
                phf_codegen::OrderedMap::new()
                    .entry(1, "\"128\"")
                    .entry(2, "\"256\"")
                    .build()
            )?;

        writeln!(
            &mut file,
            "static NAMES_NONCE: SubNamesMap = \n{};\n\n",
            phf_codegen::OrderedMap::new()
                .entry(idx('_'), "(\"undefined\", None)")
                .entry(idx('-'), "(\"list\", None)")
                .build()
        )?;

        writeln!(
            &mut file,
            "static NAMES_POLICY: SubNamesMap = \n{};\n\n",
            phf_codegen::OrderedMap::new()
                .entry(idx('_'), "(\"undefined\", None)")
                .entry(idx('-'), "(\"list\", None)")
                .entry(idx('b'), "(\"bitcoin\", None)")
                .entry(idx('s'), "(\"solidity\", None)")
                .entry(idx('B'), "(\"Bitcoin\", None)")
                .entry(idx('S'), "(\"Solidity\", None)")
                .build()
        )?;

        writeln!(
            &mut file,
            "static NAMES_SIGNATURE: SubNamesMap = \n{};\n\n",
            phf_codegen::OrderedMap::new()
                .entry(idx('_'), "(\"undefined\", None)")
                .entry(idx('-'), "(\"list\", None)")
                .entry(idx('m'), "(\"minisign\", None)")
                .entry(idx('o'), "(\"openssl\", None)")
                .entry(idx('p'), "(\"pgp\", None)")
                .entry(idx('x'), "(\"x509\", None)")
                .entry(idx('M'), "(\"Minisign\", None)")
                .entry(idx('O'), "(\"Openssl\", None)")
                .entry(idx('P'), "(\"Pgp\", None)")
                .entry(idx('X'), "(\"X509\", None)")
                .build()
        )?;

        writeln!(
            &mut file,
            "static NAMES_TIMESTAMP: SubNamesMap = \n{};\n\n",
            phf_codegen::OrderedMap::new()
                .entry(idx('_'), "(\"undefined\", None)")
                .entry(idx('-'), "(\"list\", None)")
                .entry(idx('u'), "(\"unix\", None)")
                .entry(idx('i'), "(\"iso8601\", None)")
                .entry(idx('b'), "(\"bitcoin\", None)")
                .entry(idx('U'), "(\"Unix\", None)")
                .entry(idx('I'), "(\"Iso8601\", None)")
                .entry(idx('B'), "(\"Bitcoin\", None)")
                .build()
        )?;

        writeln!(
            &mut file,
            "static NAMES_LIST: SubNamesMap = \n{};\n\n",
            phf_codegen::OrderedMap::new()
                .entry(idx('-'), "(\"list\", None)")
                .build()
        )?;

        writeln!(
            &mut file,
            "static NAMES_UNDEFINED: SubNamesMap = \n{};\n\n",
            phf_codegen::OrderedMap::new()
                .entry(idx('_'), "(\"undefined\", None)")
                .entry(idx('-'), "(\"list\", None)")
                .build()
        )?;

    writeln!(&mut file, "type SubSubValuesMap = phf::OrderedMap<&'static str, u8>;")?;
    writeln!(&mut file, "type SubValuesMap<'a> = phf::OrderedMap<&'static str, (u8, Option<&'a SubSubValuesMap>)>;")?;
    writeln!(&mut file, "type ValuesMap<'a> = phf::OrderedMap<&'static str, (u8, &'a SubValuesMap<'a>)>;")?;

    writeln!(
        &mut file,
        "static VALUES: ValuesMap = \n{};\n\n",
        phf_codegen::OrderedMap::new()
            .entry("aead", "(0, &VALUES_AEAD)")
            .entry("claim", "(2, &VALUES_CLAIM)")
            .entry("digest", "(3, &VALUES_DIGEST)")
            .entry("encryption", "(4, &VALUES_ENCRYPTION)")
            .entry("hmac", "(7, &VALUES_HMAC)")
            .entry("identifier", "(8, &VALUES_IDENTIFIER)")
            .entry("key", "(10, &VALUES_KEY)")
            .entry("nonce", "(13, &VALUES_NONCE)")
            .entry("policy", "(15, &VALUES_POLICY)")
            .entry("signature", "(18, &VALUES_SIGNATURE)")
            .entry("timestamp", "(19, &VALUES_TIMESTAMP)")
            .entry("list", "(31, &VALUES_LIST)")
            .entry("undefined", "(63, &VALUES_UNDEFINED)")
            .entry("Aead", "(32, &VALUES_AEAD)")
            .entry("Claim", "(34, &VALUES_CLAIM)")
            .entry("Digest", "(35, &VALUES_DIGEST)")
            .entry("Encryption", "(36, &VALUES_ENCRYPTION)")
            .entry("Hmac", "(39, &VALUES_HMAC)")
            .entry("Identifier", "(40, &VALUES_IDENTIFIER)")
            .entry("Key", "(42, &VALUES_KEY)")
            .entry("Nonce", "(45, &VALUES_NONCE)")
            .entry("Policy", "(47, &VALUES_POLICY)")
            .entry("Signature", "(50, &VALUES_SIGNATURE)")
            .entry("Timestamp", "(51, &VALUES_TIMESTAMP)")
            .build()
    )?;

        writeln!(
            &mut file,
            "static VALUES_AEAD: SubValuesMap = \n{};\n\n",
            phf_codegen::OrderedMap::new()
                .entry("undefined", "(63, None)")
                .entry("list", "(31, None)")
                .entry("aes256-gcm", "(0, None)")
                .entry("chacha20-poly1305", "(2, None)")
                .entry("chacha20-poly1305-ietf", "(8, None)")
                .entry("xchacha20-poly1305-ietf", "(23, None)")
                .entry("Aes256-gcm", "(32, None)")
                .entry("Chacha20-poly1305", "(34, None)")
                .entry("Chacha20-poly1305-ietf", "(40, None)")
                .entry("Xchacha20-poly1305-ietf", "(55, None)")
                .build()
        )?;

        writeln!(
            &mut file,
            "static VALUES_CLAIM: SubValuesMap = \n{};\n\n",
            phf_codegen::OrderedMap::new()
                .entry("undefined", "(63, None)")
                .entry("list", "(31, None)")
                .entry("oberon", "(14, None)")
                .entry("Oberon", "(46, None)")
                .build()
        )?;

        writeln!(
            &mut file,
            "static VALUES_DIGEST: SubValuesMap = \n{};\n\n",
            phf_codegen::OrderedMap::new()
                .entry("undefined", "(63, None)")
                .entry("list", "(31, None)")
                .entry("blake2", "(1, Some(&VALUES_DIGEST_BLAKE2))")
                .entry("md", "(12, Some(&VALUES_DIGEST_MD))")
                .entry("sha1", "(18, None)")
                .entry("sha2", "(7, Some(&VALUES_DIGEST_SHA2))")
                .entry("sha3", "(0, Some(&VALUES_DIGEST_SHA3))")
                .entry("Blake2", "(33, Some(&VALUES_DIGEST_BLAKE2))")
                .entry("Md", "(44, Some(&VALUES_DIGEST_MD))")
                .entry("Sha1", "(50, None)")
                .entry("Sha2", "(39, Some(&VALUES_DIGEST_SHA2))")
                .entry("Sha3", "(32, Some(&VALUES_DIGEST_SHA3))")
                .build()
        )?;

            writeln!(
                &mut file,
                "static VALUES_DIGEST_BLAKE2: SubSubValuesMap = \n{};\n\n",
                phf_codegen::OrderedMap::new()
                    .entry("b", "1")
                    .entry("s", "2")
                    .build()
            )?;

            writeln!(
                &mut file,
                "static VALUES_DIGEST_MD: SubSubValuesMap = \n{};\n\n",
                phf_codegen::OrderedMap::new()
                    .entry("5", "1")
                    .entry("4", "2")
                    .entry("2", "3")
                    .entry("6", "4")
                    .build()
            )?;

            writeln!(
                &mut file,
                "static VALUES_DIGEST_SHA2: SubSubValuesMap = \n{};\n\n",
                phf_codegen::OrderedMap::new()
                    .entry("256", "1")
                    .entry("512", "2")
                    .entry("224", "3")
                    .entry("384", "4")
                    .entry("512/224", "5")
                    .entry("512/256", "6")
                    .build()
            )?;

            writeln!(
                &mut file,
                "static VALUES_DIGEST_SHA3: SubSubValuesMap = \n{};\n\n",
                phf_codegen::OrderedMap::new()
                    .entry("256", "1")
                    .entry("512", "2")
                    .entry("224", "3")
                    .entry("384", "4")
                    .entry("shake128", "5")
                    .entry("shake256", "6")
                    .build()
            )?;

        writeln!(
            &mut file,
            "static VALUES_ENCRYPTION: SubValuesMap = \n{};\n\n",
            phf_codegen::OrderedMap::new()
                .entry("undefined", "(63, None)")
                .entry("list", "(31, None)")
                .entry("aes", "(0, Some(&VALUES_CIPHER_AES))")
                .entry("xchacha20", "(23, None)")
                .entry("Aes", "(32, Some(&VALUES_CIPHER_AES))")
                .entry("Xchacha20", "(55, None)")
                .build()
        )?;

            writeln!(
                &mut file,
                "static VALUES_CIPHER_AES: SubSubValuesMap = \n{};\n\n",
                phf_codegen::OrderedMap::new()
                    .entry("256", "1")
                    .entry("128", "2")
                    .entry("192", "3")
                    .build()
            )?;

        writeln!(
            &mut file,
            "static VALUES_HMAC: SubValuesMap = \n{};\n\n",
            phf_codegen::OrderedMap::new()
                .entry("list", "(31, None)")
                .build()
        )?;

        writeln!(
            &mut file,
            "static VALUES_IDENTIFIER: SubValuesMap = \n{};\n\n",
            phf_codegen::OrderedMap::new()
                .entry("undefined", "(63, None)")
                .entry("list", "(31, None)")
                .entry("adi", "(0, None)")
                .entry("did", "(3, None)")
                .entry("email", "(4, None)")
                .entry("Adi", "(32, None)")
                .entry("Did", "(35, None)")
                .entry("Email", "(36, None)")
                .build()
        )?;

        writeln!(
            &mut file,
            "static VALUES_KEY: SubValuesMap = \n{};\n\n",
            phf_codegen::OrderedMap::new()
                .entry("undefined", "(63, None)")
                .entry("list", "(31, None)")
                .entry("ed25519", "(4, Some(&VALUES_KEY_ED25519))")
                .entry("x25519", "(23, Some(&VALUES_KEY_X25519))")
                .entry("rsa", "(17, Some(&VALUES_KEY_RSA))")
                .entry("bls12381", "(1, Some(&VALUES_KEY_BLS12381))")
                .entry("k256", "(10, Some(&VALUES_KEY_K256))")
                .entry("p256", "(15, Some(&VALUES_KEY_P256))")
                .entry("xchacha20", "(2, None)")
                .entry("aes", "(0, Some(&VALUES_KEY_AES))")
                .entry("Ed25519", "(36, Some(&VALUES_KEY_ED25519))")
                .entry("X25519", "(55, Some(&VALUES_KEY_X25519))")
                .entry("Rsa", "(49, Some(&VALUES_KEY_RSA))")
                .entry("Bls12381", "(33, Some(&VALUES_KEY_BLS12381))")
                .entry("K256", "(42, Some(&VALUES_KEY_K256))")
                .entry("P256", "(47, Some(&VALUES_KEY_P256))")
                .entry("Xchacha20", "(34, None)")
                .entry("Aes", "(32, Some(&VALUES_KEY_AES))")
                .build()
        )?;

            writeln!(
                &mut file,
                "static VALUES_KEY_ED25519: SubSubValuesMap = \n{};\n\n",
                phf_codegen::OrderedMap::new()
                    .entry("public", "1")
                    .entry("secret", "2")
                    .build()
            )?;

            writeln!(
                &mut file,
                "static VALUES_KEY_X25519: SubSubValuesMap = \n{};\n\n",
                phf_codegen::OrderedMap::new()
                    .entry("public", "1")
                    .entry("secret", "2")
                    .build()
            )?;

            writeln!(
                &mut file,
                "static VALUES_KEY_RSA: SubSubValuesMap = \n{};\n\n",
                phf_codegen::OrderedMap::new()
                    .entry("public", "1")
                    .entry("secret", "2")
                    .build()
            )?;

            writeln!(
                &mut file,
                "static VALUES_KEY_BLS12381: SubSubValuesMap = \n{};\n\n",
                phf_codegen::OrderedMap::new()
                    .entry("public", "1")
                    .entry("secret", "2")
                    .build()
            )?;

            writeln!(
                &mut file,
                "static VALUES_KEY_K256: SubSubValuesMap = \n{};\n\n",
                phf_codegen::OrderedMap::new()
                    .entry("public", "1")
                    .entry("secret", "2")
                    .build()
            )?;

            writeln!(
                &mut file,
                "static VALUES_KEY_P256: SubSubValuesMap = \n{};\n\n",
                phf_codegen::OrderedMap::new()
                    .entry("public", "1")
                    .entry("secret", "2")
                    .build()
            )?;

            writeln!(
                &mut file,
                "static VALUES_KEY_AES: SubSubValuesMap = \n{};\n\n",
                phf_codegen::OrderedMap::new()
                    .entry("128", "1")
                    .entry("256", "2")
                    .build()
            )?;

        writeln!(
            &mut file,
            "static VALUES_NONCE: SubValuesMap = \n{};\n\n",
            phf_codegen::OrderedMap::new()
                .entry("list", "(31, None)")
                .build()
        )?;

        writeln!(
            &mut file,
            "static VALUES_POLICY: SubValuesMap = \n{};\n\n",
            phf_codegen::OrderedMap::new()
                .entry("undefined", "(63, None)")
                .entry("list", "(31, None)")
                .entry("bitcoin", "(1, None)")
                .entry("solidity", "(18, None)")
                .entry("Bitcoin", "(33, None)")
                .entry("Solidity", "(50, None)")
                .build()
        )?;

        writeln!(
            &mut file,
            "static VALUES_SIGNATURE: SubValuesMap = \n{};\n\n",
            phf_codegen::OrderedMap::new()
                .entry("undefined", "(63, None)")
                .entry("list", "(31, None)")
                .entry("minisign", "(12, None)")
                .entry("openssl", "(14, None)")
                .entry("pgp", "(15, None)")
                .entry("x509", "(23, None)")
                .entry("Minisign", "(44, None)")
                .entry("Openssl", "(46, None)")
                .entry("Pgp", "(47, None)")
                .entry("X509", "(55, None)")
                .build()
        )?;

        writeln!(
            &mut file,
            "static VALUES_TIMESTAMP: SubValuesMap = \n{};\n\n",
            phf_codegen::OrderedMap::new()
                .entry("undefined", "(63, None)")
                .entry("list", "(31, None)")
                .entry("bitcoin", "(1, None)")
                .entry("iso8601", "(8, None)")
                .entry("unix", "(20, None)")
                .entry("Bitcoin", "(33, None)")
                .entry("Iso8601", "(40, None)")
                .entry("Unix", "(52, None)")
                .build()
        )?;

        writeln!(
            &mut file,
            "static VALUES_LIST: SubValuesMap = \n{};\n\n",
            phf_codegen::OrderedMap::new()
                .entry("list", "(31, None)")
                .build()
        )?;

        writeln!(
            &mut file,
            "static VALUES_UNDEFINED: SubValuesMap = \n{};\n\n",
            phf_codegen::OrderedMap::new()
                .entry("list", "(31, None)")
                .entry("undefined", "(63, None)")
                .build()
        )?;

    Ok(())
}
