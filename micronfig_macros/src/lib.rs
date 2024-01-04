extern crate proc_macro;
use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream};
use syn::{Ident, parse_macro_input, Token, Type, TypePath};
use syn::punctuated::Punctuated;


type Config = Punctuated<ConfigItem, Token![,]>;

#[derive(Clone)]
struct ConfigItem {
	identifier: Ident,
	optional: bool,
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


const VALID_INITIAL_TYPES: [&'static str; 2] = ["String", "std::string::String"];

impl Parse for ConfigItem {
	fn parse(input: ParseStream) -> syn::Result<Self> {
		let identifier = input.parse::<Ident>()?;

		let optional = input.lookahead1().peek(Token![?]);
		if optional {
			input.parse::<Token![?]>()
				.expect("this token to be parsed correctly, as it has been previously peeked");
		}

		let types = match input.lookahead1().peek(Token![:]) {
			true => {
				input.parse::<Token![:]>()
					.expect("this token to be parsed correctly, as it has been previously peeked");

				let string_type = input.parse::<TypePath>()?;
				if !VALID_INITIAL_TYPES.contains(&&*string_type.to_token_stream().to_string()) {
					return Err(syn::Error::new_spanned(string_type, "first type of a conversion chain should always be `String` or `std::string::String`, type aliases are not allowed"))
				}

				let mut types = Vec::new();
				while let Ok(typ) = input.parse::<ConfigPair>() {
					types.push(typ)
				}
				types
			},
			false => Vec::new(),
		};

		Ok(Self { identifier, optional, types })
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
	let input: Config = parse_macro_input!(input with syn::punctuated::Punctuated::parse_terminated);

	let cache_code = quote! {
		#[allow(non_snake_case)]
		mod _cache {
			pub static _lock: std::sync::OnceLock<micronfig::cache::Cache> = std::sync::OnceLock::new();
		}

		#[allow(non_snake_case)]
		fn _cache() -> &'static micronfig::cache::Cache {
			_cache::_lock.get_or_init(micronfig::cache::Cache::new)
		}
	};

	let items_code = input.iter().map(|item: &ConfigItem| {
		let identifier = &item.identifier;
		let identifier_string = identifier.to_string();

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
							.map(std::convert::Into::into);
					},
					(Conversion::TryFrom, true) => quote! {
						let value: Option<#typ> = value
							.map(std::convert::TryInto::try_into)
							.map(|v| v.expect(&format!("to be able to convert {}", #identifier_string))
						);
					},
					(Conversion::FromStr, true) => quote! {
						let value: Option<#typ> = value
							.map(str::parse)
							.map(|v| v.expect(&format!("to be able to parse {}", #identifier_string))
						);
					},
					(Conversion::From, false) => quote! {
						let value: #typ = value
							.into();
					},
					(Conversion::TryFrom, false) => quote! {
						let value: #typ = value
							.try_into()
							.expect(&format!("to be able to convert {}", #identifier_string));
					},
					(Conversion::FromStr, false) => quote! {
						let value: #typ = value
							.parse()
							.expect(&format!("to be able to parse {}", #identifier_string));
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
					.expect(&format!("to be have {} set", #identifier_string));
			},
		};

		quote! {
			#[allow(non_snake_case)]
			mod #identifier {
				pub(super) static _lock: std::sync::OnceLock<#type_final_option> = std::sync::OnceLock::new();
			}

			#[allow(non_snake_case)]
			pub(crate) fn #identifier() -> &'static #type_final_option {
				#identifier::_lock.get_or_init(|| {
					let key: std::ffi::OsString = #identifier_string.into();
					let value: Option<String> = _cache().get(&key);

					#require_code
					#conversion_code

					value
				})
			}
		}
	}).reduce(|acc, new| {
		quote! { #acc #new }
	});

	let quote = quote! {
		#cache_code
		#items_code
	};

	quote.into()
}