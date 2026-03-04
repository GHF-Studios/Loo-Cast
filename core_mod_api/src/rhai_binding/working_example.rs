pub mod shop {
    use core_mod_macros::{
        reflect_constructor_function, reflect_inherent_impl, reflect_item_associated_function,
        reflect_method_function, reflect_module_associated_function, reflect_sub_module,
        reflect_top_level_module, reflect_trait, reflect_trait_impl, reflect_type,
    };

    reflect_top_level_module!(
        id = shop,
        sub_modules = [divisions],
        traits = [],
        types = [],
        module_associated_functions = [],
    );

    pub mod divisions {
        use core_mod_macros::{reflect_sub_module, reflect_trait, reflect_trait_impl, reflect_type, reflect_inherent_impl, reflect_module_associated_function, reflect_item_associated_function, reflect_constructor_function, reflect_method_function};

        reflect_sub_module!(
            id = shop::divisions,
            sub_modules = [sex],
            traits = [],
            types = [],
            module_associated_functions = [],
        );

        pub mod sex {
            use core_mod_macros::{
                reflect_constructor_function, reflect_inherent_impl, reflect_item_associated_function,
                reflect_method_function, reflect_module_associated_function, reflect_sub_module,
                reflect_trait, reflect_trait_impl, reflect_type,
            };

            reflect_sub_module!(
                id = shop::divisions::sex,
                sub_modules = [],
                traits = [SexShopTest],
                types = [SexShopProduct],
                module_associated_functions = [test_function],
            );

            #[reflect_trait(shop::divisions::sex::SexShopTest)]
            pub trait SexShopTest {
                fn test();
            }

            #[reflect_type(shop::divisions::sex::SexShopProduct)]
            #[derive(Clone)]
            pub struct SexShopProduct {
                name: &'static str,
                price_usd: f64,
            }

            #[reflect_inherent_impl(shop::divisions::sex::SexShopProduct)]
            impl SexShopProduct {
                #[reflect_constructor_function(shop::divisions::sex::SexShopProduct)]
                pub fn new_(name: &'static str, price_usd: f64) -> Self {
                    Self { name, price_usd }
                }

                #[reflect_method_function(shop::divisions::sex::SexShopProduct)]
                pub fn name(&mut self) -> &'static str {
                    self.name
                }

                #[reflect_method_function(shop::divisions::sex::SexShopProduct)]
                pub fn price_usd(&mut self) -> f64 {
                    self.price_usd
                }

                #[reflect_item_associated_function(shop::divisions::sex::SexShopProduct)]
                pub fn verify_price(price_usd: f64) {
                    if price_usd < 0.0 {
                        panic!("Price '{}USD' could not be verified", price_usd)
                    }
                }
            }

            #[reflect_trait_impl(<shop::divisions::sex::SexShopProduct as shop::divisions::sex::SexShopTest>)]
            impl SexShopTest for SexShopProduct {
                #[reflect_item_associated_function(<shop::divisions::sex::SexShopProduct as shop::divisions::sex::SexShopTest>)]
                fn test() {
                    println!("Big paling sound!")
                }
            }

            #[reflect_module_associated_function(shop::divisions::sex::test_function)]
            pub fn test_function() {
                println!("Small vieze asbak sound!")
            }
        }
    }
}
