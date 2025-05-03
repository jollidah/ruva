use proc_macro2::TokenStream;
use quote::quote;
use syn::{Attribute, DeriveInput, Error};

/// extract all the fields from the struct and type, then create a new instance of the struct with the associated function named construct
/// construct function will take all the fields as arguments and return the struct instance
/// with #[except] attribute, the field will be excluded from the construct function.
/// With the use of #[except] attribute, the struct must derive Default to be able to construct the struct
/// ```rust
/// #[derive(Default, TConstruct)]
/// struct TestStruct {
/// value: i32,
/// #[except]
/// name: String,
/// }
///
/// let test = TestStruct::construct(1);
/// assert_eq!(test.value, 1);
/// assert_eq!(test.name, String::default());
/// ```
pub fn expand_derive_construct(input: &mut DeriveInput) -> syn::Result<TokenStream> {
	let name = &input.ident;
	let generics = &input.generics;
	let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

	let fields = match &input.data {
		syn::Data::Struct(data) => match &data.fields {
			syn::Fields::Named(fields) => &fields.named,
			_ => return Err(Error::new_spanned(input, "Only Structs with named fields are supported")),
		},
		_ => return Err(Error::new_spanned(input, "Only Structs are supported")),
	};

	let mut input_arguments = Vec::new();
	let mut struct_fields_init = Vec::new();
	let mut has_except = false;

	for field in fields {
		let ident = field.ident.as_ref().unwrap();
		let ty = &field.ty;

		if !has_except_attribute(&field.attrs)? {
			input_arguments.push(quote!(#ident: #ty));
			struct_fields_init.push(quote!(#ident));
		} else {
			has_except = true;
		}
	}

	let construct_body = if has_except {
		quote! {
			Self {
				#(#struct_fields_init),*
				,..Default::default()
			}
		}
	} else {
		quote! {
			Self {
				#(#struct_fields_init),*
			}
		}
	};

	let construct_fn = quote! {
		impl #impl_generics #name #ty_generics #where_clause {
			pub fn construct(#(#input_arguments),*) -> Self {
				#construct_body
			}
		}
	};

	Ok(construct_fn)
}

/// Checks if a field has the #[except] attribute.
fn has_except_attribute(attrs: &[Attribute]) -> syn::Result<bool> {
	for attr in attrs {
		if attr.path().is_ident("except") {
			return Ok(true);
		}
	}
	Ok(false)
}
