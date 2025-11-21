use doppleganger_macros_parse::{AdtDecl, Cons, EndOfStream, Struct};
use proc_macro2::TokenStream;
use unsynn::*;

#[proc_macro_derive(Doppleganger, attributes(dg))]
pub fn macros(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    dg_macros(input.into()).into()
}

fn dg_macros(input: TokenStream) -> TokenStream {
    let mut i = input.to_token_iter();

    // Parse as TypeDecl
    match i.parse::<Cons<AdtDecl, EndOfStream>>() {
        Ok(it) => match it.first {
            AdtDecl::Struct(parsed) => process_struct(parsed),
            AdtDecl::Enum(_parsed) => todo!("Not yet implemented"),
        },
        Err(err) => {
            panic!("Could not parse type declaration: {err}");
        }
    }
}

fn process_struct(s: Struct) -> TokenStream {
    use doppleganger_macros_parse::ToTokens;
    use quote::quote;

    // Find the dg attribute with direction
    let mut direction = None;
    for attr in &s.attributes {
        if let doppleganger_macros_parse::AttributeInner::Dg(dg_attr) = &attr.body.content {
            for item in dg_attr.inner.content.iter() {
                if let doppleganger_macros_parse::DgInner::Direction(dir) = &item.value {
                    direction = Some(dir);
                    break;
                }
            }
        }
        if direction.is_some() {
            break;
        }
    }

    let Some(direction) = direction else {
        panic!("Missing #[dg(forward = ...)] or #[dg(backward = ...)] attribute on struct");
    };

    let struct_name = &s.name;

    // Extract generic params for the impl block as TokenStream
    let generic_params_ts = if let Some(generics) = &s.generics {
        let params_ts = generics.params.to_token_stream();
        quote! { < #params_ts > }
    } else {
        quote! {}
    };

    // Extract just the names for use in the type
    let generic_names_ts = if let Some(generics) = &s.generics {
        let names_ts: Vec<TokenStream> = generics
            .params
            .iter()
            .map(|p| match &p.value {
                doppleganger_macros_parse::GenericParam::Lifetime { name, .. } => {
                    name.to_token_stream()
                }
                doppleganger_macros_parse::GenericParam::Type { name, .. } => {
                    name.to_token_stream()
                }
                doppleganger_macros_parse::GenericParam::Const { name, .. } => {
                    name.to_token_stream()
                }
            })
            .collect();
        quote! { < #(#names_ts),* > }
    } else {
        quote! {}
    };

    // Get the fields from the struct
    let fields = match &s.kind {
        doppleganger_macros_parse::StructKind::Struct { fields, .. } => fields,
        doppleganger_macros_parse::StructKind::TupleStruct { .. } => {
            panic!("Tuple structs not yet supported");
        }
        doppleganger_macros_parse::StructKind::UnitStruct { .. } => {
            panic!("Unit structs not yet supported");
        }
    };

    // Generate field transformations
    let field_transforms: Vec<_> = fields
        .content
        .iter()
        .map(|field| {
            let field_name = &field.value.name;
            let field_type = &field.value.typ;
            let field_type_ts = field_type.to_token_stream();
            quote! {
                #field_name: <#field_type_ts as ::doppleganger::Mirror>::mirror(source.#field_name)
            }
        })
        .collect();

    match direction {
        doppleganger_macros_parse::DgDirection::Forward { path, .. } => {
            // For forward: Source = Self, Dest = OtherType
            let path_ts = path.to_token_stream();
            quote! {
                impl #generic_params_ts ::doppleganger::Mirror for #struct_name #generic_names_ts {
                    type Source = Self;
                    type Dest = #path_ts;

                    fn mirror(source: Self::Source) -> Self::Dest {
                        Self::Dest {
                            #(#field_transforms),*
                        }
                    }
                }
            }
        }
        doppleganger_macros_parse::DgDirection::Backward { path, .. } => {
            // For backward: Source = OtherType, Dest = Self
            let path_ts = path.to_token_stream();
            quote! {
                impl #generic_params_ts ::doppleganger::Mirror for #struct_name #generic_names_ts {
                    type Source = #path_ts;
                    type Dest = Self;

                    fn mirror(source: Self::Source) -> Self::Dest {
                        Self {
                            #(#field_transforms),*
                        }
                    }
                }
            }
        }
    }
}
