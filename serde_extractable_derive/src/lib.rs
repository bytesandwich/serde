use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields, Type};

/// Derive macro for Extractable trait
/// 
/// Generates code to extract renderable primitives from a struct by walking
/// its fields and checking for the Renderable trait.
/// 
/// # Usage
/// 
/// ```ignore
/// #[derive(Extractable)]
/// pub struct SoccerGame {
///     #[extract]
///     pub field: Field,
///     #[extract]
///     pub ball: Ball,
///     #[extract]
///     pub players: Vec<Player>,
///     pub score: [u32; 2], // Not extracted
/// }
/// ```
#[proc_macro_derive(Extractable, attributes(extract))]
pub fn derive_extractable(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    
    let name = &input.ident;
    let generics = &input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    
    // Extract fields that have #[extract] attribute
    let extract_fields = match &input.data {
        Data::Struct(data) => {
            match &data.fields {
                Fields::Named(fields) => {
                    fields.named.iter().filter_map(|field| {
                        // Check if field has #[extract] attribute
                        let has_extract = field.attrs.iter().any(|attr| {
                            attr.path().is_ident("extract")
                        });
                        
                        if has_extract {
                            let field_name = &field.ident;
                            let field_name_str = field_name.as_ref().unwrap().to_string();
                            let field_type = &field.ty;
                            
                            // Generate extraction code based on field type
                            let extract_code = generate_extract_code(field_name_str, field_type);
                            
                            Some(quote! {
                                ExtractHelper::extract(&self.#field_name, #field_name_str, "", &mut __result);
                            })
                        } else {
                            None
                        }
                    }).collect::<Vec<_>>()
                }
                _ => {
                    return syn::Error::new_spanned(
                        &input,
                        "Extractable can only be derived for structs with named fields"
                    ).to_compile_error().into();
                }
            }
        }
        _ => {
            return syn::Error::new_spanned(
                &input,
                "Extractable can only be derived for structs"
            ).to_compile_error().into();
        }
    };
    
    let expanded = quote! {
        impl #impl_generics Extractable for #name #ty_generics #where_clause {
            fn extract_renderables(&self) -> ::std::collections::HashMap<String, Primitive> {
                let mut __result = ::std::collections::HashMap::new();
                
                #(#extract_fields)*
                
                __result
            }
        }
    };
    
    TokenStream::from(expanded)
}

fn generate_extract_code(field_name: String, field_type: &Type) -> proc_macro2::TokenStream {
    // For simplicity, we assume the user provides ExtractHelper trait
    // that knows how to extract different types
    quote! {
        ExtractHelper::extract(&self.#field_name, #field_name, "", &mut __result);
    }
}
