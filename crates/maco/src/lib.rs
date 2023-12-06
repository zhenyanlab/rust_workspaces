use proc_macro::TokenStream;
use syn::parse_macro_input;
// use proc_macro::quote;
use quote::quote;
use syn::DeriveInput;
use proc_macro2::Ident;
 // use proc_macro::Ident;
// use proc_macro::bridge::Ident;
// use syn::Ident;

#[proc_macro]
pub fn query(input: TokenStream) -> TokenStream {
    println!("{:#?}", input);
    "fn hello() { println!(\"Hello world!\"); }"
        .parse()
        .unwrap()
}

macro_rules! print_message {
    () => {
        println!("Hello, World!");
    };
}
#[proc_macro_derive(Builder)]
pub fn derive_builder(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let ident = input.ident;
    let ident_builder = Ident::new(&format!("{}Builder", ident), ident.span());

    println!("mm-ident:{}",ident);
    println!("mm-ident_builder:{}",ident_builder);
    quote!(
        pub struct #ident_builder{};
        impl #ident_builder {
            pub fn builder() -> #ident {
                #ident::default()
            }
        }
    ).into()


}
mod maco_test;