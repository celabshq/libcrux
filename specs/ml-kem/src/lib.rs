// This module is declared here since otherwise, hax reports the following error:
//
// The THIR body of item
// DefId(0:313 ~ hacspec_kyber[4591]::parameters::KyberFieldElement::{constant#0})
// was stolen.
//
// This is being tracked in https://github.com/hacspec/hacspec-v2/issues/27
pub mod parameters;

pub mod compress;
mod ind_cca;
pub mod ind_cpa;
pub mod invert_ntt;
pub mod matrix;
pub mod ntt;
pub mod polynomial;
pub mod sampling;
pub mod serialize;

pub use parameters::{
    MlKemParams, ML_KEM_1024, ML_KEM_1024_CT_SIZE, ML_KEM_1024_DK_PKE_SIZE, ML_KEM_1024_DK_SIZE,
    ML_KEM_1024_EK_SIZE, ML_KEM_1024_J_INPUT_SIZE, ML_KEM_1024_U_SIZE, ML_KEM_1024_V_SIZE,
    ML_KEM_512, ML_KEM_512_CT_SIZE, ML_KEM_512_DK_PKE_SIZE, ML_KEM_512_DK_SIZE, ML_KEM_512_EK_SIZE,
    ML_KEM_512_J_INPUT_SIZE, ML_KEM_512_U_SIZE, ML_KEM_512_V_SIZE, ML_KEM_768, ML_KEM_768_CT_SIZE,
    ML_KEM_768_DK_PKE_SIZE, ML_KEM_768_DK_SIZE, ML_KEM_768_EK_SIZE, ML_KEM_768_J_INPUT_SIZE,
    ML_KEM_768_U_SIZE, ML_KEM_768_V_SIZE,
};
pub use sampling::BadRejectionSamplingRandomnessError;

pub use ind_cca::{
    decapsulate, encapsulate, generate_keypair, ind_cca_unpack_decapsulate,
    ind_cca_unpack_encapsulate, ind_cca_unpack_generate_keypair, public_key_modulus_check,
    IndCcaUnpackedKeyPair,
};
