#![feature(proc_macro_span, proc_macro_quote)]

use proc_macro::{Span, TokenStream};
use quote::{quote, format_ident};
use serde_json::Value;
use std::fs;

fn generate_token_stream(struct_name_string: &str, json_value: &Value, struct_defs: &mut Vec<proc_macro2::TokenStream>, rust_type: &mut proc_macro2::TokenStream, entry_point: bool) -> proc_macro2::TokenStream {
    let token = match json_value {
        Value::Object(object_map) => {
            let mut struct_fields_instance = Vec::new();
            let mut struct_fields = Vec::new();
            for (field_name, field_value) in object_map {
                let identity = format_ident!("{}", field_name);

                let mut field_rust_type =  quote! {};
                let value_token = generate_token_stream(struct_name_string, field_value, struct_defs, &mut field_rust_type, false);

                struct_fields_instance.push(quote!{#identity: #value_token});
                struct_fields.push(quote!{#identity: #field_rust_type});
            }

            let struct_name = if entry_point {
                format_ident!("{}", struct_name_string)
            } else {
                format_ident!("{}{}", struct_name_string, struct_defs.len())
            };

            let struct_def = quote! {
                struct #struct_name {
                    #(#struct_fields),*
                }
            };
            struct_defs.push(struct_def);

            *rust_type = quote! { #struct_name };

            quote! {
                #struct_name {
                    #(#struct_fields_instance),*
                }
            }
        },
        Value::String(string_value) => {
            *rust_type = quote! { &'static str };
            quote! { #string_value }
        },
        Value::Null => todo!(),
        Value::Bool(bool_value) => {
            *rust_type = quote! { bool };
            quote! { #bool_value }
        },
        Value::Number(number_value) => {
            let n;
            if number_value.is_u64() {
                let value = number_value.as_u64().expect("could not parse");
                *rust_type = quote! { u64 };
                n = quote! { #value };
            } else if number_value.is_i64() {
                let value = number_value.as_i64().expect("could not parse");
                *rust_type = quote! { i64 };
                n = quote! { #value };
            } else {
                let value = number_value.as_f64().expect("could not parse");
                *rust_type = quote! { f64 };
                n = quote! { #value };
            }
            n
        },
        Value::Array(array_value) => {
            let mut array_type = quote!{};
            let iter = array_value.iter()
            .map(|element| generate_token_stream(struct_name_string, element, struct_defs, &mut array_type, false));
            let value = quote! { [#(#iter),*] };

            let array_length = array_value.len();
            *rust_type = quote! {[#array_type; #array_length]};
            return value;
        },
    };

    return token;
}

#[proc_macro_attribute]
pub fn load_json(attr: TokenStream, the_rest: TokenStream) -> TokenStream {
    let args_string = attr.to_string();
    let args = args_string.split(',').collect::<Vec<&str>>();

    let variable_name = args[0].trim();
    let struct_name = args[1].trim();
    let json_path = args[2].trim();

    let mut json_path_chars = json_path.chars();
    json_path_chars.next();
    json_path_chars.next_back();
    let json_path = json_path_chars.as_str();

    let span = Span::call_site();
    let json_path = span
        .source_file()
        .path()
        .parent()
        .expect("borkn1")
        .join(json_path);

    let json_file = fs::read_to_string(json_path).expect("borken2");
    let json_file: Value = serde_json::from_str(&json_file).expect("prblem");

    let mut struct_definitions = Vec::new();

    let mut rust_type = quote!{};
    let struct_value = generate_token_stream(struct_name, &json_file, &mut struct_definitions, &mut rust_type, true).to_string();
    let struct_def = struct_definitions.iter().map(|a| a.to_string()).collect::<String>();

    // TODO: can use TokenStream extending?
    let generated_string = format!("{}\nconst {}: {} = {};\n{}",struct_def, variable_name, struct_name, struct_value, the_rest);
    generated_string.parse().unwrap()
}
