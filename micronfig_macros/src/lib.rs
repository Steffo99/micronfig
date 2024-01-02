extern crate proc_macro;
use proc_macro::TokenStream;
use quote::{quote, quote_spanned};
use syn::parse::{Parse, ParseStream};
use syn::{Ident, parse_macro_input, Token, Type};
use syn::punctuated::{Pair, Punctuated};


type Config = Punctuated<Config, Token![,]>;

struct ConfigItem {
	identifier: Ident,
	types: ConfigTypes,
}

type ConfigTypes = Punctuated<Type, Conversion>;

enum Conversion {
	From,
	TryFrom,
	FromStr,
}


impl Parse for ConfigItem {
	fn parse(input: ParseStream) -> syn::Result<Self> {
		let identifier = input.parse::<Ident>()?;
		let types = input.parse::<Punctuated<Type, Conversion>>()?;
		Ok(Self { identifier, types })
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
	let input: Config = parse_macro_input!(input as Config);

	let cache = quote! {
		mod _cache {
			pub static lock: std::sync::OnceLock<Cache> = std::sync::OnceLock::new();
		}

		pub(self) fn _cache() {
			_cache::lock.get_or_init(micronfig::cache::Cache::new)
		}
	};

	let items = input.iter().map(|item: ConfigItem| {
		let identifier = item.identifier;

		// TODO: Can types be zero-length?
		let mut conversion_code = quote! {};
		let mut previous_conversion: Option<&Conversion> = None;
		for pair in item.types.pairs().into_iter() {
			let mut current_type: &Type = match pair {
				Pair::Punctuated(ty, _) => ty,
				Pair::End(ty) => ty,
			};
			let next_conversion: Option<&Conversion> = match pair {
				Pair::Punctuated(_, cv) => Some(cv),
				_ => None,
			};

			if let Some(previous_conversion) = previous_conversion {
				conversion_code = match previous_conversion {
					Conversion::From => quote! {
						#conversion_code
						let value: #current_type = value.into();
					}
					Conversion::TryFrom => quote! {
						#conversion_code
						let value: #current_type = value.try_into()
							.expect("to be able to convert {}", stringify!(#identifier));
					}
					Conversion::FromStr => quote! {
						#conversion_code
						let value: #current_type = value.parse()
							.expect("to be able to parse {}", stringify!(#identifier));
					}
				};
			}

			previous_conversion = next_conversion;
		};

		let last_type = item.types.last();

		quote! {
			mod #identifier {
				pub(super) lock: std::sync::OnceLock<#last_type> = std::sync::OnceLock::new();
			}

			pub(crate) fn #identifier() {
				#identifier::lock.get_or_init(|| {
					let key = stringify!(#identifier);
					let value = _cache().get(&key);

					#conversion_code

					value
				})
			}
		}
	});

	quote! {
		#cache
		#items
	}
}