use proc_macro::TokenStream;
use proc_macro_error::{abort, proc_macro_error};
use quote::quote;

fn get_resolver(meta: &syn::Meta) -> syn::Expr {
    use syn::Lit::*;
    use syn::Meta::NameValue;
    
    if let NameValue(m) = meta {
	if let Str(s) = &m.lit {
	    return syn::parse_str(&s.value()).unwrap_or_else(|_| abort!(&m.lit, "Bad expression"))
	}
    }
    
    abort!(meta, "Resolver must be a string containing an expression")
}

fn collect_fn_info(func: &syn::ItemFn) -> FnInfo {
    let mut named_args = vec![];

    let args = func.sig.inputs.iter();
    for arg in args {
	if let syn::FnArg::Typed(t) = arg {
	    if let syn::Pat::Ident(syn::PatIdent { ident, .. }) = &*t.pat {
		named_args.push((ident.clone(), *t.ty.clone()));
	    } else {
		abort!(t, "Only named arguments are supported.")
	    }
	} else {
	    abort!(arg, "Functions with receivers are not supported.")
	}
    }

    FnInfo {
	args: named_args,
	ret: if let syn::ReturnType::Type(_, t) = &func.sig.output {
	    Some(*t.clone())
	} else {
	    None
	}
    }
}

fn build_fn_sig(info: &FnInfo) -> proc_macro2::TokenStream {
    let tys = info.args.iter().map(|(_, t)| t);
    let ret = if let Some(t) = &info.ret {
	quote! {-> #t}
    } else {
	quote! {-> ()}
    };
    quote!{
	&'static fn (#(#tys),*) #ret
    }
}

struct FnInfo {
    args: Vec<(syn::Ident, syn::Type)>,
    ret: Option<syn::Type>
}

#[proc_macro_attribute]
#[proc_macro_error]
pub fn indirect(attr: TokenStream, item : TokenStream) -> TokenStream {
    use syn::{AttributeArgs, NestedMeta, ItemFn};
    use syn::parse_macro_input;
    
    let args = parse_macro_input!(attr as AttributeArgs);
    let mut item = parse_macro_input!(item as ItemFn);

    let mut resolver = None;

    for arg in &args {
	if let NestedMeta::Meta(m) = arg {
	    match m.path() {
		p if p.is_ident("resolver") => {
		    resolver = Some(get_resolver(m));
		},
		_ => abort!(arg, "Unknown argument, expected 'resolver'")
	    }
	}
    }

    let info = collect_fn_info(&item);
    let ptr_sig = build_fn_sig(&info);
    let new_args = info.args.iter().map(|(a, _)| a);

    if let Some(resolver) = resolver {
	item.block = Box::new(syn::parse((quote!{{
	    use std::sync::Once;
	    
	    static mut IMPL: Option<#ptr_sig> = None;
	    static INIT: Once = Once::new();

	    unsafe {
		INIT.call_once(|| {
		    IMPL = Some(#resolver());
		});
		(IMPL.unwrap())(#(#new_args),*)
	    }
	}}).into()).unwrap());
    }

    use quote::ToTokens;
    item.to_token_stream().into()
}
