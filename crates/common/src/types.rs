use alloy_primitives::hex;
use alloy_primitives::FixedBytes;
use serde::{Deserialize, Serialize};

//==============================================================================
// for int type, use uint type
// for string type, if formatted, use chunk[] to store field elements

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Eq, Hash)]
pub struct Uint256 {
    pub low: String,
    pub high: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Eq, Hash)]
pub struct HeaderProof {
    pub leaf_idx: u64,
    pub mmr_path: Vec<String>,
}

/// HeaderProofFormatted is the formatted version of HeaderProof
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Eq, Hash)]
pub struct HeaderProofFormatted {
    pub leaf_idx: u64,
    // mmr_path is encoded with poseidon
    pub mmr_path: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Eq, Hash)]
pub struct Header {
    pub rlp: String,
    pub proof: HeaderProof,
}

impl Header {
    pub fn to_cairo_format(&self) -> HeaderFormatted {
        let chunk_result = hex_to_8_byte_chunks_little_endian(&self.rlp);
        let proof = self.proof.clone();
        HeaderFormatted {
            rlp: chunk_result.chunks,
            rlp_bytes_len: chunk_result.chunks_len,
            proof: HeaderProofFormatted {
                leaf_idx: proof.leaf_idx,
                mmr_path: proof.mmr_path,
            },
        }
    }
}

/// HeaderFormatted is the formatted version of Header
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Eq, Hash)]
pub struct HeaderFormatted {
    pub rlp: Vec<String>,
    /// rlp_bytes_len is the byte( 8 bit ) length from rlp string
    pub rlp_bytes_len: u64,
    pub proof: HeaderProofFormatted,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Eq, Hash)]
pub struct Account {
    pub address: String,
    // U256 type
    pub account_key: String,
    pub proofs: Vec<MPTProof>,
}

impl Account {
    pub fn to_cairo_format(&self) -> AccountFormatted {
        let address_chunk_result = hex_to_8_byte_chunks_little_endian(&self.address);
        let account_key = split_hex_into_key_parts(&self.account_key);
        let proofs = self
            .proofs
            .iter()
            .map(|proof| {
                let proof_chunk_result: Vec<CairoFormattedChunkResult> = proof
                    .proof
                    .iter()
                    .map(|proof| hex_to_8_byte_chunks_little_endian(proof))
                    .collect();

                let proof_bytes_len = proof_chunk_result.iter().map(|x| x.chunks_len).collect();
                let proof_result: Vec<Vec<String>> = proof_chunk_result
                    .iter()
                    .map(|x| x.chunks.clone())
                    .collect();

                MPTProofFormatted {
                    block_number: proof.block_number,
                    proof_bytes_len,
                    proof: proof_result,
                }
            })
            .collect();
        AccountFormatted {
            address: address_chunk_result.chunks,
            account_key,
            proofs,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Eq, Hash)]
pub struct AccountFormatted {
    pub address: Vec<String>,
    pub account_key: Uint256,
    pub proofs: Vec<MPTProofFormatted>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Eq, Hash)]
pub struct MPTProof {
    pub block_number: u64,
    pub proof: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Eq, Hash)]
pub struct MPTProofFormatted {
    pub block_number: u64,
    /// proof_bytes_len is the byte( 8 bit ) length from each proof string
    pub proof_bytes_len: Vec<u64>,
    pub proof: Vec<Vec<String>>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Eq, Hash)]
pub struct MMRMeta {
    pub id: u64,
    pub root: String,
    pub size: u64,
    // hex encoded
    pub peaks: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Eq, Hash)]
pub struct Storage {
    pub address: String,
    // U256 type
    pub account_key: String,
    // U256 type
    pub storage_key: String,
    pub proofs: Vec<MPTProof>,
}

//TODO: not tested yet
impl Storage {
    pub fn to_cairo_format(&self) -> StorageFormatted {
        let address_chunk_result = hex_to_8_byte_chunks_little_endian(&self.address);
        let account_key = split_hex_into_key_parts(&self.account_key);
        let storage_key = split_hex_into_key_parts(&self.storage_key);
        let proofs = self
            .proofs
            .iter()
            .map(|proof| {
                let proof_chunk_result: Vec<CairoFormattedChunkResult> = proof
                    .proof
                    .iter()
                    .map(|proof| hex_to_8_byte_chunks_little_endian(proof))
                    .collect();

                let proof_bytes_len = proof_chunk_result.iter().map(|x| x.chunks_len).collect();
                let proof_result: Vec<Vec<String>> = proof_chunk_result
                    .iter()
                    .map(|x| x.chunks.clone())
                    .collect();

                MPTProofFormatted {
                    block_number: proof.block_number,
                    proof_bytes_len,
                    proof: proof_result,
                }
            })
            .collect();
        StorageFormatted {
            address: address_chunk_result.chunks,
            account_key,
            storage_key,
            proofs,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Eq, Hash)]
pub struct StorageFormatted {
    pub address: Vec<String>,
    pub account_key: Uint256,
    // storage key == storage slot
    pub storage_key: Uint256,
    pub proofs: Vec<MPTProofFormatted>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Eq, Hash)]
pub struct Task {
    pub computational_task: String,
    pub task_commitment: String,
    pub result: String,
    pub task_proof: Vec<FixedBytes<32>>,
    pub result_proof: Vec<FixedBytes<32>>,
    pub datalake: String,
    // ex. dynamic datalake / block sampled datalake
    pub datalake_type: u8,
    // ex. "header", "account", "storage"
    pub property_type: u8,
}

impl Task {
    pub fn to_cairo_format(&self) -> TaskFormatted {
        let computational_task_chunk_result =
            hex_to_8_byte_chunks_little_endian(&self.computational_task);
        let datalake_chunk_result = hex_to_8_byte_chunks_little_endian(&self.datalake);
        TaskFormatted {
            computational_bytes_len: computational_task_chunk_result.chunks_len,
            computational_task: computational_task_chunk_result.chunks,
            datalake_bytes_len: datalake_chunk_result.chunks_len,
            datalake: datalake_chunk_result.chunks,
            datalake_type: self.datalake_type,
            property_type: self.property_type,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TaskFormatted {
    pub computational_bytes_len: u64,
    pub computational_task: Vec<String>,
    pub datalake_bytes_len: u64,
    pub datalake: Vec<String>,
    pub datalake_type: u8,
    pub property_type: u8,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProcessedResult {
    // U256 type
    pub results_root: String,
    // U256 type
    pub tasks_root: String,
    pub headers: Vec<Header>,
    pub mmr: MMRMeta,
    pub accounts: Vec<Account>,
    pub storages: Vec<Storage>,
    pub tasks: Vec<Task>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProcessedResultFormatted {
    pub results_root: Uint256,
    pub tasks_root: Uint256,
    pub headers: Vec<HeaderFormatted>,
    pub mmr: MMRMeta,
    pub accounts: Vec<AccountFormatted>,
    pub storages: Vec<StorageFormatted>,
    pub tasks: Vec<TaskFormatted>,
}

pub fn bytes_to_8_bytes_chunks_little(input_bytes: &[u8]) -> Vec<u64> {
    input_bytes
        .chunks(8)
        .map(|chunk| {
            let mut arr = [0u8; 8];
            for (i, &byte) in chunk.iter().enumerate() {
                arr[i] = byte;
            }
            u64::from_le_bytes(arr)
        })
        .collect()
}

pub struct CairoFormattedChunkResult {
    pub chunks: Vec<String>,
    pub chunks_len: u64,
}

pub fn hex_to_8_byte_chunks_little_endian(input_hex: &str) -> CairoFormattedChunkResult {
    // Convert hex string to bytes
    let bytes = hex::decode(input_hex).expect("Invalid hex input");
    let chunks_len = bytes.len() as u64;
    // Process bytes into 8-byte chunks and convert to little-endian u64, then to hex strings
    let chunks = bytes
        .chunks(8)
        .map(|chunk| {
            let mut arr = [0u8; 8];
            let len = chunk.len();
            arr[..len].copy_from_slice(chunk);
            let le_int = u64::from_le_bytes(arr);
            format!("0x{:x}", le_int)
        })
        .collect();

    CairoFormattedChunkResult { chunks, chunks_len }
}

pub fn split_hex_into_key_parts(hex_str: &str) -> Uint256 {
    // Ensure the input is a hexadecimal string without the '0x' prefix.
    let clean_hex = hex_str.trim_start_matches("0x");

    // Pad the hexadecimal string to ensure it has 64 characters (256 bits).
    let padded_hex = format!("{:0>64}", clean_hex);

    // Split the padded string into "high" and "low" parts.
    let (high_part, low_part) = padded_hex.split_at(32); // Split at the 128-bit (32 hex char) mark.

    // Convert these parts into strings with the '0x' prefix.
    Uint256 {
        high: format!("0x{}", high_part),
        low: format!("0x{}", low_part),
    }
}
