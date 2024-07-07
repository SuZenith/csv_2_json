use std::fs;
use std::io::Read;
use anyhow::Error;
use base64::Engine;
use ed25519_dalek::{Signature, Signer, SigningKey};
use crate::{get_reader, TextSignFormat};

trait TextSign {
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>, anyhow::Error>;
}

trait TextVerify {
    fn verify(&self, reader: impl Read, sig: &[u8]) -> bool;
}

struct Blake3 {
    key: [u8; 32],
}

struct Ed25519 {
    key: SigningKey,
}

impl TextSign for Blake3 {
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>, Error> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        let hash = blake3::keyed_hash(&self.key, &buf).as_bytes().to_vec();

        Ok(hash)
    }
}

impl TextVerify for Blake3 {
    fn verify(&self, mut reader: impl Read, sig: &[u8]) -> bool {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf).unwrap();
        let hash = blake3::keyed_hash(&self.key, &buf);
        let hash = hash.as_bytes().to_vec();

        hash == sig
    }
}

impl TextSign for Ed25519 {
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>, Error> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        let hash = self.key.sign(&buf);

        Ok(hash.to_vec())
    }

}

impl TextVerify for Ed25519 {
    fn verify(&self, mut reader: impl Read, sig: &[u8]) -> bool {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf).unwrap();
        let sig = Signature::from_bytes(&sig.try_into().unwrap());
        let hash = self.key.verify(&buf, &sig);

        hash.is_ok()
    }
}

pub fn process_sign(input: &str, key: &str, format: TextSignFormat) -> Result<(), anyhow::Error> {
    let mut reader = get_reader(input);

    let key = fs::read(key)?;
    let key = &key[..32];
    let key: [u8; 32] = key.try_into()?;

    let encoded = match format {
        TextSignFormat::Blake3 => {
            let signer = Blake3 { key };
            signer.sign(&mut reader)?
        },
        TextSignFormat::Ed25519 => {
            let key = SigningKey::try_from(key)?;
            let signer = Ed25519 { key };
            signer.sign(&mut reader)?
        },
    };
    println!("{:?}", base64::engine::general_purpose::STANDARD.encode(&encoded));
    Ok(())
}

pub fn process_verify(input: &str, key: &str, sig: &str, format: TextSignFormat) -> Result<(), anyhow::Error> {
    let reader = get_reader(input);
    let key = fs::read(key)?;
    let key = &key[..32];
    let key: [u8; 32] = key.try_into()?;
    let sig = base64::engine::general_purpose::STANDARD.decode(sig).unwrap();

    let result = match format {
        TextSignFormat::Blake3 => {
            let verifier = Blake3 { key };
            verifier.verify(reader, &sig)
        },
        TextSignFormat::Ed25519 => {
            let key = SigningKey::try_from(key)?;
            let verifier = Ed25519 { key };
            verifier.verify(reader, &sig)
        },
    };
    println!("{}", result);
    Ok(())
}