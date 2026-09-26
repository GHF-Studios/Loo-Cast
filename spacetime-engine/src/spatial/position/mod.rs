//! Engine-facing bridge to the pure USF canonical spatial algebra.
//!
//! `spacetime_engine_usf` owns the Scale Stack number itself. Canonical chunk
//! topology intentionally remains here until #30 extracts that separate concern.

mod chunk_address;

pub use chunk_address::UsfChunkAddress;
pub use spacetime_engine_usf::*;
