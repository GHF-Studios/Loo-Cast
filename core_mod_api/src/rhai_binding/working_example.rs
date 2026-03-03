// The vision: 

// The actual end-user code
// <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<< //
pub mod shop {
    use core_mod_macros::reflect_top_level_module;

    use crate::{
        rhai_binding::{
            meta::{
                monomorphized::{
                   module::*,
                },
                generic::{
                    abstract_primitive::ConstDynMetadata,
                    module::*,
                },
                registry::*,
            },
            path::{
                module_path::*,
                trait_path::*,
                type_path::*,
                impl_path::*,
                function_path::*,
            }
        },
        utils::{clone_closure::CloneClosure, clone_lazy::CloneLazy}
    };

    reflect_top_level_module!(
        id = shop,
        sub_modules = [divisions],
        traits = [],
        types = [],
        module_associated_functions = [],
    );
// >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>> //

// Expanded MetaProgramming Magic (It's just a bunch of metadata)
// <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<< //
    #[allow(non_upper_case_globals)]
    static __SHOP__TOP_LEVEL_MODULE__: CloneLazy<TopLevelModuleMetadata> = CloneLazy::new(CloneClosure::new((), |(), ()| __Shop__TopLevelModule__.from_comptime_to_runtime(&__Shop__TopLevelModule__)));
    inventory::submit!(TopLevelModuleMetadataEntry(&__SHOP__TOP_LEVEL_MODULE__));

    #[allow(non_camel_case_types)]
    #[derive(Clone, PartialEq, Eq, Hash)]
    pub struct __Shop__TopLevelModule__;
    impl ConstDynMetadata for __Shop__TopLevelModule__ {
        fn raw_rust_module_path(&self) -> &'static str { module_path!() }
    }
    impl NativeModuleConstDynMetadata for __Shop__TopLevelModule__ {
        fn traits(&self) -> CloneLazy<Vec<TraitPath>> {
            CloneLazy::new(CloneClosure::new((), |_, _| vec![]))
        }
        fn types(&self) -> CloneLazy<Vec<TypePath>> {
            CloneLazy::new(CloneClosure::new((), |_, _| vec![]))
        }
        fn inherent_impls(&self) -> CloneLazy<Vec<InherentImplPath>> {
            CloneLazy::new(CloneClosure::new((), |_, _| vec![]))
        }
        fn trait_impls(&self) -> CloneLazy<Vec<TraitImplPath>> {
            CloneLazy::new(CloneClosure::new((), |_, _| vec![]))
        }
    }
    impl TopLevelModuleConstDynMetadata for __Shop__TopLevelModule__ {
        fn id_path(&self) -> CloneLazy<TopLevelModulePath> {
            CloneLazy::new(CloneClosure::new((), |_, _| "shop".into()))
        }
        fn sub_modules(&self) -> CloneLazy<Vec<SubModulePath>> {
            CloneLazy::new(CloneClosure::new((), |_, _| vec!["shop::divisions".into()]))
        }
        fn type_binding_modules(&self) -> CloneLazy<Vec<TypeBindingModulePath>> {
            CloneLazy::new(CloneClosure::new((), |_, _| vec![]))
        }
        fn module_associated_functions(&self) -> CloneLazy<Vec<ModuleAssociatedFunctionPath>> {
            CloneLazy::new(CloneClosure::new((), |_, _| vec![]))
        }
    }
    impl TopLevelModuleDynamicTypedMetadata for __Shop__TopLevelModule__ {}
// >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>> //

// The actual end-user code
// <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<< //
    pub mod divisions {
        use core_mod_macros::reflect_sub_module;

        use crate::{
            rhai_binding::{
                meta::{
                    monomorphized::{
                       module::*,
                    },
                    generic::{
                        abstract_primitive::ConstDynMetadata,
                        module::*,
                    },
                    registry::*,
                },
                path::{
                    module_path::*,
                    trait_path::*,
                    type_path::*,
                    impl_path::*,
                    function_path::*,
                }
            },
            utils::{clone_closure::CloneClosure, clone_lazy::CloneLazy}
        };

        reflect_sub_module!(
            id = shop::divisions,
            sub_modules = [sex],
            traits = [],
            types = [],
            module_associated_functions = [],
        );
// >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>> //

// Expanded MetaProgramming Magic (It's just a bunch of metadata)
// <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<< //
        #[allow(non_upper_case_globals)]
        static __DIVISIONS__SUB_MODULE__: CloneLazy<SubModuleMetadata> = CloneLazy::new(CloneClosure::new((), |(), ()| __Divisions__SubModule__.from_comptime_to_runtime(&__Divisions__SubModule__)));
        inventory::submit!(SubModuleMetadataEntry(&__DIVISIONS__SUB_MODULE__));

        #[allow(non_camel_case_types)]
        #[derive(Clone, PartialEq, Eq, Hash)]
        pub struct __Divisions__SubModule__;
        impl ConstDynMetadata for __Divisions__SubModule__ {
            fn raw_rust_module_path(&self) -> &'static str { module_path!() }
        }
        impl NativeModuleConstDynMetadata for __Divisions__SubModule__ {
            fn traits(&self) -> CloneLazy<Vec<TraitPath>> {
                CloneLazy::new(CloneClosure::new((), |_, _| vec![]))
            }
            fn types(&self) -> CloneLazy<Vec<TypePath>> {
                CloneLazy::new(CloneClosure::new((), |_, _| vec![]))
            }
            fn inherent_impls(&self) -> CloneLazy<Vec<InherentImplPath>> {
                CloneLazy::new(CloneClosure::new((), |_, _| vec![]))
            }
            fn trait_impls(&self) -> CloneLazy<Vec<TraitImplPath>> {
                CloneLazy::new(CloneClosure::new((), |_, _| vec![]))
            }
        }
        impl SubModuleConstDynMetadata for __Divisions__SubModule__ {
            fn id_path(&self) -> CloneLazy<SubModulePath> {
                CloneLazy::new(CloneClosure::new((), |_, _| "shop::divisions".into()))
            }
            fn sub_modules(&self) -> CloneLazy<Vec<SubModulePath>> {
                CloneLazy::new(CloneClosure::new((), |_, _| vec!["shop::divisions::sex".into()]))
            }
            fn type_binding_modules(&self) -> CloneLazy<Vec<TypeBindingModulePath>> {
                CloneLazy::new(CloneClosure::new((), |_, _| vec![]))
            }
            fn module_associated_functions(&self) -> CloneLazy<Vec<ModuleAssociatedFunctionPath>> {
                CloneLazy::new(CloneClosure::new((), |_, _| vec![]))
            }
        }
        impl SubModuleDynamicTypedMetadata for __Divisions__SubModule__ {}
// >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>> //

// The actual end-user code
// <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<< //
        pub mod sex {
            use core_mod_macros::{
                reflect_sub_module,
                reflect_trait,
                reflect_trait_impl,
                reflect_type,
                reflect_inherent_impl,
                reflect_module_associated_function,
                reflect_item_associated_function,
                reflect_constructor_function,
                reflect_method_function
            };
            use rhai::ImmutableString;

            use crate::{
                reflection::{
                    internals::traits::*,
                    traits::StaticTraitObject,
                },
                rhai_binding::{
                    meta::{
                        generic::{
                            abstract_primitive::ConstDynMetadata, function::*, impl_::*, module::*, trait_::*, type_::*
                        }, monomorphized::{
                           function::*, impl_::*, module::*, trait_::*, type_::*
                        }, registry::*
                    },
                    path::{
                        function_path::*,
                        impl_path::*,
                        module_path::*,
                        trait_path::*,
                        type_path::*
                    }
                },
                utils::{clone_closure::CloneClosure, clone_lazy::CloneLazy}
            };

            reflect_sub_module!(
                id_path = shop::divisions::sex,
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
                pub fn new_(name: &'static str, price_usd: f64) -> Self { Self { name, price_usd } }

                // TODO: MAYBE FIX: &mut instead of & because rhai only recognizes methods as such if the first arg is &mut SomeType
                #[reflect_method_function(shop::divisions::sex::SexShopProduct)]
                pub fn name(&mut self) -> &'static str { self.name }

                // TODO: MAYBE FIX: &mut instead of & because rhai only recognizes methods as such if the first arg is &mut SomeType
                #[reflect_method_function(shop::divisions::sex::SexShopProduct)]
                pub fn price_usd(&mut self) -> f64 { self.price_usd }

                #[reflect_item_associated_function(shop::divisions::sex::SexShopProduct)]
                pub fn verify_price(price_usd: f64) {
                    if price_usd < 0.0 { panic!("Price '{}USD' could not be verified", price_usd) }
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
// >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>> //

// Expanded MetaProgramming Magic (It's just a bunch of metadata)
// <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<< //

            #[allow(non_upper_case_globals)]
            static __SEX__SUB_MODULE__: CloneLazy<SubModuleMetadata> = CloneLazy::new(CloneClosure::new((), |(), ()| __Sex__SubModule__.from_comptime_to_runtime(&__Sex__SubModule__)));
            inventory::submit!(SubModuleMetadataEntry(&__SEX__SUB_MODULE__));
            #[allow(non_upper_case_globals)]
            static __SEX_SHOP_TEST__TRAIT__: CloneLazy<TraitMetadata> = CloneLazy::new(CloneClosure::new((), |(), ()| __SexShopTest__Trait__.from_comptime_to_runtime(&__SexShopTest__Trait__)));
            inventory::submit!(TraitMetadataEntry(&__SEX_SHOP_TEST__TRAIT__));
            #[allow(non_upper_case_globals)]
            static __SEX_SHOP_TEST__TRAIT_OBJECT__: CloneLazy<TraitObjectMetadata> = CloneLazy::new(CloneClosure::new((), |(), ()| __SexShopTest__TraitObject__.from_comptime_to_runtime(&__SexShopTest__TraitObject__)));
            inventory::submit!(TraitObjectMetadataEntry(&__SEX_SHOP_TEST__TRAIT_OBJECT__));
            #[allow(non_upper_case_globals)]
            static __SEX_SHOP_PRODUCT__TYPE__: CloneLazy<TypeMetadata> = CloneLazy::new(CloneClosure::new((), |(), ()| __SexShopProduct__Type__.from_comptime_to_runtime(&__SexShopProduct__Type__)));
            inventory::submit!(TypeMetadataEntry(&__SEX_SHOP_PRODUCT__TYPE__));
            #[allow(non_upper_case_globals)]
            static __SEX_SHOP_PRODUCT__TYPE_PROXY_MODULE__: CloneLazy<TypeBindingModuleMetadata> = CloneLazy::new(CloneClosure::new((), |(), ()| __SexShopProduct__TypeBindingModule__.from_comptime_to_runtime(&__SexShopProduct__TypeBindingModule__)));
            inventory::submit!(TypeBindingModuleMetadataEntry(&__SEX_SHOP_PRODUCT__TYPE_PROXY_MODULE__));
            #[allow(non_upper_case_globals)]
            static __SEX_SHOP_PRODUCT__INHERENT_IMPL__: CloneLazy<InherentImplMetadata> = CloneLazy::new(CloneClosure::new((), |(), ()| __SexShopProduct__InherentImpl__.from_comptime_to_runtime(&__SexShopProduct__InherentImpl__)));
            inventory::submit!(InherentImplMetadataEntry(&__SEX_SHOP_PRODUCT__INHERENT_IMPL__));
            #[allow(non_upper_case_globals)]
            static __SEX_SHOP_PRODUCT__AS__SEX_SHOP_TEST__TRAIT_IMPL__: CloneLazy<TraitImplMetadata> = CloneLazy::new(CloneClosure::new((), |(), ()| __SexShopProduct__as__SexShopTest__TraitImpl__.from_comptime_to_runtime(&__SexShopProduct__as__SexShopTest__TraitImpl__)));
            inventory::submit!(TraitImplMetadataEntry(&__SEX_SHOP_PRODUCT__AS__SEX_SHOP_TEST__TRAIT_IMPL__));
            #[allow(non_upper_case_globals)]
            static __TEST_FUNCTION__MODULE_ASSOCIATED_FUNCTION__: CloneLazy<ModuleAssociatedFunctionMetadata> = CloneLazy::new(CloneClosure::new((), |(), ()| __TestFunction__ModuleAssociatedFunction__.from_comptime_to_runtime(&__TestFunction__ModuleAssociatedFunction__)));
            inventory::submit!(ModuleAssociatedFunctionMetadataEntry(&__TEST_FUNCTION__MODULE_ASSOCIATED_FUNCTION__));
            #[allow(non_upper_case_globals)]
            static __VERIFY_PRICE__ITEM_ASSOCIATED_FUNCTION__: CloneLazy<ItemAssociatedFunctionMetadata> = CloneLazy::new(CloneClosure::new((), |(), ()| __VerifyPrice__ItemAssociatedFunction__.from_comptime_to_runtime(&__VerifyPrice__ItemAssociatedFunction__)));
            inventory::submit!(ItemAssociatedFunctionMetadataEntry(&__VERIFY_PRICE__ITEM_ASSOCIATED_FUNCTION__));
            #[allow(non_upper_case_globals)]
            static __TEST__ITEM_ASSOCIATED_FUNCTION__: CloneLazy<ItemAssociatedFunctionMetadata> = CloneLazy::new(CloneClosure::new((), |(), ()| __Test__ItemAssociatedFunction__.from_comptime_to_runtime(&__Test__ItemAssociatedFunction__)));
            inventory::submit!(ItemAssociatedFunctionMetadataEntry(&__TEST__ITEM_ASSOCIATED_FUNCTION__));
            #[allow(non_upper_case_globals)]
            static __NEW__CONSTRUCTOR_FUNCTION__: CloneLazy<ConstructorFunctionMetadata> = CloneLazy::new(CloneClosure::new((), |(), ()| __New__ConstructorFunction__.from_comptime_to_runtime(&__New__ConstructorFunction__)));
            inventory::submit!(ConstructorFunctionMetadataEntry(&__NEW__CONSTRUCTOR_FUNCTION__));
            #[allow(non_upper_case_globals)]
            static __NAME__METHOD_FUNCTION__: CloneLazy<MethodFunctionMetadata> = CloneLazy::new(CloneClosure::new((), |(), ()| __Name__MethodFunction__.from_comptime_to_runtime(&__Name__MethodFunction__)));
            inventory::submit!(MethodFunctionMetadataEntry(&__NAME__METHOD_FUNCTION__));
            #[allow(non_upper_case_globals)]
            static __PRICE_USD__METHOD_FUNCTION__: CloneLazy<MethodFunctionMetadata> = CloneLazy::new(CloneClosure::new((), |(), ()| __PriceUsd__MethodFunction__.from_comptime_to_runtime(&__PriceUsd__MethodFunction__)));
            inventory::submit!(MethodFunctionMetadataEntry(&__PRICE_USD__METHOD_FUNCTION__));

            #[allow(non_camel_case_types)]
            #[derive(Clone, PartialEq, Eq, Hash)]
            pub struct __Sex__SubModule__;
            impl ConstDynMetadata for __Sex__SubModule__ {
                fn raw_rust_module_path(&self) -> &'static str { module_path!() }
            }
            impl NativeModuleConstDynMetadata for __Sex__SubModule__ {
                fn traits(&self) -> CloneLazy<Vec<TraitPath>> { CloneLazy::new(CloneClosure::new((), |_, _| vec!["shop::divisions::sex::SexShopTest".into()])) }
                fn types(&self) -> CloneLazy<Vec<TypePath>> { CloneLazy::new(CloneClosure::new((), |_, _| vec!["shop::divisions::sex::SexShopProduct".into()])) }
                fn inherent_impls(&self) -> CloneLazy<Vec<InherentImplPath>> { CloneLazy::new(CloneClosure::new((), |_, _| vec![])) }
                fn trait_impls(&self) -> CloneLazy<Vec<TraitImplPath>> { CloneLazy::new(CloneClosure::new((), |_, _| vec![])) }
            }
            impl SubModuleConstDynMetadata for __Sex__SubModule__ {
                fn id_path(&self) -> CloneLazy<SubModulePath> { CloneLazy::new(CloneClosure::new((), |_, _| "shop::divisions::sex".into())) }
                fn sub_modules(&self) -> CloneLazy<Vec<SubModulePath>> { CloneLazy::new(CloneClosure::new((), |_, _| vec![])) }
                fn type_binding_modules(&self) -> CloneLazy<Vec<TypeBindingModulePath>> { CloneLazy::new(CloneClosure::new((), |_, _| vec!["shop::divisions::sex::SexShopProduct".into()])) }
                fn module_associated_functions(&self) -> CloneLazy<Vec<ModuleAssociatedFunctionPath>> { CloneLazy::new(CloneClosure::new((), |_, _| vec!["shop::divisions::sex::test_function".into()])) }
            }
            impl SubModuleDynamicTypedMetadata for __Sex__SubModule__ {}

            #[allow(non_camel_case_types)]
            #[derive(Clone, PartialEq, Eq, Hash)]
            pub struct __SexShopTest__Trait__;
            impl ConstDynMetadata for __SexShopTest__Trait__ {
                fn raw_rust_module_path(&self) -> &'static str { module_path!() }
            }
            impl DynGetTraitName for __SexShopTest__Trait__ {
                fn trait_name(&self) -> &'static str { "SexShopTest" }
            }
            impl GetTraitId for __SexShopTest__Trait__ {
                const TRAIT_ID: &'static str = "shop::divisions::sex::SexShopTest";
            }
            impl TraitConstDynMetadata for __SexShopTest__Trait__ {
                fn id_path(&self) -> CloneLazy<TraitPath> { CloneLazy::new(CloneClosure::new((), |_, _| "shop::divisions::sex::SexShopTest".into())) }
            }
            impl TraitDynamicTypedMetadata for __SexShopTest__Trait__ {}

            // TODO: WIP! Implement properly; this is highly experimental!
            #[repr(transparent)]
            pub struct SexShopTestTraitObject(pub StaticTraitObject<__SexShopTest__Trait__>);

            #[allow(non_camel_case_types)]
            #[derive(Clone, PartialEq, Eq, Hash)]
            pub struct __SexShopTest__TraitObject__;
            impl ConstDynMetadata for __SexShopTest__TraitObject__ {
                fn raw_rust_module_path(&self) -> &'static str { module_path!() }
            }
            impl DynGetTraitObjectName for __SexShopTest__TraitObject__ {
                fn trait_object_name(&self) -> &'static str { "SexShopTestTraitObject" }
            }
            impl GetTraitObjectId for __SexShopTest__Trait__ {
                const TRAIT_OBJECT_ID: &'static str = "shop::divisions::sex::SexShopTestTraitObject";
            }
            impl TraitObjectConstDynMetadata for __SexShopTest__TraitObject__ {
                fn id_path(&self) -> CloneLazy<TraitPath> { CloneLazy::new(CloneClosure::new((), |_, _| "shop::divisions::sex::SexShopTest".into())) }
            }
            impl TraitObjectDynamicTypedMetadata for __SexShopTest__TraitObject__ {}

            #[allow(non_camel_case_types)]
            #[derive(Clone, PartialEq, Eq, Hash)]
            pub struct __SexShopProduct__Type__;
            impl ConstDynMetadata for __SexShopProduct__Type__ {
                fn raw_rust_module_path(&self) -> &'static str { module_path!() }
            }
            impl GetTypeId for __SexShopProduct__Type__ {
                const TYPE_ID: &'static str = "shop::divisions::sex::SexShopProduct";
            }
            impl TypeConstDynMetadata for __SexShopProduct__Type__ {
                fn id_path(&self) -> CloneLazy<TypePath> { CloneLazy::new(CloneClosure::new((), |_, _| Self::TYPE_ID.into())) }
                fn registrator(self) -> CloneClosure<ImmutableString, &'static mut rhai::Module, (), fn(ImmutableString, &mut rhai::Module)> {
                    CloneClosure::new(self.id_path().get().type_name().clone(), |name, parent_module| {
                        parent_module.set_custom_type::<crate::rhai_binding::working_example::shop::divisions::sex::SexShopProduct>(&name);
                    })
                }
                fn method_functions(&self) -> CloneLazy<Vec<MethodFunctionPath>> {
                    CloneLazy::new(CloneClosure::new((), |_, _| vec![
                        "shop::divisions::sex::SexShopProduct::name".into(),
                        "shop::divisions::sex::SexShopProduct::price_usd".into(),
                    ]))
                }
            }
            impl TypeDynamicTypedMetadata for __SexShopProduct__Type__ {}

            #[allow(non_camel_case_types)]
            #[derive(Clone, PartialEq, Eq, Hash)]
            pub struct __SexShopProduct__TypeBindingModule__;
            impl ConstDynMetadata for __SexShopProduct__TypeBindingModule__ {
                fn raw_rust_module_path(&self) -> &'static str { module_path!() }
            }
            impl NativeModuleConstDynMetadata for __SexShopProduct__TypeBindingModule__ {
                fn traits(&self) -> CloneLazy<Vec<TraitPath>> { CloneLazy::new(CloneClosure::new((), |_, _| vec![])) }
                fn types(&self) -> CloneLazy<Vec<TypePath>> { CloneLazy::new(CloneClosure::new((), |_, _| vec![])) }
                fn inherent_impls(&self) -> CloneLazy<Vec<InherentImplPath>> { CloneLazy::new(CloneClosure::new((), |_, _| vec![])) }
                fn trait_impls(&self) -> CloneLazy<Vec<TraitImplPath>> { CloneLazy::new(CloneClosure::new((), |_, _| vec![])) }
            }
            impl TypeBindingModuleConstDynMetadata for __SexShopProduct__TypeBindingModule__ {
                fn id_path(&self) -> CloneLazy<TypeBindingModulePath> { CloneLazy::new(CloneClosure::new((), |_, _| "shop::divisions::sex::SexShopProduct".into())) }
                fn item_associated_functions(&self) -> CloneLazy<Vec<ItemAssociatedFunctionPath>> {
                    CloneLazy::new(CloneClosure::new((), |_, _| vec![
                        "shop::divisions::sex::SexShopProduct::verify_price".into(),
                        "<shop::divisions::sex::SexShopProduct as shop::divisions::sex::SexShopTest>::test".into()
                    ]))
                }
                fn constructor_functions(&self) -> CloneLazy<Vec<ConstructorFunctionPath>> {
                    CloneLazy::new(CloneClosure::new((), |_, _| vec!["shop::divisions::sex::SexShopProduct::new_".into()]))
                }
                fn method_functions(&self) -> CloneLazy<Vec<MethodFunctionPath>> {
                    CloneLazy::new(CloneClosure::new((), |_, _| vec![
                        "shop::divisions::sex::SexShopProduct::name".into(),
                        "shop::divisions::sex::SexShopProduct::price_usd".into(),
                    ]))
                }
            }
            impl TypeBindingModuleDynamicTypedMetadata for __SexShopProduct__TypeBindingModule__ {}

            #[allow(non_camel_case_types)]
            #[derive(Clone, PartialEq, Eq, Hash)]
            pub struct __SexShopProduct__InherentImpl__;
            impl ConstDynMetadata for __SexShopProduct__InherentImpl__ {
                fn raw_rust_module_path(&self) -> &'static str { module_path!() }
            }
            impl InherentImplConstDynMetadata for __SexShopProduct__InherentImpl__ {
                fn id_path(&self) -> CloneLazy<InherentImplPath> { CloneLazy::new(CloneClosure::new((), |_, _| "shop::divisions::sex::SexShopProduct".into())) }
                fn constructor_functions(&self) -> CloneLazy<Vec<ConstructorFunctionPath>> {
                    CloneLazy::new(CloneClosure::new((), |_, _| vec!["shop::divisions::sex::SexShopProduct::new_".into()]))
                }
                fn method_functions(&self) -> CloneLazy<Vec<MethodFunctionPath>> {
                    CloneLazy::new(CloneClosure::new((), |_, _| vec![
                        "shop::divisions::sex::SexShopProduct::name".into(),
                        "shop::divisions::sex::SexShopProduct::price_usd".into(),
                    ]))
                }
                fn item_associated_functions(&self) -> CloneLazy<Vec<ItemAssociatedFunctionPath>> {
                    CloneLazy::new(CloneClosure::new((), |_, _| vec![]))
                }
            }
            impl InherentImplDynamicTypedMetadata for __SexShopProduct__InherentImpl__ {}

            #[allow(non_camel_case_types)]
            #[derive(Clone, PartialEq, Eq, Hash)]
            pub struct __SexShopProduct__as__SexShopTest__TraitImpl__;
            impl ConstDynMetadata for __SexShopProduct__as__SexShopTest__TraitImpl__ {
                fn raw_rust_module_path(&self) -> &'static str { module_path!() }
            }
            impl TraitImplConstDynMetadata for __SexShopProduct__as__SexShopTest__TraitImpl__ {
                fn id_path(&self) -> CloneLazy<TraitImplPath> { CloneLazy::new(CloneClosure::new((), |_, _| "<shop::divisions::sex::SexShopProduct as shop::divisions::sex::SexShopTest>".into())) }
                fn constructor_functions(&self) -> CloneLazy<Vec<ConstructorFunctionPath>> {
                    CloneLazy::new(CloneClosure::new((), |_, _| vec![]))
                }
                fn method_functions(&self) -> CloneLazy<Vec<MethodFunctionPath>> {
                    CloneLazy::new(CloneClosure::new((), |_, _| vec![]))
                }
                fn item_associated_functions(&self) -> CloneLazy<Vec<ItemAssociatedFunctionPath>> {
                    CloneLazy::new(CloneClosure::new((), |_, _| vec![
                        "<shop::divisions::sex::SexShopProduct as shop::divisions::sex::SexShopTest>::test".into(),
                    ]))
                }
            }
            impl TraitImplDynamicTypedMetadata for __SexShopProduct__as__SexShopTest__TraitImpl__ {}

            #[allow(non_camel_case_types)]
            #[derive(Clone, PartialEq, Eq, Hash)]
            pub struct __TestFunction__ModuleAssociatedFunction__;
            impl ConstDynMetadata for __TestFunction__ModuleAssociatedFunction__ {
                fn raw_rust_module_path(&self) -> &'static str { module_path!() }
            }
            impl ModuleAssociatedFunctionConstDynMetadata for __TestFunction__ModuleAssociatedFunction__ {
                fn id_path(&self) -> CloneLazy<ModuleAssociatedFunctionPath> { CloneLazy::new(CloneClosure::new((), |_, _| "shop::divisions::sex::test_function".into())) }
                fn registrator(self) -> CloneClosure<ImmutableString, &'static mut rhai::Module, (), fn(ImmutableString, &mut rhai::Module)> {
                    CloneClosure::new(self.id_path().get().function_name().clone(), |name, parent_module| {
                        rhai::FuncRegistration::new(name)
                            .set_into_module(parent_module, crate::rhai_binding::working_example::shop::divisions::sex::test_function);
                    })
                }
            }
            impl ModuleAssociatedFunctionDynamicTypedMetadata for __TestFunction__ModuleAssociatedFunction__ {}

            #[allow(non_camel_case_types)]
            #[derive(Clone, PartialEq, Eq, Hash)]
            pub struct __VerifyPrice__ItemAssociatedFunction__;
            impl ConstDynMetadata for __VerifyPrice__ItemAssociatedFunction__ {
                fn raw_rust_module_path(&self) -> &'static str { module_path!() }
            }
            impl ItemAssociatedFunctionConstDynMetadata for __VerifyPrice__ItemAssociatedFunction__ {
                fn id_path(&self) -> CloneLazy<ItemAssociatedFunctionPath> { CloneLazy::new(CloneClosure::new((), |_, _| "shop::divisions::sex::SexShopProduct::verify_price".into())) }
                fn registrator(self) -> CloneClosure<ImmutableString, &'static mut rhai::Module, (), fn(ImmutableString, &mut rhai::Module)> {
                    CloneClosure::new(self.id_path().get().function_name().clone(), |name, parent_module| {
                        rhai::FuncRegistration::new(name)
                            .set_into_module(parent_module, crate::rhai_binding::working_example::shop::divisions::sex::SexShopProduct::verify_price);
                    })
                }
            }
            impl ItemAssociatedFunctionDynamicTypedMetadata for __VerifyPrice__ItemAssociatedFunction__ {}

            #[allow(non_camel_case_types)]
            #[derive(Clone, PartialEq, Eq, Hash)]
            pub struct __Test__ItemAssociatedFunction__;
            impl ConstDynMetadata for __Test__ItemAssociatedFunction__ {
                fn raw_rust_module_path(&self) -> &'static str { module_path!() }
            }
            impl ItemAssociatedFunctionConstDynMetadata for __Test__ItemAssociatedFunction__ {
                fn id_path(&self) -> CloneLazy<ItemAssociatedFunctionPath> { CloneLazy::new(CloneClosure::new((), |_, _| "<shop::divisions::sex::SexShopProduct as shop::divisions::sex::SexShopTest>::test".into())) }
                fn registrator(self) -> CloneClosure<ImmutableString, &'static mut rhai::Module, (), fn(ImmutableString, &mut rhai::Module)> {
                    CloneClosure::new(self.id_path().get().function_name().clone(), |name, parent_module| {
                        rhai::FuncRegistration::new(name)
                            .set_into_module(parent_module, crate::rhai_binding::working_example::shop::divisions::sex::SexShopProduct::test);
                    })
                }
            }
            impl ItemAssociatedFunctionDynamicTypedMetadata for __Test__ItemAssociatedFunction__ {}

            #[allow(non_camel_case_types)]
            #[derive(Clone, PartialEq, Eq, Hash)]
            pub struct __New__ConstructorFunction__;
            impl ConstDynMetadata for __New__ConstructorFunction__ {
                fn raw_rust_module_path(&self) -> &'static str { module_path!() }
            }
            impl ConstructorFunctionConstDynMetadata for __New__ConstructorFunction__ {
                /// MACRO DESIGN RELATED NOTE; NOT TO BE GENERATED AS A CODE COMMENT; THIS IS A TEMPORARY META COMMENT:
                /// `new_` and not `new` because `new` is a reserved keyword in rhai;
                /// the macro should automatically and implicitly perform this transformation,
                /// so on the rhai-side we can just reliably ""escape"" keywords by appending an underscore, just like in rust.
                fn id_path(&self) -> CloneLazy<ConstructorFunctionPath> { CloneLazy::new(CloneClosure::new((), |_, _| "shop::divisions::sex::SexShopProduct::new_".into())) }
                fn registrator(self) -> CloneClosure<ImmutableString, &'static mut rhai::Module, (), fn(ImmutableString, &mut rhai::Module)> {
                    CloneClosure::new(self.id_path().get().function_name().clone(), |name, parent_module| {
                        rhai::FuncRegistration::new(name)
                            .set_into_module(parent_module, crate::rhai_binding::working_example::shop::divisions::sex::SexShopProduct::new_);
                    })
                }
            }
            impl ConstructorFunctionDynamicTypedMetadata for __New__ConstructorFunction__ {}

            #[allow(non_camel_case_types)]
            #[derive(Clone, PartialEq, Eq, Hash)]
            pub struct __Name__MethodFunction__;
            impl ConstDynMetadata for __Name__MethodFunction__ {
                fn raw_rust_module_path(&self) -> &'static str { module_path!() }
            }
            impl MethodFunctionConstDynMetadata for __Name__MethodFunction__ {
                fn id_path(&self) -> CloneLazy<MethodFunctionPath> { CloneLazy::new(CloneClosure::new((), |_, _| "shop::divisions::sex::SexShopProduct::name".into())) }
                fn registrator(self) -> CloneClosure<ImmutableString, &'static mut rhai::Engine, (), fn(ImmutableString, &mut rhai::Engine)> {
                    CloneClosure::new(self.id_path().get().function_name().clone(), |name, engine| {
                        engine.register_fn(name, crate::rhai_binding::working_example::shop::divisions::sex::SexShopProduct::name);
                    })
                }
            }
            impl MethodFunctionDynamicTypedMetadata for __Name__MethodFunction__ {}
            
            #[allow(non_camel_case_types)]
            #[derive(Clone, PartialEq, Eq, Hash)]
            pub struct __PriceUsd__MethodFunction__;
            impl ConstDynMetadata for __PriceUsd__MethodFunction__ {
                fn raw_rust_module_path(&self) -> &'static str { module_path!() }
            }
            impl MethodFunctionConstDynMetadata for __PriceUsd__MethodFunction__ {
                fn id_path(&self) -> CloneLazy<MethodFunctionPath> { CloneLazy::new(CloneClosure::new((), |_, _| "shop::divisions::sex::SexShopProduct::price_usd".into())) }
                fn registrator(self) -> CloneClosure<ImmutableString, &'static mut rhai::Engine, (), fn(ImmutableString, &mut rhai::Engine)> {
                    CloneClosure::new(self.id_path().get().function_name().clone(), |name, engine| {
                        engine.register_fn(name, crate::rhai_binding::working_example::shop::divisions::sex::SexShopProduct::price_usd);
                    })
                }
            }
            impl MethodFunctionDynamicTypedMetadata for __PriceUsd__MethodFunction__ {}
// >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>> //

// The actual end-user code
// <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<< //
        }
    }
}
// >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>> //