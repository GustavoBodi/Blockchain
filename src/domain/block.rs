use crate::domain::block_header;

pub trait BlockTrait<const TS: usize, const HS: usize, BH>
where BH: Sized + block_header::BlockHeaderTrait<HS>
{
    fn new() -> Self;
    fn get_transactions(&self) -> [u8; TS];
    //fn get_header(&self) -> BH;
}

// TS: Transaction size
// HS: Hash size
struct Block<const TS: usize, const HS: usize, BH>
where BH: Sized + block_header::BlockHeaderTrait<HS>
{
    header: BH,
    transactions: [u8; TS],
}

impl <const TS: usize, const HS: usize, BH> BlockTrait<TS, HS, BH> for Block<TS, HS, BH>
where BH: Sized + block_header::BlockHeaderTrait<HS>
{
    fn new() -> Self {
        todo!()
    }

    fn get_transactions(&self) -> [u8; TS] {
        self.transactions
    }
}
