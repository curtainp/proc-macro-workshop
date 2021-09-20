use proc_macro::TokenStream;
use syn;
use syn::spanned::Spanned;
use quote::quote;

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let st = syn::parse_macro_input!(input as syn::DeriveInput);

    match do_expand(&st) {
        Ok(token) => token.into(),
        Err(e) => e.to_compile_error().into(),
    }
}


fn get_fields_from_derive_input(st: &syn::DeriveInput) -> syn::Result<&syn::Fields> {
    // 不用解析到这么深的层次--因为syn::Fields的iter()方法底层会自己解析并调用对应得syn::FieldsNamed的iter()
    // if let syn::Data::Struct(syn::DataStruct{
    //     fields: syn::Fields::Named(
    //         syn::FieldsNamed{ ref named,
    //         ..}),
    //     .. }) = st.data {
    //     return Ok(named)
    // };

    match st.data {
        syn::Data::Struct(ref data) => return Ok(&data.fields),
        _ => {
            return Err(syn::Error::new_spanned(st, "Must define struct".to_string()))
        }
    };
}


fn do_expand(st: &syn::DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    // eprint!("{:#?}", st.data);
    let builder_name_literal = format!("{}Builder", st.ident.to_string());
    let builder_name_ident = syn::Ident::new(&builder_name_literal, st.span());

    // 在模版代码中不能使用`.`语法获取ident
    let struct_ident = &st.ident;
    let fields = get_fields_from_derive_input(st)?;

    let builder_fields = fields.iter().map(|f| {
        let ident = &f.ident;
        let ty = &f.ty;

        quote! {
            #ident: std::option::Option<#ty>,
        }
    });

    let builder_init = fields.iter().map(|f| {
        let ident = &f.ident;

        quote! {
            #ident: std::option::Option::None,
        }
    });

    let builder_setter = fields.iter().map(|f| {
        let ident = &f.ident;
        let ty = &f.ty;

        quote! {
            fn #ident(&mut self, #ident: #ty) -> &mut Self {
                self.#ident = std::option::Option::Some(#ident);
                self
            }
        }
    });

    let builder_build = fields.iter().map(|f| {
        let ident = &f.ident;

        quote! {
            #ident: self.#ident.as_ref().ok_or("missing".to_string())?.clone(),
        }
    });

    let ret = quote!{
        pub struct #builder_name_ident {
            #(#builder_fields)*
        }

        impl #builder_name_ident {
            #(#builder_setter)*

            pub fn build(&mut self) -> std::result::Result<#struct_ident, std::boxed::Box<dyn std::error::Error>> {
                std::result::Result::<_,_>::Ok(#struct_ident{
                    #(#builder_build)*
                })
            }
        }

        impl #struct_ident {
            pub fn builder() -> #builder_name_ident {
                #builder_name_ident {
                    #(#builder_init)*
                }
            }
        }
    };

    Ok(ret)
}
