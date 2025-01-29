use base64ct::{Base64, Encoding};
use hex::ToHex;
use sha1::{Digest, Sha1};

pub fn sha1_encode(bytes: impl AsRef<[u8]>) -> String {
    let mut sha1 = Sha1::new();
    sha1.update(bytes);
    sha1.finalize().encode_hex()
}

pub fn base64_encode(bytes: impl AsRef<[u8]>) -> String {
    Base64::encode_string(bytes.as_ref())
}
