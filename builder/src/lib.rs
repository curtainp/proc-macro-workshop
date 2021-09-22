use proc_macro::TokenStream;
use syn;
use syn::spanned::Spanned;
use quote::quote;
use std::option::Option::Some;

#[proc_macro_derive(Builder, attributes(builder))]
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
            return Err(syn::Error::new_spanned(st, "Must define struct".to_string()));
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

        if get_option_type(ty).is_some() || get_fields_attribute_ident(f).is_some() {
            quote! {
                #ident: #ty,
            }
        } else {
            quote! {
                #ident: std::option::Option<#ty>,
            }
        }
    });

    let builder_init = fields.iter().map(|f| {
        let ident = &f.ident;

        if get_fields_attribute_ident(f).is_some() {
            quote! {
                #ident: std::vec::Vec::new(),
            }
        } else {
            quote! {
                #ident: std::option::Option::None,
            }
        }
    });

    let builder_setter = fields.iter().map(|f| {
        let ident = &f.ident;
        let ty = &f.ty;

        if let Some(gty) = get_option_type(ty) {
            quote! {
                fn #ident(&mut self, #ident: #gty) -> &mut Self {
                    self.#ident = std::option::Option::Some(#ident);
                    self
                }
            }
        } else if let Some(attr_ident) = get_fields_attribute_ident(f) {
            let inner_ty = get_generic_type(ty, "Vec").expect("each attribute must be specified with Vec field");
            let mut tmp = quote! {
                fn #attr_ident(&mut self, #attr_ident: #inner_ty) -> &mut Self {
                    self.#ident.push(#attr_ident);
                    self
                }
            };
            if attr_ident.to_string() != ident.as_ref().unwrap().to_string() {
                tmp.extend(
                    quote! {
                        fn #ident(&mut self, #ident: #ty) -> &mut Self {
                            self.#ident = #ident.clone();
                            self
                        }
                    }
                );
            }
            return tmp;
        } else {
            quote! {
                fn #ident(&mut self, #ident: #ty) -> &mut Self {
                    self.#ident = std::option::Option::Some(#ident);
                    self
                }
            }
        }
    });

    let builder_build = fields.iter().map(|f| {
        let ident = &f.ident;
        let ty = &f.ty;

        // 原始类型字段为Option<T>类型的 直接clone
        if get_option_type(ty).is_some() || get_fields_attribute_ident(f).is_some() {
            quote! {
                #ident: self.#ident.clone(),
            }
        } else {
            // 否则，获取类型Builder的内部类型再clone
            quote! {
                #ident: self.#ident.as_ref().ok_or("missing".to_string())?.clone(),
            }
        }
    });

    let ret = quote! {
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

fn get_generic_type<'a>(ty: &'a syn::Type, pattern: &'static str) -> Option<&'a syn::Type> {
    match ty {
        // 支持相对路径和绝对路径 std::option::Option<T> Option<T> ::std::option::Option<T>
        syn::Type::Path(path) => {
            if path.qself.is_some() {
                return None;
            }

            // 判断路径上最后一个元素-即类型T紧挨的类型
            // 用于支持绝对路径
            let path_seg = path.path.segments.last().unwrap();
            if path_seg.ident.to_string() != pattern {
                return None;
            }

            // 获取<>中的类型参数
            let ab = match path_seg.arguments {
                syn::PathArguments::AngleBracketed(ref ab) => ab,
                _ => return None,
            };

            if ab.args.len() != 1 {
                return None;
            }

            match ab.args.first().unwrap() {
                syn::GenericArgument::Type(gty) => Some(gty),
                syn::GenericArgument::Const(_)
                | syn::GenericArgument::Binding(_)
                | syn::GenericArgument::Lifetime(_)
                | syn::GenericArgument::Constraint(_) => None,
            }
        }
        _ => None,
    }
}

fn get_option_type(ty: &syn::Type) -> Option<&syn::Type> {
    return get_generic_type(ty, "Option");
}

fn get_fields_attribute_ident(field: &syn::Field) -> Option<syn::Ident> {
    for attr in &field.attrs {
        if let Ok(syn::Meta::List(outer_meta)) = attr.parse_meta() {
            if outer_meta.path.segments.first().unwrap().ident == "builder" {
                if let syn::NestedMeta::Meta(syn::Meta::NameValue(meta_kv)) =
                    outer_meta.nested.first().unwrap()
                {
                    if meta_kv.path.segments.first().unwrap().ident == "each" {
                        if let syn::Lit::Str(ref lit) = meta_kv.lit {
                            return Some(syn::Ident::new(
                                lit.value().as_str(),
                                attr.span(),
                            ));
                        }
                    }
                }
            }
        }
    }
    None
}