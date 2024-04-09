use oqs::*;

pub trait KeyPairTrait<KEYCORE> {
    fn new() -> Self;
    fn from_str(str: String) -> Self;
    fn from_bytes(bytes: &[u8]) -> Self;
    fn from_private_key(private_key: &impl PrivateKeyTrait<KEYCORE>) -> Self;
    fn public_as_str(&self) -> String;
    fn private_as_str(&self) -> String;
    fn sign(&self, data: &[u8]) -> Vec<u8>;
    fn verify(&self, data: &[u8], signature: &[u8]) -> bool;
}

pub trait PublicKeyTrait<KEYCORE> {
    fn new() -> Self;
    fn from_str(str: String) -> Self;
    fn from_bytes(bytes: &[u8]) -> Self;
    fn as_str(&self) -> String;
    fn verify(&self, data: &[u8], signature: &[u8]) -> bool;
}

pub trait PrivateKeyTrait<KEYCORE> {
    fn new() -> Self;
    fn from_str(str: String) -> Self;
    fn from_bytes(bytes: &[u8]) -> Self;
    fn as_str(&self) -> String;
    fn sign(&self, data: &[u8]) -> Vec<u8>;
}

pub trait KeyCore<const SIZE: usize>
where Self: Sized
{
    fn new() -> Result<Self>;
    fn get_public_key(&self) -> &[u8];
    fn get_private_key(&self) -> &[u8];
    fn as_str(&self) -> String;
}

pub struct Dilithium2Core {
    dilithium_key: sig::Sig
}

impl <const PBSIZE: usize> KeyCore<PBSIZE> for Dilithium2Core {
    fn new() -> Result<Self> {
        let dilithium_key = sig::Sig::new(sig::Algorithm::Dilithium2)?;
        Ok(Self { dilithium_key })
    }

    fn get_public_key(&self) -> &[u8] {
        self.dilithium_key.keypair().unwrap().0.as_ref()
    }

    fn get_private_key(&self) -> &[u8] {
        self.dilithium_key.keypair().unwrap().1.as_ref()
    }

    fn as_str(&self) -> String {
    }
}

pub struct Key<const SIZE: usize, CORE>
where CORE: KeyCore<SIZE>
{
    key_core: CORE,
}
