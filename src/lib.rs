mod parsing;
use pest::{Parser, iterators::Pair};
use proc_macro2::TokenStream;
use syn::{LitBool, LitFloat};
use parsing::*;
use quote::quote;

#[proc_macro]
pub fn glider(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut result_token_stream = proc_macro2::TokenStream::new();
    let input_string = input.to_string();
    let parsed_code = match GliderParser::parse(Rule::program, &input_string) {
        Ok(pairs) => pairs,
        Err(e) => {
            let error_message = format!("Glider parsing error: {}", e);
            return quote! { compile_error!(#error_message) }.into();
        }
    };
    for pair in parsed_code {
        let rust_code = gen_rust_code(pair);
        println!("{}", rust_code);
        result_token_stream.extend(rust_code);
    }
    return result_token_stream.into();
}

fn gen_rust_code(pair: Pair<Rule>) -> TokenStream {
    let mut result_token_stream = TokenStream::new();
    match pair.as_rule() {
        Rule::expression => {
            let expression = pair.into_inner();
            for pair in expression {
                result_token_stream.extend(gen_rust_code(pair));
            }
            result_token_stream.extend(quote! { ; });
        }
        Rule::definition => {
            let mut def = pair.into_inner();
            let ident = gen_rust_code(def.next().unwrap());
            let expr = gen_rust_code(def.next().unwrap());
            result_token_stream.extend(quote! { let mut #ident = #expr });
        }
        Rule::assignment => {
            let mut def = pair.into_inner();
            let ident = gen_rust_code(def.next().unwrap());
            let expr = gen_rust_code(def.next().unwrap());
            result_token_stream.extend(quote! { #ident = #expr });
        }
        Rule::identifier => {
            let ident = syn::parse_str::<syn::Ident>(pair.as_str()).unwrap();
            result_token_stream.extend(quote! { #ident });
        }
        Rule::float => {
            let float_lit = syn::parse_str::<LitFloat>(pair.as_str()).unwrap();
            result_token_stream.extend(quote! { #float_lit });
        }
        Rule::number => {
            let int_lit = syn::parse_str::<LitFloat>(&(pair.as_str().to_string()+".0")).unwrap();
            result_token_stream.extend(quote! { #int_lit });
        }
        Rule::bool => {
            let bool_lit = syn::parse_str::<LitBool>(pair.as_str()).unwrap();
            result_token_stream.extend(quote! { #bool_lit });
        }
        Rule::string => {
            let string_lit = syn::parse_str::<syn::LitStr>(pair.as_str()).unwrap();
            result_token_stream.extend(quote! { #string_lit });
        }
        // list = { "[" ~ ((identifier|evaluation|literal) ~ ",")* ~ (identifier|evaluation|literal) ~ "]" }
        Rule::list => {
            let list = pair.into_inner();
            let elements: Vec<TokenStream> = list.map(gen_rust_code).collect();
            result_token_stream.extend(quote! { [#(#elements),*] });
        }
        //term = { literal | identifier | "(" ~ evaluation ~ ")" }
        Rule::term => {
            let mut inner = pair.into_inner();
            if let Some(inner_pair) = inner.next() {
                match inner_pair.as_rule() {
                    Rule::evaluation => {
                        let eval_tokens = gen_rust_code(inner_pair);
                        result_token_stream.extend(quote! { ( #eval_tokens ) });
                    }
                    _ => {
                        let tokens = gen_rust_code(inner_pair);
                        result_token_stream.extend(tokens);
                    }
                }
            }
        }
        Rule::operation => {
            let mut operation = pair.into_inner();
            while operation.len() > 0 {
                let left = gen_rust_code(operation.next().unwrap());
                let operator = gen_rust_code(operation.next().unwrap());
                let right = gen_rust_code(operation.next().unwrap());
                result_token_stream.extend(quote! { #left #operator #right });
            }
        }
        Rule::add => {
            result_token_stream.extend(quote! { + });
        }
        Rule::sub => {
            result_token_stream.extend(quote! { - });
        }
        Rule::mul => {
            result_token_stream.extend(quote! { * });
        }
        Rule::div => {
            result_token_stream.extend(quote! { / });
        }
        Rule::EOI => {
            return result_token_stream
        }
        _ => {
            for pair in pair.into_inner() {
                result_token_stream.extend(gen_rust_code(pair));
            }
        }
    }
    return result_token_stream;
}
