extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{
    braced,
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
    token, Attribute, DeriveInput, Ident, Result, Token,
};

#[allow(dead_code)]
#[derive(Debug, Clone)]
struct OffsetPadStruct {
    vis: syn::Visibility,
    keyword: syn::token::Struct,
    ident: syn::Ident,
    brace_token: token::Brace,
    fields: Punctuated<CustomField, Token![,]>,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
struct CustomField {
    attrs: Vec<Attribute>,
    offset: Option<PossibleOffsetType>,
    vis: syn::Visibility,
    ident: syn::Ident,
    colon_token: Token![:],
    ty: syn::Type,
}

#[derive(Debug, Clone)]
enum PossibleOffsetType {
    /// when a var is used
    Ident(syn::Ident),
    /// when a literal is used (number) 0x10
    Lit(syn::Lit),
}

impl Parse for PossibleOffsetType {
    fn parse(input: ParseStream) -> Result<Self> {
        if let Ok(ident) = input.parse() {
            return Ok(PossibleOffsetType::Ident(ident));
        }

        if let Ok(lit) = input.parse() {
            return Ok(PossibleOffsetType::Lit(lit));
        }

        Err(input.error("expected identifier or literal"))
    }
}

impl ToTokens for PossibleOffsetType {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            PossibleOffsetType::Ident(ident) => ident.to_tokens(tokens),
            PossibleOffsetType::Lit(lit) => lit.to_tokens(tokens),
        }
    }
}

impl Parse for CustomField {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(CustomField {
            attrs: input.call(Attribute::parse_outer)?,
            offset: input.parse().ok(),
            vis: input.parse()?,
            ident: input.parse()?,
            colon_token: input.parse()?,
            ty: input.parse()?,
        })
    }
}

impl Parse for OffsetPadStruct {
    fn parse(input: ParseStream) -> Result<Self> {
        #[allow(unused_assignments)]
        let content;
        Ok(OffsetPadStruct {
            vis: input.parse()?,
            keyword: input.parse()?,
            ident: input.parse()?,
            brace_token: braced!(content in input),
            fields: content.parse_terminated(CustomField::parse, Token![,])?,
        })
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
struct CustomStruct {
    attrs: Vec<Attribute>,
    struct_def: OffsetPadStruct,
}

impl Parse for CustomStruct {
    fn parse(input: ParseStream) -> Result<Self> {
        #[allow(unused_assignments)]
        Ok(CustomStruct {
            attrs: input.call(Attribute::parse_outer)?,
            struct_def: input.parse()?,
        })
    }
}
/// creates a struct with padding to align the fields to the offsets provided  
/// this does not use the `#[repr(packed)]`` attribute, so the struct will be aligned to the offsets provided  
/// which means no need to do let binding when accessing fields  
///
/// # Details
/// - struct is repr(C)
/// - struct has debug impl without pads
///
/// # Format
/// - normal struct definition with fields
/// - the fields must be in order of the offsets
/// - the offset must be provided in the format `0x10` or `0x10u32` or `0x10usize` or ident like variable name
/// - the offset must be provided before the field
///
/// # Example
/// ```rust
/// const Component: usize = 0x38; // Component*
/// struct_pad_aligned! {
///    pub struct Entity {
///       0x10 pub node: usize,
///       0x20 pub health: i32,
///       pub team: u8,
///       Component pub component: usize,
///    }
/// }
/// ````
///
/// # Expanded
///
/// ```rust
/// #[derive(Debug, Clone, Copy)]
/// #[repr(C)]
/// pub struct Entity {
///    _pad0: [u8; 0x10],
///   pub node: usize,
///   _pad1: [u8; 0x20 - (0x10 + std::mem::size_of::<usize>())],
///   pub health: i32,
///   pub team: u8,
///   _pad2: [u8; Component (0x20 + std::mem::size_of::<i32>())],
///   pub component: usize,
/// }
///
/// ````
#[proc_macro]
pub fn struct_pad_aligned(input: TokenStream) -> TokenStream {
    let parsed = parse_macro_input!(input as CustomStruct);
    let fields = parsed
        .struct_def
        .fields
        .iter()
        .enumerate()
        .map(field_mapper(parsed.clone()));
    let ident = parsed.struct_def.ident;
    let vis = parsed.struct_def.vis;

    // generate attributes for the struct
    let attrs = parsed.attrs.iter().map(|attr| quote! { #attr });

    TokenStream::from(quote! {
        #(#attrs)*
        #[repr(C)]
        #vis struct #ident {
            #(#fields)*
        }
    })
}

#[proc_macro]
pub fn struct_pad_packed(input: TokenStream) -> TokenStream {
    let parsed = parse_macro_input!(input as CustomStruct);
    let fields = parsed
        .struct_def
        .fields
        .iter()
        .enumerate()
        .map(field_mapper(parsed.clone()));
    let ident = parsed.struct_def.ident;
    let vis = parsed.struct_def.vis;

    // generate attributes for the struct
    let attrs = parsed.attrs.iter().map(|attr| quote! { #attr });

    TokenStream::from(quote! {
        #(#attrs)*
        #[repr(C, packed)]
        #vis struct #ident {
            #(#fields)*
        }
    })
}

fn field_mapper(
    parsed: CustomStruct,
) -> impl FnMut((usize, &CustomField)) -> proc_macro2::TokenStream {
    return move |(i, field)| {
        let field_name = field.ident.clone();
        let field_type = field.ty.clone();
        let field_annotation = field.attrs.iter().map(|attr| quote! { #attr });
        let vis = &field.vis;
        if let Some(offset) = &field.offset {
            if i == 0 {
                return quote! {
                    _pad0: [u8; #offset],
                    #(#field_annotation)*
                    #vis #field_name: #field_type,
                };
            }

            let prior_field = &parsed.struct_def.fields[i - 1];
            let prior_offset = &prior_field.offset;
            let prior_type = &prior_field.ty;
            let padname = Ident::new(&format!("_pad{}", i), prior_field.ident.span());

            // if the pior offset is none we need to traverse back to find the latest offset and gather
            // all the types without an offset along the way
            if prior_offset.is_none() {
                let mut last_offset = None;
                let mut last_offset_type = None;
                let mut types_until_last_offset = Vec::new();
                for j in (0..i).rev() {
                    let field = &parsed.struct_def.fields[j];
                    if field.offset.is_some() {
                        last_offset = field.offset.clone();
                        last_offset_type = Some(field.ty.clone());
                        break;
                    } else {
                        types_until_last_offset.push(&field.ty);
                    }
                }

                if types_until_last_offset.is_empty() {
                    panic!("No fields with offsets found before field {}", i);
                }

                let types_to_subtract = types_until_last_offset
                    .iter()
                    .map(|ty| quote! { std::mem::size_of::<#ty>() });
                // if we found a last offset, we can calculate the size of the pad
                if let (Some(last_offset), Some(last_offset_type)) = (last_offset, last_offset_type)
                {
                    let pad_info = quote! {
                        #offset - (#last_offset + std::mem::size_of::<#last_offset_type>()) - #(#types_to_subtract)-*
                    };
                    let field = quote! {
                        #padname: [u8; #pad_info],
                        #(#field_annotation)*
                        #vis #field_name: #field_type,
                    };
                    return field;
                }

                let pad_info = quote! {
                    #offset - #(#types_to_subtract)-*
                };
                let field = quote! {
                    #padname: [u8; #pad_info],
                    #(#field_annotation)*
                    #vis #field_name: #field_type,
                };
                return field;
            }

            return quote! {
                #padname: [u8; #offset - (#prior_offset + std::mem::size_of::<#prior_type>())],
                #(#field_annotation)*
                #vis #field_name: #field_type,
            };
        }

        quote! {
            #vis #field_name: #field_type,
        }
    };
}

#[proc_macro_derive(DebugPadLess)]
pub fn derive_debug_pad_less(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let ident = input.ident;
    let fields = match &input.data {
        syn::Data::Struct(data) => match &data.fields {
            syn::Fields::Named(fields) => fields
                .named
                .iter()
                .filter(|e| {
                    !e.ident
                        .clone()
                        .expect("named field")
                        .to_string()
                        .contains("_pad")
                })
                .map(|field| {
                    let field_name = field.ident.clone().unwrap();
                    quote! {
                        .field(stringify!(#field_name), &self.#field_name)
                    }
                }),
            syn::Fields::Unnamed(_) => panic!("Unnamed fields not supported"),
            syn::Fields::Unit => panic!("Unit fields not supported"),
        },
        syn::Data::Enum(_) => panic!("Enum not supported"),
        syn::Data::Union(_) => panic!("Union not supported"),
    };

    TokenStream::from(quote! {
        impl core::fmt::Debug for #ident {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.debug_struct(stringify!(#ident))
                #(#fields)*
                 .finish()
            }
        }
    })
}
