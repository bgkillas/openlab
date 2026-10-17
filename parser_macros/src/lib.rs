#![feature(proc_macro_quote)]
#![feature(proc_macro_value)]
#![allow(clippy::shadow_unrelated)]
use proc_macro::{TokenStream, TokenTree, quote};
#[proc_macro]
pub fn units(tokens: TokenStream) -> TokenStream {
    let mut iter = tokens.into_iter();
    let mut parse = Vec::<TokenStream>::new();
    let mut shortv = Vec::<TokenStream>::new();
    let mut longv = Vec::<TokenStream>::new();
    while let Some(token) = iter.next() {
        let TokenTree::Literal(lit) = token else {
            continue;
        };
        let Ok(long) = lit.str_value() else {
            continue;
        };
        let TokenTree::Punct(punct) = iter.next().unwrap() else {
            continue;
        };
        let short = (punct.as_char() == '|').then(|| {
            let Some(TokenTree::Literal(lite)) = iter.next() else {
                return None;
            };
            let Ok(short) = lite.str_value() else {
                return None;
            };
            iter.next();
            Some(short)
        });
        iter.next();
        iter.next();
        iter.next();
        iter.next();
        let Some(TokenTree::Ident(ident)) = iter.next() else {
            continue;
        };
        iter.next();
        if let Some(s) = short {
            parse.push(quote!($long | $s => Self::$ident));
            shortv.push(quote!(Self::$ident => $s));
        } else {
            parse.push(quote!($long => Self::$ident));
            shortv.push(quote!(Self::$ident => $long));
        }
        longv.push(quote!(Self::$ident => $long));
    }
    quote!(
        fn parse(s: &str) -> Option<Self> {
            Some(match s {
                $(
                    $parse,
                )*
                _ => return None,
            })
        }
        fn to_short(self) -> &'static str {
            match self {
                $(
                    $shortv,
                )*
            }
        }
        fn to_long(self) -> &'static str {
            match self {
                $(
                    $longv,
                )*
            }
        }
    )
}
