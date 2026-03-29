use core_mod_api::usf::substrate as UsfSubstrate;

fn register_dpt_categorizer() {
    UsfSubstrate::add_dpt_categorizer("dpt_categorizer.debug.zlm_lookup.v1");
}
