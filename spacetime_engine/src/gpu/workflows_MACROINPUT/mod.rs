use spacetime_engine_macros::define_workflow_mod;

define_workflow_mod!("Gpu", [
    workflow!("SetupTextureGenerator")
    workflow!("GenerateTexture")
]);
