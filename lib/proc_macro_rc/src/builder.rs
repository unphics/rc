use proc_macro::TokenStream;
use syn::parse_macro_input;
use syn::spanned::Spanned;
use syn::AttributeArgs;
use syn::Item;
use quote::quote;


pub fn do_extend(st: &syn::DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let struct_name_literal = st.ident.to_string();
    let builder_name_literal = format!("{}Builder", struct_name_literal);
    let builder_name_ident = syn::Ident::new(&builder_name_literal, st.span());
    let struct_ident = st.ident.clone();
    let build_struct_field = gen_builder_struct_field_def(st)?;
    let builder_struct_factory_init_clauses = gen_builder_struct_factory_init_clauses(st)?;
    let ret = quote::quote! (
        pub struct #builder_name_ident {
            #build_struct_field
        }
        impl #struct_ident {
            pub fn builder() -> #builder_name_ident {
                #builder_name_ident {
                    #(#builder_struct_factory_init_clauses),*
                }
            }
        }
    );
    return Ok(ret);
}

type StructField = syn::punctuated::Punctuated<syn::Field, syn::Token![,]>;
fn get_fields_from_derive_input(st: &syn::DeriveInput) -> syn::Result<&StructField> {
    if let syn::Data::Struct(syn::DataStruct{
        fields:syn::Fields::Named(syn::FieldsNamed{
            ref named, ..
        }),
        ..
    }) = st.data {
        return Ok(named);
    };
    Err(syn::Error::new_spanned(st, "Must Define On Struct, Not On Enum"))
}

fn gen_builder_struct_field_def(st: &syn::DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let fields = get_fields_from_derive_input(st)?;
    let idents: Vec<_> = fields.iter().map(|f| &f.ident).collect();
    let types: Vec<_> = fields.iter().map(|f| &f.ty).collect();
    let ret = quote::quote!(
        #( #idents: std::option::Option<#types>),* // 星号表示要对前面的代码进行重复
    );
    return Ok(ret);
}

fn gen_builder_struct_factory_init_clauses(st: &syn::DeriveInput) ->syn::Result<Vec<proc_macro2::TokenStream>> {
    let fields = get_fields_from_derive_input(st)?;
    let init_cluase: Vec<_> = fields.iter().map(|f| {
        let ident = &f.ident;
        quote::quote!(
            #ident: std::option::Option::None
        )
    }).collect();
    return Ok(init_cluase);
}

fn gen_setter_functions(st: &syn::DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let fields = get_fields_from_derive_input(st)?;
    let idents: Vec<_> = fields.iter().map(|f| &f.ident).collect();
    let types: Vec<_> = fields.iter().map(|f| &f.ty).collect();
    let mut final_token_stream = proc_macro2::TokenStream::new();
    for (ident, type_) in idents.iter().zip(types.iter()) {
        let token_stream_piece = quote::quote!(
            fn #ident(&mut self, #ident: #type_) ->&mut Self {
                self.#ident = std::option::Option::Some(#ident);
            }
        );
        final_token_stream.extend(token_stream_piece);
    }
    Ok(final_token_stream)
}