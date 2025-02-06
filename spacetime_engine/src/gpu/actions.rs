use bevy::prelude::*;

use crate::action::{resources::ActionTypeModuleRegistry, target::ActionTypeModule};

// TODO: Create macro to define actions and their types in a more streamlined and natural way
// TODO: Instead of an Action Target Type, we should register an Action Module Type, and integrate that change everywhere that's related

pub fn initialize_action_type_module(action_type_module_registry: &mut ResMut<ActionTypeModuleRegistry>) {
    action_type_module_registry.register(
        ActionTypeModule {
            name: "GPU".to_owned(),
            action_types: vec![
                generate_texture::create_action_type(),
            ],
        },
    );
}

pub mod generate_texture {
    use bevy::prelude::*;

    use crate::{action::{stage::{ActionStage, ActionStageAsync, ActionStageEcs}, stage_io::{ActionIO, InputState, OutputState}, types::ActionType}, chunk::{components::ChunkComponent, functions::chunk_pos_to_world, resources::ChunkManager}, config::statics::CONFIG};

    pub struct Input(pub GenerateTextureInput);

    pub struct GenerateTextureInput {
        pub texture_size: usize
    }

    pub struct Output(pub Result<Handle<Image>, String>);

    pub fn create_action_type() -> ActionType {
        ActionType {
            name: "GenerateTexture".to_owned(),
            validation: Box::new(|io: ActionIO<InputState>, world: &mut World| -> Result<ActionIO<OutputState>, String> {
                let (action_input, io) = io.get_input::<Input>();
                let stage_input = action_input.0;
                Ok(io.set_output(stage_input))
            }),
            stages: vec![
                ActionStage::Async(ActionStageAsync {
                    name: "GenerateTexture".to_owned(),
                    function: Box::new(|io: ActionIO<InputState>| Box::pin(async move {
                        let (input, io) = io.get_input::<GenerateTextureInput>();
                        io.set_output(())
                    })),
                }),
            ],
        }
    }
}