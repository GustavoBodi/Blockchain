use crate::domain::public_key::PublicKeyTrait;
use crate::domain::public_key::PublicKey;

trait TransactionTrait<const PK_SIZE: usize, PK>
where PK: Sized + PublicKeyTrait<PK_SIZE>
{
    fn new() -> Self;
    fn get_id(&self) -> u64;
    fn get_from(&self) -> String;
    fn get_to(&self) -> String;
    fn get_amount(&self) -> u64;
    fn get_timestamp(&self) -> u64;
}

struct Transaction<const SIZE: usize> {
    pub id: u64,
    pub from: PublicKey<SIZE>,
    pub to: PublicKey<SIZE>,
    pub amount: u64,
    pub timestamp: u64,
}

impl <const PK_SIZE: usize, PK> TransactionTrait<PK_SIZE, PK> for Transaction<PK_SIZE>
where PK: Sized + PublicKeyTrait<PK_SIZE>
{
    fn new() -> Self {
        todo!()
    }

    fn get_id(&self) -> u64 {
        self.id
    }

    fn get_from(&self) -> String {
        self.from.to_string()
    }

    fn get_to(&self) -> String {
        self.to.to_string()
    }

    fn get_amount(&self) -> u64 {
        self.amount
    }

    fn get_timestamp(&self) -> u64 {
        self.timestamp
    }
}
