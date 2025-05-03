use bevy::prelude::*;
use tokio::task::JoinHandle;
use spacetime_engine_macros::define_composite_workflow;

pub(crate) fn update_chunk_loader_system(mut composite_workflow_handle: Local<Option<JoinHandle<()>>>) {
    let some_stuff = do_something();

    define_workflow_with_captured!(some_stuff, JustDoIt {
        println!("{:?}", captured_context()); // <- how you access captured value inside

        let categorize_chunks_output = workflow!(O, ChunkLoader::CategorizeChunks);
        workflow!(I, ChunkLoader::LoadChunks, Input {
            inputs: categorize_chunks_output.load_chunk_inputs
        });
        workflow!(I, ChunkLoader::UnloadChunks, Input {
            inputs: categorize_chunks_output.unload_chunk_inputs
        });

        other_things(captured_context());
        println!("{:?}", captured_context());
    });

    match *composite_workflow_handle {
        Some(ref handle) if handle.is_finished() => {
            *composite_workflow_handle = None;
        },
        Some(_) => todo!(),
        None => {
            *composite_workflow_handle = Some(crate::workflow::statics::COMPOSITE_WORKFLOW_RUNTIME
                .lock()
                .unwrap()
                .spawn(Box::pin(just_do_it())));
        }
    }
}

thread_local! {
    pub static FOO: Cell<u32> = Cell::new(1);

    static BAR: RefCell<Vec<f32>> = RefCell::new(vec![1.0, 2.0]);
}

fn foo() {
    let msg = "Hello World!";

    complex_macro_which_expands_to_a_function_and_some_types_and_is_baaaasically_a_glorified_function_declaration!(msg, Bar {
        println!("sex");
        println!(msg);
    });

    bar();

    println!("Finished with message: {}", msg)
}

fn foo() {
    let mut msg = "Hello World!";
    
    fn bar() {
        println!("sex");
        println!(msg);
        msg = "Mashallah";
    }

    bar();

    println!("Finished with message: {}", msg)
}
