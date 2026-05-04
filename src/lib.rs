// WASM bindings for the ZK-STARK verifier using the ZisK library.
//
// The Proof / PublicValues / ProgramVK / ProofKind types below mirror the
// bincode layout of `zisk_common` v0.17.0 so we can deserialize proofs
// produced by zisk without pulling in zisk-common (which depends on
// tokio/quinn and does not build for wasm32-unknown-unknown).
use std::sync::atomic::AtomicUsize;

use proofman_verifier::{verify_vadcop_final_bytes, verify_vadcop_final_compressed_bytes};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

const ZISK_PUBLICS: usize = 64;

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
enum ProofKind {
    #[default]
    VadcopFinal,
    VadcopFinalMinimal,
    Plonk,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
struct ProgramVK {
    pub vk: Vec<u8>,
}

#[derive(Default, Debug, Serialize, Deserialize)]
struct PublicValues {
    data: Vec<u8>,
    #[serde(skip)]
    _ptr: AtomicUsize,
}

impl PublicValues {
    fn public_bytes(&self) -> Vec<u8> {
        let mut bytes = vec![0u8; ZISK_PUBLICS * 8];
        for i in 0..ZISK_PUBLICS {
            let start = i * 4;
            let val32 = u32::from_le_bytes([
                self.data[start],
                self.data[start + 1],
                self.data[start + 2],
                self.data[start + 3],
            ]);
            let val64 = val32 as u64;
            bytes[i * 8..(i + 1) * 8].copy_from_slice(&val64.to_le_bytes());
        }
        bytes
    }
}

#[derive(Default, Debug, Serialize, Deserialize)]
struct Proof {
    proof_kind: ProofKind,
    proof_bytes: Vec<u8>,
    publics: PublicValues,
    program_vk: ProgramVK,
    zisk_vk: Vec<u8>,
}

#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub fn verify_stark(proof_bytes: &[u8], vk_bytes: &[u8]) -> Result<bool, JsValue> {
    let proof: Proof = bincode::deserialize(proof_bytes)
        .map_err(|e| JsValue::from_str(&format!("failed to deserialize proof: {e}")))?;

    // Public input layout expected by proofman-verifier::verify_*_bytes:
    //   [n_publics_u64_count: 8 bytes LE][publics][proof_bytes]
    let mut pubs = proof.program_vk.vk.clone();
    pubs.extend(proof.publics.public_bytes());

    let mut blob = Vec::with_capacity(8 + pubs.len() + proof.proof_bytes.len());
    blob.extend_from_slice(&((pubs.len() / 8) as u64).to_le_bytes());
    blob.extend_from_slice(&pubs);
    blob.extend_from_slice(&proof.proof_bytes);

    let ok = match proof.proof_kind {
        ProofKind::VadcopFinal => verify_vadcop_final_bytes(&blob, vk_bytes),
        ProofKind::VadcopFinalMinimal => verify_vadcop_final_compressed_bytes(&blob, vk_bytes),
        ProofKind::Plonk => {
            return Err(JsValue::from_str("Plonk proofs are not supported in the wasm verifier"));
        }
    };

    Ok(ok)
}
