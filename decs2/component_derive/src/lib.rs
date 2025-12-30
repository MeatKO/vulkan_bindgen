use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{
    DeriveInput, Error, FnArg, GenericArgument, Pat, PathArguments, Type, TypePath, TypeReference, TypeTuple, parse_macro_input, punctuated::Punctuated, token::Comma
};

#[proc_macro_derive(Component)]
pub fn derive_component(input: TokenStream) -> TokenStream 
{
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;

    // generics handling is automatic here
    let generics = &ast.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let expanded = 
        quote! {
            impl #impl_generics Component for #name #ty_generics #where_clause 
            {
                fn as_any(&self) -> &dyn std::any::Any 
                {
                    self
                }

                fn as_any_mut(&mut self) -> &mut dyn std::any::Any 
                {
                    self
                }
            }
        };

    TokenStream::from(expanded)
}

#[allow(non_camel_case_types)]
enum mQueryParameter
{
    Ref(Type),
    MutRef(Type),
    OptionRef(Type),
    MutOptionRef(Type),
}

#[allow(non_camel_case_types)]
struct mQuery
{
    identifier: String,
    query_parameters: Vec<mQueryParameter>,
}

#[proc_macro_attribute]
pub fn system(_attr: TokenStream, item: TokenStream) -> TokenStream 
{
    let function_item = parse_macro_input!(item as syn::ItemFn);

    let signature = function_item.sig;
    let vis= &function_item.vis;
    let ident= &signature.ident;
    let generics= &signature.generics;
    let body = &function_item.block.stmts;

    let parsed_query_parameters = 
        match parse_function_inputs(&signature.inputs)
        {
            Ok(parsed_inputs) => { parsed_inputs }
            Err(err) => 
            {
                return 
                err
                .to_compile_error()
                .into();
            }
        };

    let type_registrations = 
        parsed_query_parameters
        .iter()
        .flat_map(
            |query|
            {
                query.query_parameters
                .iter()
                .map(
                    |parameter|
                    {
                        match parameter
                        {
                            mQueryParameter::Ref(ty) 
                            | mQueryParameter::MutRef(ty)
                            | mQueryParameter::OptionRef(ty)
                            | mQueryParameter::MutOptionRef(ty) => 
                            {
                                quote! { decs.register_type::<#ty>(); }
                            }
                        }
                    }
                )
            }
        );

    let query_tokens = 
        parsed_query_parameters
        .iter()
        .map(
            |query|
            {
                let identifier = syn::Ident::new(&query.identifier, Span::call_site());

                let query_params = 
                    query.query_parameters
                    .iter()
                    .map(query_param_to_type);

                // Changed this because Rust-analyzer was struggling to see the types
                // Because #identifier is first in the function parameters and then removed and put in the body
                // quote! {
                //     let #identifier = Query::<( #(#query_params),* )>::new(decs);
                // }
                quote! {
                    let #identifier: Query::<( #(#query_params),* )> = Query::new(decs);
                }
            }
        );

    let expanded =
        quote!{
            #vis fn #ident #generics (decs: &mut dECSManager)
            {
                use decs2::query::*;

                #(#type_registrations)*

                #(#query_tokens)*

                #(#body)*
            }
        };

    expanded.into()
}

// gippidy
fn query_param_to_type(param: &mQueryParameter) -> proc_macro2::TokenStream 
{
    match param 
    {
        mQueryParameter::Ref(t) => { quote! { Read<#t> } }
        mQueryParameter::MutRef(t) => { quote! { Write<#t> } }
        mQueryParameter::OptionRef(t) => { quote! { MaybeRead<#t> } }
        mQueryParameter::MutOptionRef(t) => { quote! { MaybeWrite<#t> } }
    }
}

fn parse_function_inputs(inputs: &Punctuated<FnArg, Comma>) -> Result<Vec<mQuery>, syn::Error>
{
    let mut out_queries = vec![];

    for input in inputs.iter()
    {
        match input
        {
            FnArg::Receiver(pat_type) => 
            {
                return Err(Error::new_spanned(
                    &pat_type,
                    "self references are not allowed, define a Query<(&A, &mut B, Option<&C>, Option<&mut D>)> instead.",
                ))
            }
            FnArg::Typed(pat_type) => 
            {
                let input_name = 
                    match pat_type.pat.as_ref()
                    {
                        Pat::Ident(pat_ident) => { pat_ident.ident.to_string() }
                        _ => 
                        {
                            return Err(Error::new_spanned(
                                &pat_type.pat,
                                "destructuring patterns and macros are not allowed as System parameters",
                            ))
                        }
                    };

                match pat_type.ty.as_ref()
                {
                    Type::Path(type_path) =>
                    {
                        match query_arguments_to_enums(type_path, &input_name)
                        {
                            Ok(query_parameters) => 
                            {
                                out_queries.push(
                                    mQuery { 
                                        identifier: input_name,
                                        query_parameters: query_parameters
                                    }
                                );

                                continue;
                            }
                            Err(err) => 
                            {
                                return Err(Error::new_spanned(
                                    &pat_type.ty,
                                    err,
                                ))
                            }
                        }
                    }
                    _ => 
                    {
                        return Err(Error::new_spanned(
                            &pat_type.ty,
                            format!("consider specifying a query instead - {}: Query<(&SomeComponent)>", input_name),
                        ))
                    }
                };
            }
        }
    }

    Ok(out_queries)
}

fn query_arguments_to_enums(type_path: &TypePath, input_name: &str) -> Result<Vec<mQueryParameter>, String>
{
    if type_path.path.segments.len() == 0
    {
        return Err(format!("consider specifying a query instead - {}: Query<(&SomeComponent)>", input_name))
    }

    let path_identifier = &type_path.path.segments.last().unwrap().ident;
    if path_identifier != "Query"
    {
        return Err(format!("unsupported identifier '{}', consider specifying a query instead - {}: Query<(&SomeComponent)>", path_identifier, input_name))
    }

    let arguments = 
        match &type_path.path.segments.last().unwrap().arguments
        {
            PathArguments::AngleBracketed(arguments) => { &arguments.args }
            _ => { return Err(format!("correct synthax is: Query<(&SomeComponent)>")) }
        };

    if arguments.len() != 1
    {
        return Err(format!("expected a single tuple containing the component types"))
    }
    
    match arguments.last().unwrap()
    {
        GenericArgument::Type(tuple_contents) => 
        {
            match tuple_contents
            {
                Type::Tuple(type_tuple) => { return parse_tuple(type_tuple) } // multiple params in a tuple
                // Type::Paren(type_paren) => { return parse_single(type_paren) } // single param
                _ => { return Err(format!("only tupled arguments are allowed")) }
            }
        }
        _ => { return Err(format!("expected a tuple, lifetimes, const expressions and associated types are not allowed.")) }
    };
}
fn parse_tuple(type_tuple: &TypeTuple) -> Result<Vec<mQueryParameter>, String>
{
    let mut out_query_types = vec![];

    for inner_type in type_tuple.elems.iter().by_ref()
    {
        match inner_type
        {
            Type::Path(type_path) => { out_query_types.push(parse_concrete_query_parameter(type_path)?) }
            Type::Reference(type_reference) => { out_query_types.push(parse_reference_query_parameter(type_reference)?) },
            Type::Tuple(_) | Type::Paren(_) => { return Err(format!("nested tuples are not allowed")) },
            _ => { return Err(format!("expected a reference or an Option of a reference")) },
        }
    }

    Ok(out_query_types)
}

fn parse_reference_query_parameter(ref_parameter: &TypeReference) -> Result<mQueryParameter, String>
{
    match ref_parameter.elem.as_ref()
    {
        Type::Path(_) => {}
        Type::Reference(_) => { return Err(format!("nested references are not allowed")) }
        ty => { return Err(format!("could not infer type for '{:?}'", ty)) }
    };

    match ref_parameter.mutability
    {
        Some(_) => { return Ok(mQueryParameter::MutRef(ref_parameter.elem.as_ref().clone())) },
        None => { return Ok(mQueryParameter::Ref(ref_parameter.elem.as_ref().clone())) }
    };
}

// everything that is not labelled & or &mut
fn parse_concrete_query_parameter(concrete_parameter: &TypePath) -> Result<mQueryParameter, String>
{
    if concrete_parameter.path.segments.len() == 0
    {
        return Err("how the actual fuck did you write an empty type?".into())  
    }

    if concrete_parameter.path.segments.last().unwrap().ident.to_string() != "Option"
    {
        return Err("concrete types are not accepted".into());
    }

    let arguments = 
        match &concrete_parameter.path.segments.last().unwrap().arguments
        {
            PathArguments::AngleBracketed(arguments) => { arguments }
            _ => { return Err("improperly specified Option type".into()); }
        };

    if arguments.args.len() != 1
    {
        return Err("only a single type is allowed inside the option".into())
    }

    match arguments.args.last().unwrap()
    {
        GenericArgument::Type(ty) => 
        {
            match ty
            {
                Type::Reference(type_reference) => 
                { 
                    match type_reference.elem.as_ref()
                    {
                        Type::Path(_) => {}
                        Type::Reference(_) => { return Err(format!("nested references are not allowed")) }
                        ty => { return Err(format!("could not infer type for '{:?}'", ty)) }
                    };

                    match type_reference.mutability
                    {
                        Some(_) => { return Ok(mQueryParameter::MutOptionRef(type_reference.elem.as_ref().clone())) },
                        None => { return Ok(mQueryParameter::OptionRef(type_reference.elem.as_ref().clone())) }
                    };
                }
                Type::Tuple(_) | Type::Paren(_) => { return Err(format!("nested tuples are not allowed")) },
                _ => { return Err(format!("expected a reference")) },
            }
        }
        _ => { return Err("expected a type, lifetimes, const expressions and associated types are not allowed.".into())}
    }
}