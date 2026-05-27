// WASM bindings for the ZK-STARK verifier using the ZisK library.
//
// Input format from zisk_common::Proof::get_proof_u64():
//   [minimal(1)][n_publics(1)][program_vk(4)][publics(64)][proof(...)][zisk_vk(4)]
//
// The vk_bytes parameter is ignored since zisk_vk is embedded in the proof.

use proofman_verifier::{verify_vadcop_final_compressed_u64, verify_vadcop_final_u64};
use wasm_bindgen::prelude::*;

const PROGRAM_VK_LEN: usize = 4;

#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub fn verify_stark(proof_bytes: &[u8], _vk_bytes: &[u8]) -> Result<bool, JsValue> {
    // Convert bytes to u64 array (little-endian)
    if proof_bytes.len() % 8 != 0 {
        return Err(JsValue::from_str("proof_bytes length must be a multiple of 8"));
    }

    let words: Vec<u64> = proof_bytes
        .chunks_exact(8)
        .map(|c| u64::from_le_bytes([c[0], c[1], c[2], c[3], c[4], c[5], c[6], c[7]]))
        .collect();

    // Parse format: [minimal(1)][n_publics(1)][program_vk][publics][proof][zisk_vk(4)]
    if words.len() < 2 + PROGRAM_VK_LEN {
        return Err(JsValue::from_str("proof too short"));
    }

    let minimal = words[0] != 0;
    let _n_publics = words[1] as usize;

    // zisk_vk is the last PROGRAM_VK_LEN u64s
    let zisk_vk_start = words.len() - PROGRAM_VK_LEN;
    let zisk_vk = &words[zisk_vk_start..];

    let blob = &words[1..zisk_vk_start];

    let ok = if minimal {
        verify_vadcop_final_compressed_u64(blob, zisk_vk)
    } else {
        verify_vadcop_final_u64(blob, zisk_vk)
    };

    Ok(ok)
}
