use proc_macro2::{Delimiter, Group, Ident, Punct, Spacing, Span, TokenStream, TokenTree};
use syn::parse2;
use super::workflow_id::IdMacro;
use super::workflow_invocation::WorkflowMacro;

pub enum SubMacro {
    WorkflowId,
    WorkflowInvocation,
}

impl SubMacro {
    pub fn expand_in(self, input: TokenStream) -> TokenStream {
        self.expand_in_internal(input)
    }

    fn expand_in_internal(&self, input: TokenStream) -> TokenStream {
        let mut output = TokenStream::new();
        let mut iter = input.into_iter().peekable();

        while let Some(tt) = iter.next() {
            match &tt {
                TokenTree::Ident(ident) => {
                    let matches_submacro = match self {
                        SubMacro::WorkflowId => ident == "id",
                        SubMacro::WorkflowInvocation => ident == "workflow",
                    };

                    if matches_submacro {
                        // Expect '!' next
                        if let Some(TokenTree::Punct(punct)) = iter.peek() {
                            if punct.as_char() == '!' && punct.spacing() == Spacing::Alone {
                                iter.next(); // consume '!'
                                if let Some(TokenTree::Group(group)) = iter.next() {
                                    if group.delimiter() == Delimiter::Parenthesis {
                                        let stream = group.stream();

                                        let result = match self {
                                            SubMacro::WorkflowId => {
                                                parse2::<IdMacro>(stream).map(|m| m.generate())
                                            }
                                            SubMacro::WorkflowInvocation => {
                                                parse2::<WorkflowMacro>(stream).map(|m| m.generate())
                                            }
                                        };

                                        match result {
                                            Ok(expanded) => {
                                                output.extend(expanded);
                                            }
                                            Err(err) => {
                                                output.extend(err.to_compile_error());
                                            }
                                        }

                                        continue;
                                    }
                                }
                            }
                        }
                    }

                    // If it doesn't match submacro name or fails structure
                    output.extend(std::iter::once(tt));
                }

                TokenTree::Group(group) => {
                    // Only recurse into inner blocks/groups if allowed
                    // Since nesting is only allowed for id! inside workflow!, we assume
                    // orchestration handles this ordering externally.
                    let inner = self.expand_in_internal(group.stream());
                    let mut new_group = Group::new(group.delimiter(), inner);
                    new_group.set_span(group.span());
                    output.extend(std::iter::once(TokenTree::Group(new_group)));
                }

                _ => {
                    output.extend(std::iter::once(tt));
                }
            }
        }

        output
    }
}