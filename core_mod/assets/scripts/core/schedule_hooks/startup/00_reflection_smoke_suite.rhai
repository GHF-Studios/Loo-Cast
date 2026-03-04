fn run_reflection_smoke_suite() {
    print("[Startup][Reflection] smoke suite");
    print("[Startup][Reflection] " + ecs::bridge_info());

    shop::divisions::sex::test_function();
    shop::divisions::sex::SexShopProduct::test();

    let product = shop::divisions::sex::SexShopProduct::new_("SuspiciouslyPenisShapedItem", 9.99);
    let name = product.name();
    let price_usd = product.price_usd();
    shop::divisions::sex::SexShopProduct::verify_price(price_usd);

    print("[Startup][Reflection] product check: " + name + " @ " + price_usd + "USD");
}
