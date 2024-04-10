pub mod block_type;
pub mod signature_type;

use block_type::*;
use signature_type::*;
use syn::parse::{Parse, ParseStream};

#[derive(Clone)]
pub struct CommandCodeType {
    pub code_signature: CommandCodeSignature,
    pub code_block: CommandCodeBlock,
    pub interpolation: String
}

impl Parse for CommandCodeType {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let code_signature = input.parse::<CommandCodeSignature>()?;
        let code_block = input.parse::<CommandCodeBlock>()?;

        Ok(CommandCodeType {
            code_signature,
            code_block,
            interpolation: format!("code_signature: ({})", code_signature.interpolation)
        })
    }

}
