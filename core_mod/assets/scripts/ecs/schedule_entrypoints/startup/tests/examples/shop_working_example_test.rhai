use shop::divisions::sex as SexShop;

private fn run_shop_working_example_test() {
    print("[Startup][Tests][Examples] shop working example test");

    SexShop::test_function();
    SexShop::SexShopProduct::test();

    let product = SexShop::SexShopProduct::new_("SuspiciouslyPenisShapedItem", 9.99);
    let name = product.name();
    let price_usd = product.price_usd();
    SexShop::SexShopProduct::verify_price(price_usd);

    print("[Startup][Tests][Examples] product check: " + name + " @ " + price_usd + "USD");
}
