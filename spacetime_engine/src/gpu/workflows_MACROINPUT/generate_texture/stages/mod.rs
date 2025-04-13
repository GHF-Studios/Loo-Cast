use spacetime_engine_macros::define_worfklow_stages;

define_worfklow_stages![
    stage!("PrepareRequest")
    stage!("GetTextureView")
    stage!("DispatchCompute")
    stage!("WaitForCompute")
];
