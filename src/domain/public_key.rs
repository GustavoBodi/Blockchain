pub trait PublicKeyTrait<const SIZE: usize> {
    fn new() -> Self;
    fn get_key(&self) -> [u8; SIZE];
    fn to_string(&self) -> String;
}

pub struct PublicKey<const SIZE: usize> {
    key: [u8; SIZE],
}

impl <const SIZE: usize>PublicKeyTrait<SIZE> for PublicKey<SIZE> {
    fn new() -> Self {
        todo!()
    }

    fn get_key(&self) -> [u8; SIZE] {
        self.key
    }

    fn to_string(&self) -> String {
        std::str::from_utf8(&self.key).unwrap().to_string()
    }
}

