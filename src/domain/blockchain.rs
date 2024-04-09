use crate::domain::block::BlockTrait;
use crate::domain::block_header;
use std::marker::PhantomData;

pub trait BlockchainTrait {

}

struct Blockchain<BLK, BH, const TS: usize, const HS: usize>
where BH: Sized + block_header::BlockHeaderTrait<HS>,
      BLK: Sized + BlockTrait<TS, HS, BH>,
{
    blocks: Vec<BLK>,
    phantom: PhantomData<BH>,
}
