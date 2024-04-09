pub trait BlockHeaderTrait<const HS: usize> {
    fn new() -> Self;
    fn get_version(&self) -> u32;
    fn get_previous_block_hash(&self) -> [u8; HS];
    fn get_merkle_root_hash(&self) -> [u8; HS];
    fn get_difficulty(&self) -> u32;
    fn get_timestamp(&self) -> u32;
    fn get_bits(&self) -> u32;
    fn get_nonce(&self) -> u32;
}

// Block header is the first part of a block. It contains metadata about the block.
// HS is the hash size. It is the size of the hash in bytes.
pub struct BlockHeader<const HS: usize> {
    version: u32,
    previous_block_hash: [u8; HS],
    merkle_root_hash: [u8; HS],
    difficulty: u32,
    timestamp: u32,
    bits: u32,
    nonce: u32,
}

impl <const HS: usize> BlockHeaderTrait<HS> for BlockHeader<HS> {
    fn new() -> Self {
        todo!()
    }

    fn get_version(&self) -> u32 {
        self.version
    }

    fn get_previous_block_hash(&self) -> [u8; HS] {
        self.previous_block_hash
    }

    fn get_merkle_root_hash(&self) -> [u8; HS] {
        self.merkle_root_hash
    }

    fn get_difficulty(&self) -> u32 {
        self.difficulty
    }

    fn get_timestamp(&self) -> u32 {
        self.timestamp
    }

    fn get_bits(&self) -> u32 {
        self.bits
    }

    fn get_nonce(&self) -> u32 {
        self.nonce
    }
}
