use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

/// Derive macro for Extractable trait
/// 
/// **NOTE**: This is a stub implementation demonstrating how the macro would work.
/// For production use, this would need to be expanded with better error handling
/// and support for generic types.
///
/// Generates code to extract renderable primitives from a struct by walking
/// its fields and checking for the Renderable trait.
/// 
/// # Required Imports
///
/// The generated code requires these traits to be in scope:
/// - `Extractable` - The main trait being implemented
/// - `ExtractHelper` - Helper trait for extraction logic  
/// - `Primitive` - The primitive type being extracted
///
/// ```ignore
/// use game_engine::{Extractable, ExtractHelper, Primitive};
/// ```
///
/// # Usage
/// 
/// ```ignore
/// use game_engine::{Extractable, ExtractHelper, Primitive};
///
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
                            
                            // NOTE: This assumes ExtractHelper is in scope
                            // Production version could use a configurable path
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
    
    // NOTE: This assumes Primitive and ExtractHelper are in scope
    // A production version could accept paths as macro attributes:
    // #[derive(Extractable)]
    // #[extractable(primitive = "my_crate::Primitive")]
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


