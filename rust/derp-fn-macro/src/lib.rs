use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, Expr, Ident, Result, Token,
};

struct Lazy {
    name: Ident,
    expect: Expr,
}

impl Parse for Lazy {
    fn parse(input: ParseStream) -> Result<Self> {
        let name: Ident = input.parse()?;
        input.parse::<Token![@]>()?;
        let expect: Expr = input.parse()?;

        Ok(Lazy { name, expect })
    }
}
#[proc_macro]
pub fn lazy(input: TokenStream) -> TokenStream {
    let Lazy { name, expect } = parse_macro_input!(input as Lazy);

    quote::format_ident!("");
    TokenStream::from(quote!(
        #[test]
        fn #name() {
            let message = generate_twitch_command("#name", #name);

            let message = serde_json::from_str::<TwitchMessage>(&message).unwrap();

            let result = handle_webhook_message(message);

            assert_eq!(result, #expect);
        }
    ))
}
