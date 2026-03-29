use core_mod_api::usf::substrate as UsfSubstrate;

fn register_dpt_sampler() {
    UsfSubstrate::add_dpt_sampler("dpt_sampler.debug.default.v1");
}
