use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemFn, parse_macro_input};

#[proc_macro_attribute]
pub fn gates_dope(attr: TokenStream, func: TokenStream) -> TokenStream {
    let b = attr.to_string();
    let fun = parse_macro_input!(func as ItemFn);
    let name = &fun.sig.ident;
    println!("{}", b);
    quote! {
      // use tokio_tungstenite::tungstenite::WebSocket;
      pub fn #name(explore:&mut GatesDope,path:String){
        // println!("{}",path.to_string());
        let billionaires = #b.replace("\"","");
        if path == billionaires {
          #fun
          {#name(explore)}
        }
      }
    }
    .into()
}
