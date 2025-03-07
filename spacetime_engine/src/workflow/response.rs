use std::any::Any;

pub struct WorkflowResponse;
pub struct WorkflowResponseE(pub Result<(), Box<dyn Any + Send + Sync>>);
pub struct WorkflowResponseO(pub Box<dyn Any + Send + Sync>);
pub struct WorkflowResponseOE(pub Result<Box<dyn Any + Send + Sync>, Box<dyn Any + Send + Sync>>);
pub struct WorkflowResponseS;
pub struct WorkflowResponseSE(pub Result<(), Box<dyn Any + Send + Sync>>);
pub struct WorkflowResponseSO(pub Box<dyn Any + Send + Sync>);
pub struct WorkflowResponseSOE(pub Result<Box<dyn Any + Send + Sync>, Box<dyn Any + Send + Sync>>);
pub struct WorkflowResponseI;
pub struct WorkflowResponseIE(pub Result<(), Box<dyn Any + Send + Sync>>);
pub struct WorkflowResponseIO(pub Box<dyn Any + Send + Sync>);
pub struct WorkflowResponseIOE(pub Result<Box<dyn Any + Send + Sync>, Box<dyn Any + Send + Sync>>);
pub struct WorkflowResponseIS;
pub struct WorkflowResponseISE(pub Result<(), Box<dyn Any + Send + Sync>>);
pub struct WorkflowResponseISO(pub Box<dyn Any + Send + Sync>);
pub struct WorkflowResponseISOE(pub Result<Box<dyn Any + Send + Sync>, Box<dyn Any + Send + Sync>>);

impl WorkflowResponse {}
impl WorkflowResponseE {
    pub fn unpack<E>(self) -> Result<(), E> {
        let downcast_error_result = match self.0 {
            Ok(_) => return Ok(()),
            Err(raw_error) => raw_error.downcast()
        };

        let error = match downcast_error_result {
            Ok(error) => error,
            Err(_) => panic!("Failed to unpack WorkflowResponseE")
        };

        Err(error)
    }
}
impl WorkflowResponseO {
    pub fn unpack<O>(self) -> O {
        let downcast_output_result = self.0.downcast();

        let output = match downcast_output_result {
            Ok(output) => output,
            Err(_) => panic!("Failed to unpack WorkflowResponseO")
        };

        output
    }
}
impl WorkflowResponseOE {
    pub fn unpack<O, E>(self) -> Result<O, E> {
        match self.0 {
            Ok(output) => {
                let downcast_output_result = output.downcast();

                let output = match downcast_output_result {
                    Ok(output) => output,
                    Err(_) => panic!("Failed to unpack WorkflowResponseOE")
                };

                Ok(output)
            },
            Err(raw_error) => {
                let downcast_error_result = raw_error.downcast();

                let error = match downcast_error_result {
                    Ok(error) => error,
                    Err(_) => panic!("Failed to unpack WorkflowResponseOE")
                };

                Err(error)
            }
        }
    }
}
impl WorkflowResponseS {}
impl WorkflowResponseSE {
    pub fn unpack<E>(self) -> Result<(), E> {
        let downcast_error_result = match self.0 {
            Ok(_) => return Ok(()),
            Err(raw_error) => raw_error.downcast()
        };

        let error = match downcast_error_result {
            Ok(error) => error,
            Err(_) => panic!("Failed to unpack WorkflowResponseSE")
        };

        Err(error)
    }
}
impl WorkflowResponseSO {
    pub fn unpack<O>(self) -> O {
        let downcast_output_result = self.0.downcast();

        let output = match downcast_output_result {
            Ok(output) => output,
            Err(_) => panic!("Failed to unpack WorkflowResponseSO")
        };

        output
    }
}
impl WorkflowResponseSOE {
    pub fn unpack<O, E>(self) -> Result<O, E> {
        match self.0 {
            Ok(output) => {
                let downcast_output_result = output.downcast();

                let output = match downcast_output_result {
                    Ok(output) => output,
                    Err(_) => panic!("Failed to unpack WorkflowResponseSOE")
                };

                Ok(output)
            },
            Err(raw_error) => {
                let downcast_error_result = raw_error.downcast();

                let error = match downcast_error_result {
                    Ok(error) => error,
                    Err(_) => panic!("Failed to unpack WorkflowResponseSOE")
                };

                Err(error)
            }
        }
    }
}
impl WorkflowResponseI {}
impl WorkflowResponseIE {
    pub fn unpack<E>(self) -> Result<(), E> {
        let downcast_error_result = match self.0 {
            Ok(_) => return Ok(()),
            Err(raw_error) => raw_error.downcast()
        };

        let error = match downcast_error_result {
            Ok(error) => error,
            Err(_) => panic!("Failed to unpack WorkflowResponseIE")
        };

        Err(error)
    }
}
impl WorkflowResponseIO {
    pub fn unpack<O>(self) -> O {
        let downcast_output_result = self.0.downcast();

        let output = match downcast_output_result {
            Ok(output) => output,
            Err(_) => panic!("Failed to unpack WorkflowResponseIO")
        };

        output
    }
}
impl WorkflowResponseIOE {
    pub fn unpack<O, E>(self) -> Result<O, E> {
        match self.0 {
            Ok(output) => {
                let downcast_output_result = output.downcast();

                let output = match downcast_output_result {
                    Ok(output) => output,
                    Err(_) => panic!("Failed to unpack WorkflowResponseIOE")
                };

                Ok(output)
            },
            Err(raw_error) => {
                let downcast_error_result = raw_error.downcast();

                let error = match downcast_error_result {
                    Ok(error) => error,
                    Err(_) => panic!("Failed to unpack WorkflowResponseIOE")
                };

                Err(error)
            }
        }
    }
}
impl WorkflowResponseIS {}
impl WorkflowResponseISE {
    pub fn unpack<E>(self) -> Result<(), E> {
        let downcast_error_result = match self.0 {
            Ok(_) => return Ok(()),
            Err(raw_error) => raw_error.downcast()
        };

        let error = match downcast_error_result {
            Ok(error) => error,
            Err(_) => panic!("Failed to unpack WorkflowResponseISE")
        };

        Err(error)
    }
}
impl WorkflowResponseISO {
    pub fn unpack<O>(self) -> O {
        let downcast_output_result = self.0.downcast();

        let output = match downcast_output_result {
            Ok(output) => output,
            Err(_) => panic!("Failed to unpack WorkflowResponseISO")
        };

        output
    }
}
impl WorkflowResponseISOE {
    pub fn unpack<O, E>(self) -> Result<O, E> {
        match self.0 {
            Ok(output) => {
                let downcast_output_result = output.downcast();

                let output = match downcast_output_result {
                    Ok(output) => output,
                    Err(_) => panic!("Failed to unpack WorkflowResponseISOE")
                };

                Ok(output)
            },
            Err(raw_error) => {
                let downcast_error_result = raw_error.downcast();

                let error = match downcast_error_result {
                    Ok(error) => error,
                    Err(_) => panic!("Failed to unpack WorkflowResponseISOE")
                };

                Err(error)
            }
        }
    }
}
