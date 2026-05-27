//! Scalar conversion facade re-exported from `base_mod_shared`.
//!
//! This keeps math-domain call sites stable while sharing one canonical
//! implementation with tools and other crates.

pub use base_mod_shared::utils::scalar_words::{DecimalWordNameError, decimal_string_to_snake_case, decimal_string_to_word_tokens};
