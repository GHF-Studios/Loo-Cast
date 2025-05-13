use std::any::{type_name, Any};

use crate::debug::type_name::get_type_name;

pub enum WorkflowResponse {
    E(TypedWorkflowResponseE),
    O(TypedWorkflowResponseO),
    OE(TypedWorkflowResponseOE),
}

pub struct TypedWorkflowResponseE(pub Result<(), Box<dyn Any + Send + Sync>>);
pub struct TypedWorkflowResponseO(pub Box<dyn Any + Send + Sync>);
pub struct TypedWorkflowResponseOE(
    pub Result<Box<dyn Any + Send + Sync>, Box<dyn Any + Send + Sync>>,
);

impl TypedWorkflowResponseE {
    pub fn unpack<E: 'static + Any + Send + Sync>(self) -> Result<(), E> {
        let downcast_error_result = match self.0 {
            Ok(_) => return Ok(()),
            Err(error) => error.downcast(),
        };

        let error = match downcast_error_result {
            Ok(error) => error,
            Err(original) => {
                let actual_type = get_type_name(&original);
                panic!(
                    "Failed to unpack TypedWorkflowResponseE:\n  expected = {}\n  actual = {}",
                    type_name::<E>(),
                    actual_type
                );
            },
        };

        Err(*error)
    }
}

impl TypedWorkflowResponseO {
    pub fn unpack<O: 'static + Any + Send + Sync>(self) -> O {
        let downcast_output_result = self.0.downcast();

        let output = match downcast_output_result {
            Ok(output) => output,
            Err(original) => {
                let actual_type = get_type_name(&original);
                panic!(
                "Failed to unpack TypedWorkflowResponseO:\n  expected = {}\n  actual = {}",
                    type_name::<O>(),
                    actual_type
                );
            }
        };

        *output
    }
}

impl TypedWorkflowResponseOE {
    pub fn unpack<O: 'static + Any + Send + Sync, E: 'static + Any + Send + Sync>(
        self,
    ) -> Result<O, E> {
        match self.0 {
            Ok(output) => {
                let downcast_output_result = output.downcast();

                let output = match downcast_output_result {
                    Ok(output) => output,
                    Err(original) => {
                        let actual_type = get_type_name(&original);
                        panic!(
                            "Failed to unpack TypedWorkflowResponseOE (Ok variant):\n  expected = {}\n  actual = {}",
                            type_name::<O>(),
                            actual_type
                        );
                    },
                };

                Ok(*output)
            }
            Err(error) => {
                let downcast_error_result = error.downcast();

                let error = match downcast_error_result {
                    Ok(error) => error,
                    Err(original) => {
                        let actual_type = get_type_name(&original);
                        panic!(
                        "Failed to unpack TypedWorkflowResponseOE (Err variant):\n  expected = {}\n  actual = {}",
                            type_name::<E>(),
                            actual_type
                        );
                    },
                };

                Err(*error)
            }
        }
    }
}
