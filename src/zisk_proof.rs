use anyhow::Result;
use proofman_util::VadcopFinalProof;
use serde::{Deserialize, Serialize};
use std::cell::Cell;

pub const ZISK_PUBLICS: usize = 64;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZiskPublics {
    data: Vec<u8>,
    ptr: Cell<usize>,
}

impl ZiskPublics {
    pub fn public_bytes(&self) -> Vec<u8> {
        let mut bytes = [0u8; ZISK_PUBLICS * 8];

        // Convert the 256 bytes back to ZISK_PUBLICS u64 values (padding upper 32 bits with zeros)
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

        bytes.to_vec()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZiskProgramVK {
    pub vk: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZiskProofWithPublicValues {
    pub proof: ZiskProof,
    pub publics: ZiskPublics,
    pub program_vk: ZiskProgramVK,
}

impl ZiskProofWithPublicValues {
    pub fn get_vadcop_final_proof(&self) -> Result<VadcopFinalProof> {
        match &self.proof {
            ZiskProof::VadcopFinal(proof_bytes) | ZiskProof::VadcopFinalCompressed(proof_bytes) => {
                let compressed = matches!(self.proof, ZiskProof::VadcopFinalCompressed(_));
                let mut pubs = self.program_vk.vk.clone();
                pubs.extend(self.publics.public_bytes());
                Ok(VadcopFinalProof::new(proof_bytes.clone(), pubs, compressed))
            }

            _ => Err(anyhow::anyhow!("Proof is not a Vadcop final proof")),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ZiskProof {
    Null(),
    VadcopFinal(Vec<u8>),
    VadcopFinalCompressed(Vec<u8>),
    Plonk(Vec<u8>),
    Fflonk(Vec<u8>),
}
