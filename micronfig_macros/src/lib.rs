extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{Ident, parse_macro_input, Token, Type, TypePath};
use syn::punctuated::Punctuated;

type Config = Punctuated<ConfigItem, Token![,]>;

#[derive(Clone)]
struct ConfigItem {
	identifier: Ident,
	optional: bool,
	first: TypePath,
	types: Vec<ConfigPair>,
}

#[derive(Clone)]
struct ConfigPair {
	conversion: Conversion,
	r#type: Type,
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
enum Conversion {
	From,
	TryFrom,
	FromStr,
}


impl Parse for ConfigItem {
	fn parse(input: ParseStream) -> syn::Result<Self> {
		let identifier = input.parse::<Ident>()?;

		let optional = input.lookahead1().peek(Token![?]);
		if optional {
			input.parse::<Token![?]>()
				.expect("Expected `?`, as it was previously peeked");
		}

		let (first, types) = match input.lookahead1().peek(Token![:]) {
			true => {
				input.parse::<Token![:]>()
					.expect("Expected `:`, as it was previously peeked");

				let first = input.parse::<TypePath>()?;

				let mut types = Vec::new();
				while let Ok(typ) = input.parse::<ConfigPair>() {
					types.push(typ)
				}
				
				(first, types)
			},
			false => {
				let first = syn::parse_quote!(String);
				let types = Vec::new();
				
				(first, types)
			},
		};

		Ok(Self { identifier, optional, first, types })
	}
}

impl Parse for ConfigPair {
	fn parse(input: ParseStream) -> syn::Result<Self> {
		let conversion = input.parse::<Conversion>()?;
		let r#type = input.parse::<Type>()?;

		Ok(Self { conversion, r#type })
	}
}

impl Parse for Conversion {
	fn parse(input: ParseStream) -> syn::Result<Self> {
		if input.parse::<Token![->]>().is_ok() {
			Ok(Conversion::From)
		}
		else if input.parse::<Token![=>]>().is_ok() {
			Ok(Conversion::TryFrom)
		}
		else if input.parse::<Token![>]>().is_ok() {
			Ok(Conversion::FromStr)
		}
		else {
			Err(input.error("cannot determine conversion method to use; valid conversion tokens are `->` (From), `=>` (TryFrom) and `>` (FromStr)."))
		}
	}
}

#[proc_macro]
pub fn config(input: TokenStream) -> TokenStream {
	let input: Config = parse_macro_input!(input with Punctuated::parse_terminated);

	let cache_code = quote! {
		#[allow(non_snake_case)]
		fn _cache() -> &'static micronfig::cache::Cache {
			static LOCK: std::sync::OnceLock<micronfig::cache::Cache> = std::sync::OnceLock::new();
			
			LOCK.get_or_init(micronfig::cache::Cache::new)
		}
	};

	let items_code = input.iter().map(|item: &ConfigItem| {
		let identifier = &item.identifier;
		let identifier_string = identifier.to_string();
		
		let type_first = &item.first;

		let type_final = match item.types.last() {
			Some(pair) => {
				let typ = pair.r#type.clone();
				quote! { #typ }
			},
			None => {
				quote! { std::string::String }
			},
		};
		let type_final_option = match item.optional {
			true => quote! { std::option::Option<#type_final> },
			false => quote! { #type_final },
		};

		let conversion_code = item.types.iter().map(
			|ConfigPair { r#type, conversion }| {
				let typ = r#type;
				match (conversion, item.optional) {
					(Conversion::From, true) => quote! {
						let value: Option<#typ> = value
							.map(|v| v.into());
					},
					(Conversion::TryFrom, true) => quote! {
						let value: Option<#typ> = value
							.map(|v| v
								.try_into()
								.unwrap_or_else(|err| panic!("{}: Couldn't perform `=> {:?}` conversion: {:#?}", #identifier_string, std::any::type_name::<#typ>(), err))
							);
					},
					(Conversion::FromStr, true) => quote! {
						let value: Option<#typ> = value
							.map(|v| v
								.parse()
								.unwrap_or_else(|err| panic!("{}: Couldn't perform `> {:?}` conversion: {:#?}", #identifier_string, std::any::type_name::<#typ>(), err))
							);
					},
					(Conversion::From, false) => quote! {
						let value: #typ = value
							.into();
					},
					(Conversion::TryFrom, false) => quote! {
						let value: #typ = value
							.try_into()
							.unwrap_or_else(|err| panic!("{}: Couldn't perform `=> {:?}` conversion: {:#?}", #identifier_string, std::any::type_name::<#typ>(), err));
					},
					(Conversion::FromStr, false) => quote! {
						let value: #typ = value
							.parse()
							.unwrap_or_else(|err| panic!("{}: Couldn't perform `> {:?}` conversion: {:#?}", #identifier_string, std::any::type_name::<#typ>(), err));
					},
				}
			}
		).reduce(|acc, new| {
			quote! { #acc #new }
		});

		let require_code = match item.optional {
			true => quote! {},
			false => quote! {
				let value: String = value
					.unwrap_or_else(|| panic!("{}: Is required, but has no value set", #identifier_string));
			},
		};

		quote! {
			#[allow(non_snake_case)]
			pub(crate) fn #identifier() -> &'static #type_final_option {
				static LOCK: std::sync::OnceLock<#type_final_option> = std::sync::OnceLock::new();
				
				LOCK.get_or_init(|| {
					let key = #identifier_string.as_ref();
					let value: Option<#type_first> = _cache().get(key);

					#require_code
					#conversion_code

					value
				})
			}
		}
	}).reduce(|acc, new| {
		quote! {
			#acc
			#new
		}
	});

	let quote = quote! {
		#cache_code
		#items_code
	};

	quote.into()
}