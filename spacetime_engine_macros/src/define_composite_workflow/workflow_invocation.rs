use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{
    parse::{Parse, ParseStream, Result},
    ExprStruct, Token,
};

use super::workflow_id::{TypedWorkflowId, WorkflowId};

pub struct WorkflowMacro(pub TypedWorkflowInvocation);

impl Parse for WorkflowMacro {
    fn parse(input: ParseStream) -> Result<Self> {
        let invocation = input.parse::<TypedWorkflowInvocation>()?;
        Ok(WorkflowMacro(invocation))
    }
}

impl WorkflowMacro {
    pub fn generate(self) -> TokenStream {
        self.0.generate()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WorkflowSignature {
    None,
    E,
    O,
    OE,
    I,
    IE,
    IO,
    IOE,
}

pub enum TypedWorkflowInvocation {
    None(WorkflowInvocation),
    E(WorkflowInvocation),
    O(WorkflowInvocation),
    OE(WorkflowInvocation),
    I(WorkflowInvocation),
    IE(WorkflowInvocation),
    IO(WorkflowInvocation),
    IOE(WorkflowInvocation),
}

impl Parse for TypedWorkflowInvocation {
    fn parse(input: ParseStream) -> Result<Self> {
        // Step 1: Parse the ID part using existing logic
        let id = input.parse::<TypedWorkflowId>()?;

        // Step 2: Check for optional ", Input { ... }"
        let has_input = input.peek(Token![,]);
        let input_block = if has_input {
            input.parse::<Token![,]>()?;
            let input_ident: syn::Ident = input.parse()?;
            if input_ident != "Input" {
                return Err(syn::Error::new_spanned(
                    input_ident,
                    "Expected 'Input' keyword after comma.",
                ));
            }

            let input_struct: ExprStruct = input.parse()?;
            Some(input_struct)
        } else {
            None
        };

        // Step 3: Construct correct variant
        let invocation = WorkflowInvocation {
            id,
            input: input_block,
        };

        Ok(match &invocation.id {
            TypedWorkflowId::None(_) => Self::None(invocation),
            TypedWorkflowId::E(_) => Self::E(invocation),
            TypedWorkflowId::O(_) => Self::O(invocation),
            TypedWorkflowId::OE(_) => Self::OE(invocation),
            TypedWorkflowId::I(_) => Self::I(invocation),
            TypedWorkflowId::IE(_) => Self::IE(invocation),
            TypedWorkflowId::IO(_) => Self::IO(invocation),
            TypedWorkflowId::IOE(_) => Self::IOE(invocation),
        })
    }
}

impl TypedWorkflowInvocation {
    pub fn generate(self) -> TokenStream {
        match self {
            Self::None(invocation) => {
                let id = invocation.id.generate();
                let input = invocation.input;
                if input.is_some() {
                    unreachable!("Input is not supported for workflow signature 'None' ")
                }

                quote! {
                    #[WorkflowSignature(None)]
                    {
                        type T = #id;
                        crate::workflow::functions::run_workflow::<T>().await;
                    }
                }
            },
            Self::E(invocation) => {
                let id = invocation.id.generate();
                let input = invocation.input;
                if input.is_some() {
                    unreachable!("Input is not supported for workflow signature 'E' ")
                }

                quote! {
                    #[WorkflowSignature(Expected)]
                    {
                        type T = #id;
                        type E = <T as crate::workflow::traits::WorkflowTypeE>::Error;
                        let response: Result<(), E> = crate::workflow::functions::run_workflow_e::<T>().await;
                        response.map_err(Into::<__CompositeWorkflowError__>::into)
                    }
                }
            },
            Self::O(invocation) => {
                let id = invocation.id.generate();
                let input = invocation.input;
                if input.is_some() {
                    unreachable!("Input is not supported for workflow signature 'O' ")
                }

                quote! {
                    #[WorkflowSignature(O)]
                    {
                        type T = #id;
                        type O = <T as crate::workflow::traits::WorkflowTypeO>::Output;
                        let response: Result<O, ()> = crate::workflow::functions::run_workflow_o::<T>().await;
                        response.map_err(Into::<__CompositeWorkflowError__>::into)
                    }
                }
            },
            Self::OE(invocation) => {
                let id = invocation.id.generate();
                let input = invocation.input;
                if input.is_some() {
                    unreachable!("Input is not supported for workflow signature 'OE' ")
                }

                quote! {
                    #[WorkflowSignature(OE)]
                    {
                        type T = #id;
                        type O = <T as crate::workflow::traits::WorkflowTypeOE>::Output;
                        type E = <T as crate::workflow::traits::WorkflowTypeOE>::Error;
                        let response: Result<O, E> = crate::workflow::functions::run_workflow_oe::<T>().await;
                        response.map_err(Into::<__CompositeWorkflowError__>::into)
                    }
                }
            },
            Self::I(invocation) => {
                let id = invocation.id.generate();
                let input = invocation.input;
                let input = match input {
                    Some(input) => quote! { #input},
                    None => {
                        unreachable!("Input is required for workflow signature 'I' ")
                    }
                };

                quote! {
                    #[WorkflowSignature(I)]
                    {
                        type T = #id;
                        type I = <T as crate::workflow::traits::WorkflowTypeI>::Input;
                        crate::workflow::functions::run_workflow_i::<T>(I #input).await;
                    }
                }
            },
            Self::IE(invocation) => {
                let id = invocation.id.generate();
                let input = invocation.input;
                let input = match input {
                    Some(input) => quote! { #input},
                    None => {
                        unreachable!("Input is required for workflow signature 'IE' ")
                    }
                };

                quote! {
                    #[WorkflowSignature(IE)]
                    {
                        type T = #id;
                        type I = <T as crate::workflow::traits::WorkflowTypeIE>::Input;
                        type E = <T as crate::workflow::traits::WorkflowTypeIE>::Error;
                        let response: Result<(), E> = crate::workflow::functions::run_workflow_ie::<T>(I #input).await;
                        response.map_err(Into::<__CompositeWorkflowError__>::into)
                    }
                }
            },
            Self::IO(invocation) => {
                let id = invocation.id.generate();
                let input = invocation.input;
                let input = match input {
                    Some(input) => quote! { #input},
                    None => {
                        unreachable!("Input is required for workflow signature 'IO' ")
                    }
                };

                quote! {
                    #[WorkflowSignature(IO)]
                    {
                        type T = #id;
                        type I = <T as crate::workflow::traits::WorkflowTypeIO>::Input;
                        type O = <T as crate::workflow::traits::WorkflowTypeIO>::Output;
                        let response: Result<O, ()> = crate::workflow::functions::run_workflow_io::<T>(I #input).await;
                        response.map_err(Into::<__CompositeWorkflowError__>::into)
                    }
                }
            },
            Self::IOE(invocation) => {
                let id = invocation.id.generate();
                let input = invocation.input;
                let input = match input {
                    Some(input) => quote! { #input},
                    None => {
                        unreachable!("Input is required for workflow signature 'IOE' ")
                    }
                };

                quote! {
                    #[WorkflowSignature(IOE)]
                    {
                        type T = #id;
                        type I = <T as crate::workflow::traits::WorkflowTypeIOE>::Input;
                        type O = <T as crate::workflow::traits::WorkflowTypeIOE>::Output;
                        type E = <T as crate::workflow::traits::WorkflowTypeIOE>::Error;
                        let response: Result<O, E> = crate::workflow::functions::run_workflow_ioe::<T>(I #input).await;
                        response.map_err(Into::<__CompositeWorkflowError__>::into)
                    }
                }
            },
        }
    }

    pub fn id(self) -> TypedWorkflowId {
        match self {
            Self::None(invocation) => invocation.id,
            Self::E(invocation) => invocation.id,
            Self::O(invocation) => invocation.id,
            Self::OE(invocation) => invocation.id,
            Self::I(invocation) => invocation.id,
            Self::IE(invocation) => invocation.id,
            Self::IO(invocation) => invocation.id,
            Self::IOE(invocation) => invocation.id,
        }
    }

    pub fn input(self) -> Option<ExprStruct> {
        match self {
            Self::None(_) => None,
            Self::E(_) => None,
            Self::O(_) => None,
            Self::OE(_) => None,
            Self::I(invocation) => invocation.input,
            Self::IE(invocation) => invocation.input,
            Self::IO(invocation) => invocation.input,
            Self::IOE(invocation) => invocation.input,
        }
    }
}

pub struct WorkflowInvocation {
    pub id: TypedWorkflowId,
    pub input: Option<ExprStruct>,
}
