use core_mod_api::usf::phenomenon as UsfPhenomenon;

fn register_phenomenon_model() {
    UsfPhenomenon::add_phenomenon_model("phenomenon_model.debug.mandelbulb.primary", "phenomenon.debug.mandelbulb");
    UsfPhenomenon::add_phenomenon_model("phenomenon_model.debug.sierpinski_sponge.primary", "phenomenon.debug.sierpinski_sponge");

    UsfPhenomenon::set_primary_model("phenomenon.debug.mandelbulb", "phenomenon_model.debug.mandelbulb.primary");
    UsfPhenomenon::set_primary_model("phenomenon.debug.sierpinski_sponge", "phenomenon_model.debug.sierpinski_sponge.primary");
}
