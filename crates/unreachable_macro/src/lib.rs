use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemTrait, TraitItem, TraitItemFn};

#[proc_macro_attribute]
pub fn with_unreachable_defaults(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemTrait);

    let trait_name = &input.ident;
    let trait_vis = &input.vis;
    let trait_methods = input.items.iter().filter_map(|item| {
        if let TraitItem::Fn(TraitItemFn { sig, .. }) = item {
            let method_sig = &sig;
            Some(quote! {
                #method_sig {
                    unreachable!();
                }
            })
        } else {
            None
        }
    });

    let generated_trait = quote! {
        #trait_vis trait #trait_name {
            #(#trait_methods)*
        }
    };

    TokenStream::from(generated_trait)
}
