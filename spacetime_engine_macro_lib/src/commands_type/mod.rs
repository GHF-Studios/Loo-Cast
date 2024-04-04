pub mod command_type;

use command_type::*;
use syn::{
    parse::{Parse, ParseStream}, spanned::Spanned, Ident, LitStr, Path, Token
};

pub struct CommandsModuleType {
    pub module_id: LitStr,
    pub module_path: Path,
    pub command_types: CommandTypes,
}

impl Parse for CommandsModuleType {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let module_id = input.parse::<Ident>()?;
        let module_id = module_id.to_string();
        let module_id = LitStr::new(&module_id, module_id.span());

        let content;
        syn::braced!(content in input);

        let module_path_label = content.parse::<Ident>()?;
        let span = module_path_label.span();
        let module_path_label = module_path_label.to_string();

        if module_path_label != "module_path" {
            return Err(syn::Error::new(span, "Expected 'module_path' Label"));
        }

        content.parse::<Token![:]>()?;

        let module_path = content.parse::<Path>()?;

        content.parse::<Token![,]>()?;

        let module_commands_label = content.parse::<Ident>()?;
        let span = module_commands_label.span();
        let module_commands_label = module_commands_label.to_string();

        if module_commands_label != "commands" {
            return Err(syn::Error::new(span, "Expected 'commands' Label"));
        }

        content.parse::<Token![:]>()?;

        let command_types = CommandTypes::parse(&content)?;

        Ok(CommandsModuleType {
            module_id,
            module_path,
            command_types
        })
    }

}