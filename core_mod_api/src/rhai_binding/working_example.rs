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

#[cfg(test)]
mod tests {
    use core_mod_macros::reflect_type;

    use crate::rhai_binding::{
        internals::statics::RUNTIME_BINDING_GRAPH,
        value_semantics::modes::{consts, HasTypeValueSemanticsConst, TypeValueSemantics},
    };

    // Sketch-only types to validate metadata + macro-generated semantics bounds.
    pub mod semantics_testbed {
        use super::reflect_type;

        #[reflect_type(semantics_testbed::CloneType, value_semantics = clone)]
        #[derive(Clone)]
        pub struct CloneType;

        #[reflect_type(semantics_testbed::OwnedType, value_semantics = owned)]
        #[derive(Clone)]
        pub struct OwnedType;

        #[reflect_type(semantics_testbed::RefType, value_semantics = ref)]
        #[derive(Clone)]
        pub struct RefType;

        #[reflect_type(semantics_testbed::MutType, value_semantics = mut)]
        #[derive(Clone)]
        pub struct MutType;

        #[reflect_type(semantics_testbed::ScopedOwnedType, value_semantics = scoped_owned)]
        #[derive(Clone)]
        pub struct ScopedOwnedType;

        #[reflect_type(semantics_testbed::ScopedRefType, value_semantics = scoped_ref)]
        #[derive(Clone)]
        pub struct ScopedRefType;

        #[reflect_type(semantics_testbed::ScopedMutType, value_semantics = scoped_mut)]
        #[derive(Clone)]
        pub struct ScopedMutType;
    }

    fn assert_semantics_const<const SEM: u8, T: HasTypeValueSemanticsConst<SEM>>() {}

    fn semantics_of(path: &'static str) -> TypeValueSemantics {
        RUNTIME_BINDING_GRAPH()
            .types
            .get(&path.into())
            .unwrap_or_else(|| panic!("Missing type metadata for '{path}'"))
            .value_semantics
            .get()
    }

    #[test]
    fn reflect_type_generates_const_semantics_bounds() {
        assert_semantics_const::<{ consts::CLONE }, semantics_testbed::CloneType>();
        assert_semantics_const::<{ consts::OWNED }, semantics_testbed::OwnedType>();
        assert_semantics_const::<{ consts::REF }, semantics_testbed::RefType>();
        assert_semantics_const::<{ consts::MUT }, semantics_testbed::MutType>();
        assert_semantics_const::<{ consts::SCOPED_OWNED }, semantics_testbed::ScopedOwnedType>();
        assert_semantics_const::<{ consts::SCOPED_REF }, semantics_testbed::ScopedRefType>();
        assert_semantics_const::<{ consts::SCOPED_MUT }, semantics_testbed::ScopedMutType>();
    }

    #[test]
    fn reflect_type_writes_semantics_into_runtime_metadata() {
        assert_eq!(semantics_of("semantics_testbed::CloneType"), TypeValueSemantics::Clone);
        assert_eq!(semantics_of("semantics_testbed::OwnedType"), TypeValueSemantics::Owned);
        assert_eq!(semantics_of("semantics_testbed::RefType"), TypeValueSemantics::Ref);
        assert_eq!(semantics_of("semantics_testbed::MutType"), TypeValueSemantics::Mut);
        assert_eq!(semantics_of("semantics_testbed::ScopedOwnedType"), TypeValueSemantics::ScopedOwned);
        assert_eq!(semantics_of("semantics_testbed::ScopedRefType"), TypeValueSemantics::ScopedRef);
        assert_eq!(semantics_of("semantics_testbed::ScopedMutType"), TypeValueSemantics::ScopedMut);
    }
}
