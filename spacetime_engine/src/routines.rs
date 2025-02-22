use crate::prelude::*;

routine_mod! {
    name: "SpacetimeEngine",
    routines: [
        Startup {
            core_functions: [
                fn Run |ctx| {
                    let shader_handle = match ctx.await_workflow(SetupTextureGeneration {
                        name: "GPU::SetupTextureGenerator",
                        input: {
                            shader_name: "texture_generators/example_compute_uv",
                            shader_path: "assets/shaders/texture_generators/example_compute_uv.wgsl"
                        },
                    }).await {
                        Ok(_) => {}
                        Err(err) => ctx.yield_error(err).await
                    };
                
                    let texture_handle = match ctx.await_workflow(GenerateTexture {
                        name: "GPU::GenerateTexture",
                        input: {
                            shader_name: "texture_generators/example_compute_uv",
                            texture_size: CONFIG.get::<f32>("chunk/size") as usize
                        },
                    }).await {
                        Ok(_) => {}
                        Err(err) => ctx.yield_error(err).await
                    };
                
                    match ctx.await_workflow(SpawnChunk {
                        name: "Chunk::Spawn",
                        input: {
                            chunk_coord: (0, 0),
                            chunk_owner: None,
                            metric_texture: texture_handle
                        },
                    }).await {
                        Ok(_) => {}
                        Err(err) => ctx.yield_error(err).await
                    };
        
                    ctx.yield_return().await
                },
            ]
        }
    ],
}
