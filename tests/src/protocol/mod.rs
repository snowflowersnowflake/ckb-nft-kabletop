
mod kabletop;
use molecule::prelude::{Byte, Builder, Entity};

use kabletop::{Args, Round, Operations, Challenge};

fn uint8_t(v: u8) -> kabletop::Uint8T {
    kabletop::Uint8TBuilder::default().set([Byte::from(v); 1]).build()
}

fn uint64_t(v: u64) -> kabletop::Uint64T {
    let mut mol_bytes: [Byte; 8] = [Byte::default(); 8];
    let bytes = v.to_le_bytes();
    for i in 0..8 {
        mol_bytes[i] = Byte::from(bytes[i]);
    }
    kabletop::Uint64TBuilder::default().set(mol_bytes).build()
}

fn blake160_t(v: [u8; 20]) -> kabletop::Blake160 {
    let mut mol_bytes: [Byte; 20] = [Byte::default(); 20];
    for i in 0..20 {
        mol_bytes[i] = Byte::from(v[i]);
    }
    kabletop::Blake160Builder::default().set(mol_bytes).build()
}

fn blake256_t(v: [u8; 32]) -> kabletop::Blake256 {
    let mut mol_bytes: [Byte; 32] = [Byte::default(); 32];
    for i in 0..32 {
        mol_bytes[i] = Byte::from(v[i]);
    }
    kabletop::Blake256Builder::default().set(mol_bytes).build()
}

fn witness_t(v: [u8; 65]) -> kabletop::Witness {
    let mut mol_bytes: [Byte; 65] = [Byte::default(); 65];
    for i in 0..65 {
        mol_bytes[i] = Byte::from(v[i]);
    }
    kabletop::WitnessBuilder::default().set(mol_bytes).build()
}

fn nfts_t(v: Vec<[u8; 20]>) -> kabletop::Nfts {
    let blake160s = v
        .into_iter()
        .map(|blake160| blake160_t(blake160))
        .collect::<Vec<kabletop::Blake160>>();
    kabletop::NftsBuilder::default().set(blake160s).build()
}

fn bytes_t(v: &[u8]) -> kabletop::Bytes {
    let bytes = v
        .to_vec()
        .iter()
        .map(|byte| Byte::new(byte.clone()))
        .collect::<Vec<Byte>>();
    kabletop::Bytes::new_builder()
        .set(bytes)
        .build()
}

#[allow(dead_code)]
pub fn to_vec<T: Entity>(t: &T) -> Vec<u8> {
    t.as_bytes().to_vec()
}

#[allow(dead_code)]
pub fn lock_args(raw: (u64, u8, u64, [u8; 32], [u8; 20], Vec<[u8; 20]>, [u8; 20], Vec<[u8; 20]>)) -> Args {
    Args::new_builder()
        .user_staking_ckb(uint64_t(raw.0))
        .user_deck_size(uint8_t(raw.1))
        .begin_blocknumber(uint64_t(raw.2))
        .lock_code_hash(blake256_t(raw.3))
        .user1_pkhash(blake160_t(raw.4))
        .user1_nfts(nfts_t(raw.5))
        .user2_pkhash(blake160_t(raw.6))
        .user2_nfts(nfts_t(raw.7))
        .build()
}

#[allow(dead_code)]
pub fn round(user_type: u8, operations: Vec<&[u8]>) -> Round {
    let operations = operations
        .iter()
        .map(|bytes| bytes_t(bytes))
        .collect::<Vec<kabletop::Bytes>>();
    let operations = Operations::new_builder()
        .set(operations)
        .build();
    Round::new_builder()
        .user_type(uint8_t(user_type))
        .operations(operations)
        .build()
}

#[allow(dead_code)]
pub fn challenge(offset: u8, signature: [u8; 65], data: Round) -> Challenge {
    Challenge::new_builder()
        .round_offset(uint8_t(offset))
        .signature(witness_t(signature))
        .round(data)
        .build()
}
