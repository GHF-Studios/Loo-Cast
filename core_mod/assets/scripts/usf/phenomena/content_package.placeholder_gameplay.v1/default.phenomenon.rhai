use core_mod_api::usf::phenomenon as UsfPhenomenon;

fn register_phenomenon() {
    UsfPhenomenon::add_phenomenon("phenomenon.debug.mandelbulb", "mandelbulb");
    UsfPhenomenon::add_phenomenon("phenomenon.debug.sierpinski_sponge", "sierpinski_sponge");
}
