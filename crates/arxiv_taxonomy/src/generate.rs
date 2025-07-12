use heck::ToUpperCamelCase;
use prettyplease::unparse;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::parse_file;

pub fn new_enum(ident: String, variants: Vec<String>) -> TokenStream {
	let variants_tokens = variants.iter().map(|v| format_ident!("{}", v));
	let enum_ident = format_ident!("{}", ident.to_upper_camel_case());

	quote! {
		#[derive(Debug, Clone, Copy, PartialEq, Eq)]
		pub enum #enum_ident {
			#(#variants_tokens),*
		}
	}
}

// see: https://stackoverflow.com/a/74360109/7368162
pub fn pretty_print(ts: &proc_macro2::TokenStream) -> String {
	let file = parse_file(&ts.to_string()).unwrap();
	unparse(&file)
}
