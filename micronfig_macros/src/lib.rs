extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{Ident, parse_macro_input, Token, Type};
use syn::punctuated::Punctuated;


type Config = Punctuated<ConfigItem, Token![,]>;

#[derive(Clone)]
struct ConfigItem {
	identifier: Ident,
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

		input.parse::<Token![:]>()?;
		input.parse::<Type>()?;

		let mut types = vec![];
		while let Ok(typ) = input.parse::<ConfigPair>() {
			types.push(typ)
		}

		Ok(Self { identifier, types })
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
			Err(input.error("Cannot determine conversion method to use; valid conversion tokens are `->` (From), `=>` (TryFrom) and `>` (FromStr)."))
		}
	}
}

#[proc_macro]
pub fn config(input: TokenStream) -> TokenStream {
	let input: Config = parse_macro_input!(input with syn::punctuated::Punctuated::parse_terminated);

	let cache_code = quote! {
		mod _cache {
			pub static lock: std::sync::OnceLock<micronfig::cache::Cache> = std::sync::OnceLock::new();
		}

		fn _cache() -> &'static micronfig::cache::Cache {
			_cache::lock.get_or_init(micronfig::cache::Cache::new)
		}
	};

	let items = input.iter().map(|item: &ConfigItem| {
		let identifier = &item.identifier;
		let identifier_string = identifier.to_string();

		let mut conversion_code = quote! {};
		for ConfigPair { r#type, conversion } in item.types.iter() {
			let typ = r#type;
			conversion_code = match conversion {
				Conversion::From => quote! {
					#conversion_code
					let value: Option<#typ> = value.map(Into::into);
				},
				Conversion::TryFrom => quote! {
					#conversion_code
					let value: Option<#typ> = value
						.map(|v| v.try_into())
						.map(|v| v.expect(&format!("to be able to convert {}", #identifier_string))
					);
				},
				Conversion::FromStr => quote! {
					#conversion_code
					let value: Option<#typ> = value
						.map(|v| v.parse())
						.map(|v| v.expect(&format!("to be able to parse {}", #identifier_string))
					);
				},
			};
		};

		let last_type = match item.types.last() {
			Some(pair) => {
				let typ = pair.r#type.clone();
				quote! { #typ }
			},
			None => {
				quote! { String }
			},
		};

		quote! {
			mod #identifier {
				pub(super) static lock: std::sync::OnceLock<Option<#last_type>> = std::sync::OnceLock::new();
			}

			pub(crate) fn #identifier() -> &'static Option<#last_type> {
				#identifier::lock.get_or_init(|| {
					let key: std::ffi::OsString = #identifier_string.into();
					let value: Option<String> = _cache().get(&key);

					#conversion_code

					value
				})
			}
		}
	});

	let mut items_code = quote! {};
	for code in items {
		items_code = quote! {
			#items_code
			#code
		};
	}

	let quote = quote! {
		#cache_code
		#items_code
	};

	println!("{quote}");

	quote.into()
}