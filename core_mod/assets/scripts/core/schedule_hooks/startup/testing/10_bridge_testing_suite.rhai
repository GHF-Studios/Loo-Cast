private fn run_testing_bridge_smoke_suite() {
    print("[Startup][Testing] bridge smoke suite");

    shop::divisions::sex::test_function();
    shop::divisions::sex::SexShopProduct::test();

    let product = shop::divisions::sex::SexShopProduct::new_("SuspiciouslyPenisShapedItem", 9.99);
    let name = product.name();
    let price_usd = product.price_usd();
    shop::divisions::sex::SexShopProduct::verify_price(price_usd);

    print("[Startup][Testing] product check: " + name + " @ " + price_usd + "USD");
}
