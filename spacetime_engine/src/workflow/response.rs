use std::any::Any;

pub enum WorkflowResponse {
    None(TypedWorkflowResponse),
    E(TypedWorkflowResponseE),
    O(TypedWorkflowResponseO),
    OE(TypedWorkflowResponseOE),
    I(TypedWorkflowResponseI),
    IE(TypedWorkflowResponseIE),
    IO(TypedWorkflowResponseIO),
    IOE(TypedWorkflowResponseIOE),
}

pub struct TypedWorkflowResponse;
pub struct TypedWorkflowResponseE(pub Result<(), Box<dyn Any + Send + Sync>>);
pub struct TypedWorkflowResponseO(pub Box<dyn Any + Send + Sync>);
pub struct TypedWorkflowResponseOE(pub Result<Box<dyn Any + Send + Sync>, Box<dyn Any + Send + Sync>>);
pub struct TypedWorkflowResponseI;
pub struct TypedWorkflowResponseIE(pub Result<(), Box<dyn Any + Send + Sync>>);
pub struct TypedWorkflowResponseIO(pub Box<dyn Any + Send + Sync>);
pub struct TypedWorkflowResponseIOE(pub Result<Box<dyn Any + Send + Sync>, Box<dyn Any + Send + Sync>>);

impl TypedWorkflowResponse {
    pub fn unpack(self) {}
}
impl TypedWorkflowResponseE {
    pub fn unpack<E: 'static>(self) -> Result<(), E> {
        let downcast_error_result = match self.0 {
            Ok(_) => return Ok(()),
            Err(raw_error) => raw_error.downcast()
        };

        let error = match downcast_error_result {
            Ok(error) => error,
            Err(_) => panic!("Failed to unpack TypedWorkflowResponseE")
        };

        Err(*error)
    }
}
impl TypedWorkflowResponseO {
    pub fn unpack<O: 'static>(self) -> O {
        let downcast_output_result = self.0.downcast();

        let output = match downcast_output_result {
            Ok(output) => output,
            Err(_) => panic!("Failed to unpack TypedWorkflowResponseO")
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
                    Err(_) => panic!("Failed to unpack TypedWorkflowResponseOE")
                };

                Ok(*output)
            },
            Err(raw_error) => {
                let downcast_error_result = raw_error.downcast();

                let error = match downcast_error_result {
                    Ok(error) => error,
                    Err(_) => panic!("Failed to unpack TypedWorkflowResponseOE")
                };

                Err(*error)
            }
        }
    }
}
impl TypedWorkflowResponseI {
    pub fn unpack(self) {}
}
impl TypedWorkflowResponseIE {
    pub fn unpack<E: 'static>(self) -> Result<(), E> {
        let downcast_error_result = match self.0 {
            Ok(_) => return Ok(()),
            Err(raw_error) => raw_error.downcast()
        };

        let error = match downcast_error_result {
            Ok(error) => error,
            Err(_) => panic!("Failed to unpack TypedWorkflowResponseIE")
        };

        Err(*error)
    }
}
impl TypedWorkflowResponseIO {
    pub fn unpack<O: 'static>(self) -> O {
        let downcast_output_result = self.0.downcast();

        let output = match downcast_output_result {
            Ok(output) => output,
            Err(_) => panic!("Failed to unpack TypedWorkflowResponseIO")
        };

        *output
    }
}
impl TypedWorkflowResponseIOE {
    pub fn unpack<O: 'static, E: 'static>(self) -> Result<O, E> {
        match self.0 {
            Ok(output) => {
                let downcast_output_result = output.downcast();

                let output = match downcast_output_result {
                    Ok(output) => output,
                    Err(_) => panic!("Failed to unpack TypedWorkflowResponseIOE")
                };

                Ok(*output)
            },
            Err(raw_error) => {
                let downcast_error_result = raw_error.downcast();

                let error = match downcast_error_result {
                    Ok(error) => error,
                    Err(_) => panic!("Failed to unpack TypedWorkflowResponseIOE")
                };

                Err(*error)
            }
        }
    }
}
