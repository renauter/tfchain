use parity_scale_codec::{Decode, Encode};
use scale_info::TypeInfo;
use sp_std::vec::Vec;

/// Utility type for managing upgrades/migrations.
#[derive(Encode, Decode, Clone, Debug, PartialEq, PartialOrd, TypeInfo)]
pub enum StorageVersion {
    V1,
    V2, // add source to both BurnTransaction type and ExpiredBurnTransaction event
}

impl Default for StorageVersion {
    fn default() -> StorageVersion {
        StorageVersion::V1
    }
}

// MintTransaction contains all the information about
// Stellar -> TF Chain minting transaction.
// if the votes field is larger then (number of validators / 2) + 1 , the transaction will be minted
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Encode, Decode, Default, Debug, TypeInfo)]
pub struct MintTransaction<AccountId, BlockNumber> {
    pub amount: u64,
    pub target: AccountId,
    pub block: BlockNumber,
    pub votes: u32,
}

// BurnTransaction contains all the information about
// TF Chain -> Stellar burn transaction
// Transaction is ready when (number of validators / 2) + 1 signatures are present
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Encode, Decode, Default, Debug, TypeInfo)]
pub struct BurnTransaction<AccountId, BlockNumber> {
    pub block: BlockNumber,
    pub amount: u64,
    pub source: Option<AccountId>,
    pub target: Vec<u8>,
    pub signatures: Vec<StellarSignature>,
    pub sequence_number: u64,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Encode, Decode, Default, Debug, TypeInfo)]
pub struct RefundTransaction<BlockNumber> {
    pub block: BlockNumber,
    pub amount: u64,
    pub target: Vec<u8>,
    pub tx_hash: Vec<u8>,
    pub signatures: Vec<StellarSignature>,
    pub sequence_number: u64,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Encode, Decode, Default, Debug, TypeInfo)]
pub struct StellarSignature {
    pub signature: Vec<u8>,
    pub stellar_pub_key: Vec<u8>,
}
