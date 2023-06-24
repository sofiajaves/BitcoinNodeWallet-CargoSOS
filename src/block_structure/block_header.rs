use super::{
    block_version::BlockVersion,
    compact256::Compact256,
    hash::{hash256d, HashType},
    merkle_tree::MerkleTree,
    transaction::Transaction,
};

use crate::{
    messages::compact_size::CompactSize,
    serialization::{
        deserializable_big_endian::DeserializableBigEndian,
        deserializable_internal_order::DeserializableInternalOrder,
        deserializable_little_endian::DeserializableLittleEndian,
        error_serialization::ErrorSerialization, serializable_big_endian::SerializableBigEndian,
        serializable_internal_order::SerializableInternalOrder,
        serializable_little_endian::SerializableLittleEndian,
    },
};

use std::io::{Read, Write};

const GENESIS_BLOCK_VERSION: BlockVersion = BlockVersion::version(1);
const GENESIS_PREVIOUS_BLOCK_HEADER_HASH: HashType = [0; 32];
const GENESIS_MERKLE_ROOT_HASH: HashType = [
    0x3b, 0xa3, 0xed, 0xfd, 0x7a, 0x7b, 0x12, 0xb2, 0x7a, 0xc7, 0x2c, 0x3e, 0x67, 0x76, 0x8f, 0x61,
    0x7f, 0xc8, 0x1b, 0xc3, 0x88, 0x8a, 0x51, 0x32, 0x3a, 0x9f, 0xb8, 0xaa, 0x4b, 0x1e, 0x5e, 0x4a,
];
const GENESIS_TIME: u32 = 0x4d49e5da;
const GENESIS_N_BITS: u32 = 0x1d00ffff;
const GENESIS_NONCE: u32 = 0x18aea41a;
const GENESIS_TRANSACTION_COUNT: u64 = 0;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct BlockHeader {
    pub version: BlockVersion,
    pub previous_block_header_hash: HashType,
    pub merkle_root_hash: HashType,
    pub time: u32,
    pub n_bits: Compact256,
    pub nonce: u32,
    pub transaction_count: CompactSize,
}

impl BlockHeader {
    pub fn new(
        version: BlockVersion,
        previous_block_header_hash: HashType,
        merkle_root_hash: HashType,
        time: u32,
        n_bits: Compact256,
        nonce: u32,
        transaction_count: CompactSize,
    ) -> Self {
        BlockHeader {
            version,
            previous_block_header_hash,
            merkle_root_hash,
            time,
            n_bits,
            nonce,
            transaction_count,
        }
    }

    /// Generates the genesis block header
    pub fn generate_genesis_block_header() -> Self {
        BlockHeader::new(
            GENESIS_BLOCK_VERSION,
            GENESIS_PREVIOUS_BLOCK_HEADER_HASH,
            GENESIS_MERKLE_ROOT_HASH,
            GENESIS_TIME,
            Compact256::from(GENESIS_N_BITS),
            GENESIS_NONCE,
            CompactSize::new(GENESIS_TRANSACTION_COUNT),
        )
    }

    /// Calculates the header hash to verify the proof of work
    pub fn proof_of_work(&self) -> bool {
        let hash = match self.get_hash256d() {
            Ok(hash) => hash,
            Err(_) => return false,
        };

        let compact_hash = match Compact256::try_from(hash) {
            Ok(compact_hash) => compact_hash,
            Err(_) => return false,
        };

        self.n_bits > compact_hash
    }

    /// Verifies that the merkle root hash is correct
    pub fn proof_of_inclusion(&self, transactions: &[Transaction]) -> bool {
        let merkle_tree: MerkleTree = match MerkleTree::new(transactions) {
            Ok(merkle_tree) => merkle_tree,
            Err(error) => {
                println!("Error while creating merkle tree: {:?}", error);
                return false;
            }
        };

        match merkle_tree.get_root() {
            Ok(root) => {
                let resultado = root == self.merkle_root_hash;
                if !resultado {
                    println!("Roots are different");
                }
                resultado
            }
            Err(error) => {
                println!("Error while getting the root: {:?}", error);
                false
            }
        }
    }

    /// Get the hash 256 double of the block header
    ///
    /// ### Error
    ///  * `ErrorSerialization::ErrorInSerialization`: It will appear when there is an error in the serialization
    pub fn get_hash256d(&self) -> Result<HashType, ErrorSerialization> {
        let mut buffer = vec![];

        self.version.le_serialize(&mut buffer)?;
        self.previous_block_header_hash.le_serialize(&mut buffer)?;
        self.merkle_root_hash.be_serialize(&mut buffer)?;
        self.time.le_serialize(&mut buffer)?;
        self.n_bits.le_serialize(&mut buffer)?;
        self.nonce.le_serialize(&mut buffer)?;

        let buffer = {
            let mut temp: Vec<u8> = Vec::new();

            for byte in hash256d(&buffer)?.iter().rev() {
                temp.push(*byte);
            }

            temp
        };

        let buffer: HashType = match (*buffer.as_slice()).try_into() {
            Ok(buffer) => buffer,
            _ => {
                return Err(ErrorSerialization::ErrorInSerialization(
                    "Error while getting hash 256 d".to_string(),
                ))
            }
        };

        Ok(buffer)
    }
}

impl SerializableInternalOrder for BlockHeader {
    fn io_serialize(&self, stream: &mut dyn Write) -> Result<(), ErrorSerialization> {
        self.version.le_serialize(stream)?;
        self.previous_block_header_hash.le_serialize(stream)?;
        self.merkle_root_hash.be_serialize(stream)?;
        self.time.le_serialize(stream)?;
        self.n_bits.le_serialize(stream)?;
        self.nonce.le_serialize(stream)?;
        self.transaction_count.le_serialize(stream)?;

        Ok(())
    }
}

impl DeserializableInternalOrder for BlockHeader {
    fn io_deserialize(stream: &mut dyn Read) -> Result<Self, ErrorSerialization> {
        Ok(BlockHeader {
            version: BlockVersion::le_deserialize(stream)?,
            previous_block_header_hash: HashType::le_deserialize(stream)?,
            merkle_root_hash: HashType::be_deserialize(stream)?,
            time: u32::le_deserialize(stream)?,
            n_bits: Compact256::le_deserialize(stream)?,
            nonce: u32::le_deserialize(stream)?,
            transaction_count: CompactSize::le_deserialize(stream)?,
        })
    }
}
