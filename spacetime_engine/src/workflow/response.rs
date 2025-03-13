use std::any::Any;

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
    pub fn unpack<E: 'static>(self) -> Result<(), E> {
        let downcast_error_result = match self.0 {
            Ok(_) => return Ok(()),
            Err(raw_error) => raw_error.downcast(),
        };

        let error = match downcast_error_result {
            Ok(error) => error,
            Err(_) => panic!("Failed to unpack TypedWorkflowResponseE"),
        };

        Err(*error)
    }
}
impl TypedWorkflowResponseO {
    pub fn unpack<O: 'static>(self) -> O {
        let downcast_output_result = self.0.downcast();

        let output = match downcast_output_result {
            Ok(output) => output,
            Err(_) => panic!("Failed to unpack TypedWorkflowResponseO"),
        };

        *output
    }
}
impl TypedWorkflowResponseOE {
    pub fn unpack<O: 'static, E: 'static>(self) -> Result<O, E> {
        match self.0 {
            Ok(output) => {
                let downcast_output_result = output.downcast();

                let output = match downcast_output_result {
                    Ok(output) => output,
                    Err(_) => panic!("Failed to unpack TypedWorkflowResponseOE"),
                };

                Ok(*output)
            }
            Err(raw_error) => {
                let downcast_error_result = raw_error.downcast();

                let error = match downcast_error_result {
                    Ok(error) => error,
                    Err(_) => panic!("Failed to unpack TypedWorkflowResponseOE"),
                };

                Err(*error)
            }
        }
    }
}
