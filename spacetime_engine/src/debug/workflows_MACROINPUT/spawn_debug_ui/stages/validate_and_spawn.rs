crate::workflow_stage_util!("ValidateAndSpawn");

pub mod core_types {
    crate::workflow_stage_core_types_util!();

    #[derive(SystemParam)]
    pub struct MainAccess<'w, 's> {
        commands: Commands<'w, 's>,
    }
}

pub mod core_functions {
    crate::workflow_stage_core_functions_util!();
    crate::workflow_stage_core_function_util!(run_ecs);

    pub fn run_ecs_inner(main_access: MainAccess) {
        main_access.commands.spawn((
            PerfUiRoot::default(),
            PerfUiFramerateEntries::default(),
            PerfUiSystemEntries::default(),
            PerfUiEntryEntityCount::default(),
        ));
    }
}
