use core_mod_api::usf::substrate as UsfSubstrate;

fn register_content_package() {
    UsfSubstrate::set_usf_content_package(
        "content_package.placeholder_gameplay.v1",
        true,
        "usf_content/content_packages/placeholder_gameplay/enabled",
    );
}
