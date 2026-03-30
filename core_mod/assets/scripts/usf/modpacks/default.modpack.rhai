use core_mod_api::usf::substrate as UsfSubstrate;

fn register_modpack() {
    UsfSubstrate::set_usf_modpack(
        "modpack.placeholder_gameplay.v1",
        "mod.placeholder_gameplay.v1",
    );
}
