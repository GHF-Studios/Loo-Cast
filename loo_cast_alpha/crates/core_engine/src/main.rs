//! Alpha bootstrap stub for legacy `core_engine`.
//!
//! The legacy engine entrypoint composes Bevy plugins and runtime module graphs.
//! In alpha bootstrap we intentionally keep only a top-level static-link smoke path.

fn main() {
    let linked_crates = [core_mod::crate_identity(), base_mod::crate_identity()];
    println!("Loo Cast alpha core_engine stub; statically linked crates: {}", linked_crates.join(", "));
}
