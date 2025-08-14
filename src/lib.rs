mod parsing;
use pest::{Parser, iterators::Pair};
use proc_macro2::TokenStream;
use syn::{LitBool, LitFloat, LitInt};
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
        result_token_stream.extend(rust_code);
    }
    return result_token_stream.into();
}

fn gen_rust_code(pair: Pair<Rule>) -> TokenStream {
    let mut result_token_stream = TokenStream::new();
    match pair.as_rule() {
        Rule::program => {
            for pair in pair.into_inner() {
                result_token_stream.extend(gen_rust_code(pair));
            }
            return result_token_stream;
        },
        Rule::assignment => {
            let mut assignment = pair.into_inner();
            let ident = gen_rust_code(assignment.next().unwrap());
            let expr = gen_rust_code(assignment.next().unwrap());
            result_token_stream.extend(quote! {#ident = #expr})
        },
        Rule::definition => {
            let mut definition = pair.into_inner();
            let ident = gen_rust_code(definition.next().unwrap());
            let expr = gen_rust_code(definition.next().unwrap());
            result_token_stream.extend(quote! {let #ident = #expr;});
        },
        Rule::identifier => {
            let ident = syn::parse_str::<syn::Ident>(pair.as_str()).unwrap();
            result_token_stream.extend(quote! {#ident});
        },
        Rule::expr_eval => {
            let expr_eval = pair.into_inner();
            for pair in expr_eval {
                result_token_stream.extend(gen_rust_code(pair));
            }
        },
        Rule::fn_call => {
            let mut fn_call = pair.into_inner();
            let ident = gen_rust_code(fn_call.next().unwrap());
            let params = gen_rust_code(fn_call.next().unwrap());
            result_token_stream.extend(quote! {#ident(#params)});
        },
        Rule::params => {
            let mut params = pair.into_inner();
            while params.len() > 0 {
                result_token_stream.extend(gen_rust_code(params.next().unwrap()));
            }
        },
        Rule::literal => {
            let mut literal = pair.into_inner();
            while literal.len() > 0 {
                result_token_stream.extend(gen_rust_code(literal.next().unwrap()));
            }
        },
        Rule::number => {
            let number = pair.into_inner().next().unwrap();
            result_token_stream.extend(gen_rust_code(number));
        },
        Rule::string => {
            let string = pair.as_str();
            result_token_stream.extend(quote! {#string});
        },
        Rule::boolean => {
            let boolean = syn::parse_str::<LitBool>(pair.as_str()).unwrap();
            result_token_stream.extend(quote! {#boolean});
        },
        Rule::discrete_number => {
            let discrete_number = syn::parse_str::<LitInt>(pair.as_str()).unwrap();
            result_token_stream.extend(quote! {#discrete_number});
        },
        Rule::float => {
            let float = syn::parse_str::<LitFloat>(pair.as_str()).unwrap();
            result_token_stream.extend(quote! {#float});
        },
        Rule::expr => {
            let mut expr = pair.into_inner();
            while expr.len() > 0 {
                result_token_stream.extend(gen_rust_code(expr.next().unwrap()));
            }
        },
        Rule::EOI => {
            return result_token_stream;
        },
        Rule::var => {
            let mut var = pair.into_inner();
            while var.len() > 0 {
                result_token_stream.extend(gen_rust_code(var.next().unwrap()));
            }
        },
        _ => {
            return result_token_stream;
        }
    }
    return result_token_stream;
}
