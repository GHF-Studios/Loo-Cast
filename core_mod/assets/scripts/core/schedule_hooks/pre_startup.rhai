fn run_reflection_smoke_suite() {
    print("[PreStartup] reflection/bridge smoke suite");
    print("[PreStartup] " + ecs::bridge_info());

    shop::divisions::sex::test_function();
    shop::divisions::sex::SexShopProduct::test();

    let product = shop::divisions::sex::SexShopProduct::new_("SuspiciouslyPenisShapedItem", 9.99);
    let name = product.name();
    let price_usd = product.price_usd();
    shop::divisions::sex::SexShopProduct::verify_price(price_usd);
    print("[PreStartup] product check: " + name + " @ " + price_usd + "USD");
}

fn main(world) {
    run_reflection_smoke_suite();
}
