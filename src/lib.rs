// WASM bindings for the ZK-STARK verifier using the ZisK library.
//
// The Proof / ProofBody / PublicValues / ProgramVK types below mirror the
// bincode layout of `zisk_common` (v0.18+) so we can deserialize proofs
// produced by zisk without pulling in zisk-common (which depends on
// tokio/quinn and does not build for wasm32-unknown-unknown).
use std::sync::atomic::AtomicUsize;

use proofman_verifier::{verify_vadcop_final_compressed_u64, verify_vadcop_final_u64};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

const ZISK_PUBLICS: usize = 64;

#[derive(Default, Debug, Serialize, Deserialize)]
struct ProgramVK {
    vk: Vec<u64>,
}

#[derive(Default, Debug, Serialize, Deserialize)]
struct PublicValues {
    data: Vec<u8>,
    #[serde(skip)]
    #[allow(dead_code)]
    ptr: AtomicUsize,
}

impl PublicValues {
    fn public_u64(&self) -> Vec<u64> {
        (0..ZISK_PUBLICS)
            .map(|i| {
                let start = i * 4;
                u32::from_le_bytes([
                    self.data[start],
                    self.data[start + 1],
                    self.data[start + 2],
                    self.data[start + 3],
                ]) as u64
            })
            .collect()
    }
}

// PlonkVkey / PlonkVkBlob exist only so bincode can consume the right number
// of bytes when a Plonk proof is encountered; their contents are never read.
#[derive(Debug, Serialize, Deserialize)]
struct PlonkVkey {
    protocol: String,
    curve: String,
    n_public: u32,
    power: u32,
    k1: String,
    k2: String,
    qm: [String; 3],
    ql: [String; 3],
    qr: [String; 3],
    qo: [String; 3],
    qc: [String; 3],
    s1: [String; 3],
    s2: [String; 3],
    s3: [String; 3],
    x_2: [[String; 2]; 3],
    w: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct PlonkVkBlob {
    vadcop_vk: Vec<u64>,
    plonk_vkey: PlonkVkey,
}

#[derive(Debug, Serialize, Deserialize)]
enum ProofBody {
    Vadcop { proof: Vec<u64>, zisk_vk: Vec<u64>, minimal: bool },
    Plonk { proof_bytes: Vec<u8>, plonk_vk: Box<PlonkVkBlob> },
}

impl Default for ProofBody {
    fn default() -> Self {
        ProofBody::Vadcop { proof: Vec::new(), zisk_vk: Vec::new(), minimal: false }
    }
}

#[derive(Default, Debug, Serialize, Deserialize)]
struct Proof {
    body: ProofBody,
    publics: PublicValues,
    program_vk: ProgramVK,
}

#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub fn verify_stark(proof_bytes: &[u8], vk_bytes: &[u8]) -> Result<bool, JsValue> {
    let (proof, _): (Proof, usize) =
        bincode::serde::decode_from_slice(proof_bytes, bincode::config::standard())
            .map_err(|e| JsValue::from_str(&format!("failed to deserialize proof: {e}")))?;

    let (vadcop_proof, minimal) = match &proof.body {
        ProofBody::Vadcop { proof, minimal, .. } => (proof.as_slice(), *minimal),
        ProofBody::Plonk { .. } => {
            return Err(JsValue::from_str("Plonk proofs are not supported in the wasm verifier"));
        }
    };

    // Layout expected by verify_*_u64:
    //   [n_publics: u64][program_vk: u64...][publics: u64...][proof: u64...]
    let publics = proof.publics.public_u64();
    let n_publics = (proof.program_vk.vk.len() + publics.len()) as u64;

    let mut blob =
        Vec::with_capacity(1 + proof.program_vk.vk.len() + publics.len() + vadcop_proof.len());
    blob.push(n_publics);
    blob.extend_from_slice(&proof.program_vk.vk);
    blob.extend_from_slice(&publics);
    blob.extend_from_slice(vadcop_proof);

    let vk = bytes_to_u64_le(vk_bytes);

    let ok = if minimal {
        verify_vadcop_final_compressed_u64(&blob, &vk)
    } else {
        verify_vadcop_final_u64(&blob, &vk)
    };

    Ok(ok)
}

fn bytes_to_u64_le(bytes: &[u8]) -> Vec<u64> {
    bytes
        .chunks_exact(8)
        .map(|c| u64::from_le_bytes([c[0], c[1], c[2], c[3], c[4], c[5], c[6], c[7]]))
        .collect()
}
