use base32::Alphabet::RFC4648;
use byteorder::{
    BigEndian,
    ReadBytesExt,
};
use ring::hmac;
use std::io::Cursor;

/// Two-step verification of HOTP algorithm.
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct HOTP {
    /// A secret token for the authentication
    secret: Vec<u8>,
}

/// Two-step verification of TOTP algorithm
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct TOTP {
    hotp: HOTP,
}

impl HOTP {
    /// Constructs a new `HOTP`
    pub fn new<S: Into<String>>(secret: S) -> HOTP {
        HOTP {
            secret: secret.into().into_bytes(),
        }
    }

    /// Constructs a new `HOTP` with base-32 encoded secret bytes
    pub fn from_base32<S: Into<String>>(secret: S) -> Option<HOTP> {
        base32::decode(RFC4648 { padding: false }, &secret.into())
            .map(|secret| HOTP { secret })
    }

    /// Constructs a new `HOTP` with secret bytes
    pub fn from_bytes(bytes: &[u8]) -> HOTP {
        HOTP {
            secret: bytes.into(),
        }
    }

    pub fn generate(
        &self,
        counter: u64,
    ) -> u32 {
        let key =
            hmac::Key::new(hmac::HMAC_SHA1_FOR_LEGACY_USE_ONLY, &self.secret);
        let wtr = counter.to_be_bytes().to_vec();
        let result = hmac::sign(&key, &wtr);
        let digest = result.as_ref();
        let ob = digest[19];
        let pos = (ob & 15) as usize;
        let mut rdr = Cursor::new(digest[pos..pos + 4].to_vec());
        let base = rdr.read_u32::<BigEndian>().unwrap() & 0x7fff_ffff;
        base % 1_000_000
    }

    /// Return the secret bytes in base32 encoding.
    pub fn base32_secret(&self) -> String {
        base32::encode(RFC4648 { padding: false }, &self.secret)
    }
}

impl TOTP {
    pub fn new<S: Into<String>>(secret: S) -> TOTP {
        TOTP {
            hotp: HOTP::new(secret),
        }
    }

    /// Constructs a new `TOTP` with base-32 encoded secret bytes
    pub fn from_base32<S: Into<String>>(secret: S) -> Option<TOTP> {
        HOTP::from_base32(secret).map(|hotp| TOTP { hotp })
    }

    /// Constructs a new `TOTP` with secret bytes
    pub fn from_bytes(bytes: &[u8]) -> TOTP {
        TOTP {
            hotp: HOTP::from_bytes(bytes),
        }
    }

    pub fn generate(
        &self,
        period: u64,
        timestamp: u64,
    ) -> u32 {
        let counter = timestamp / 30;
        self.hotp.generate(counter)
    }
}
