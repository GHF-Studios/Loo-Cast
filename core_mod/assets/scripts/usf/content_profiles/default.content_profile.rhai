use core_mod_api::usf::substrate as UsfSubstrate;

fn register_usf_content_profile() {
    UsfSubstrate::set_usf_content_profile(
        "content_profile.placeholder_gameplay.v1",
        "content_package.placeholder_gameplay.v1",
    );
}
