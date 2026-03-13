// WASM bindings for the ZK-STARK verifier using the ZisK library
use proofman_verifier::verify_vadcop_final_compressed;
use wasm_bindgen::prelude::*;

mod zisk_proof;
pub use zisk_proof::*;

// Set up panic hook for better error messages
#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub fn verify_stark(proof_bytes: &[u8], vk_bytes: &[u8]) -> Result<bool, JsValue> {
    let proof: ZiskProofWithPublicValues = bincode::deserialize(proof_bytes)
        .map_err(|e| JsValue::from_str(&format!("Failed to deserialize proof: {}", e)))?;

    let vadcop_proof = proof
        .get_vadcop_final_proof()
        .map_err(|e| JsValue::from_str(&format!("Failed to get vadcop proof: {}", e)))?;

    let result = verify_vadcop_final_compressed(&vadcop_proof, vk_bytes);
    println!("Verification result: {:?}", result);
    Ok(result)
}
