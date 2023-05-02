use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use syn::{ItemTrait, ItemFn, TraitItemFn};
use quote::quote;

#[proc_macro_attribute]
pub fn export(_: TokenStream, code: TokenStream) -> TokenStream {
    let ast: ItemTrait = match syn::parse(code) {
        Ok(v) => v,
        Err(e) => return e.to_compile_error().into(),
    };
    let name = ast.ident.to_string();
    let snake_case_name: String = name.chars()
        .map(|c| if c.is_uppercase() {
            format!("_{}", c.to_lowercase())
        }
        else {
            format!("{c}")
        })
        .collect();
    let export_macro_name = Ident::new(&format!("export{snake_case_name}s"), Span::call_site());
    quote! {
        #[macro_export]
        macro_rules! #export_macro_name {
            () => {
                println!("hi");
            };
        }
    }.into()
}

fn vtable(ast: ItemTrait) -> TokenStream {
    let name = ast.ident.to_string();
    let vtable_name = format!("{name}VTable");
    let functions: Result<Vec<_>, _> = ast.items
        .into_iter()
        .filter_map(|item| {
            match item {
                syn::TraitItem::Const(_) => Some(item),
                syn::TraitItem::Fn(_) => Some(item),
                syn::TraitItem::Type(_) => Some(item),
                syn::TraitItem::Macro(_) => Some(item),
                _ => None,
            }
        })
        .map(|item| {
            match item {
                syn::TraitItem::Const(_) => Err(quote!(compile_error!("Consts are not allowed in exported traits."))),
                syn::TraitItem::Fn(f) => Ok(f),
                syn::TraitItem::Type(_) => Err(quote!(compile_error!("Types are not allowed in exported traits."))),
                syn::TraitItem::Macro(_) => Err(quote!(compile_error!("Macro invocations are not allowed in exported traits."))),
                _ => unreachable!(),
            }
        })
        .collect();
    match functions {
        Ok(functions) => vtable_functions(functions, vtable_name),
        Err(e) => e.into(),
    }
}

fn vtable_functions(functions: Vec<TraitItemFn>, name: String) -> TokenStream  {
    let (names, types): (Vec<_>, Vec<_>) = functions.into_iter()
        .map(|f| {
            let sig = f.sig;
            let ident = sig.ident;
            let t = sig.inputs.into_iter()
                .map(|arg| {
                    match arg {
                        syn::FnArg::Receiver(r) => r.ty,
                        syn::FnArg::Typed(r) => r.ty,
                    } 
                });
            (ident, t)
        })
        .unzip();
    quote! {
        struct 
    }.into()
}
