extern crate proc_macro;

use std::fmt::format;

use proc_macro::{TokenStream, TokenTree, Delimiter, Group};

#[proc_macro_attribute]
pub fn component(_attr: TokenStream, struct_ts: TokenStream) -> TokenStream 
{
	let mut struct_tt = struct_ts.clone().into_iter().collect::<Vec<TokenTree>>();

	let mut struct_name = None;
	let mut name_found = false;

	for token in struct_ts.clone() 
	{
		if name_found
		{
			if let TokenTree::Ident(ident) = token 
			{
				struct_name = Some(ident);
				break;
			}
		}

		if let TokenTree::Ident(ident) = token 
		{
			if ident.to_string() == "struct".to_owned()
			{
				name_found = true; // check above will be triggered on next iteration... perhaps rewrite with iterators and window(2) ?
			}
        }
    }

	let struct_name =
		match struct_name 
		{
			Some(name) => { name },
			None => { return "compile_error!(\"macro can only be applied to structs.\");".parse().unwrap() }
		};

	let mut derive_debug = "#[derive(Debug)]\n".parse::<TokenStream>().unwrap().into_iter().collect::<Vec<TokenTree>>();

	derive_debug.append(&mut struct_tt);
	struct_tt = derive_debug;

	let impl_block = format!("impl Component for {} {{ }}", struct_name).parse::<TokenStream>().unwrap();
	struct_tt.extend(impl_block);

	let out_vec = struct_tt.into_iter().collect::<TokenStream>();

	out_vec 
}

#[proc_macro_attribute]
pub fn component2(_attr: TokenStream, struct_ts: TokenStream) -> TokenStream 
{
	let mut struct_tt = struct_ts.clone().into_iter().collect::<Vec<TokenTree>>();

	let mut struct_name = None;
	let mut name_found = false;

	for token in struct_ts.clone() 
	{
		if name_found
		{
			if let TokenTree::Ident(ident) = token 
			{
				struct_name = Some(ident);
				break;
			}
		}

		if let TokenTree::Ident(ident) = token 
		{
			if ident.to_string() == "struct".to_owned()
			{
				name_found = true;
			}
        }
    }

	let lifetime_str = 
		struct_ts.clone()
		.into_iter()
		.filter(|token| 
			{
				match token
				{
					TokenTree::Punct(char) => { char.as_char() == '\'' }
					_ => { false }
				}
			}
		)
		.map(|_| "'static")
		.collect::<Vec<&str>>();

	let struct_name =
		match struct_name 
		{
			Some(name) => { name },
			None => { return "compile_error!(\"macro can only be applied to structs.\");".parse().unwrap() }
		};

	// panic!("struct name is '{}'", struct_name);

	let mut derive_debug = "#[derive(Debug)]\n".parse::<TokenStream>().unwrap().into_iter().collect::<Vec<TokenTree>>();

	derive_debug.append(&mut struct_tt);
	struct_tt = derive_debug;

	let lifetime_joined_str = lifetime_str.join(", ");

	// panic!("lifetime count {}", lifetime_str.len());

	let impl_block =
		if lifetime_str.len() > 0
		{
			format!("impl Component for {}<{}> {{ }}", struct_name, lifetime_joined_str).parse::<TokenStream>().unwrap()
		}
		else
		{
			format!("impl Component for {} {{ }}", struct_name).parse::<TokenStream>().unwrap()
		};

	
	struct_tt.extend(impl_block);

	let out_vec = struct_tt.into_iter().collect::<TokenStream>();

	out_vec 
	// panic!("{:#?}", out_vec.to_string());
}

#[proc_macro_attribute]
pub fn system(_attr: TokenStream, item: TokenStream) -> TokenStream 
{
    let mut function_definition = item.into_iter().collect::<Vec<_>>();

    if let Some(pos) = 
		function_definition
			.iter()
			.position(
				|tt| 
				matches!(tt, TokenTree::Group(g) if g.delimiter() == Delimiter::Parenthesis)
			) 
	{
        if let TokenTree::Group(ref mut g) = function_definition[pos] 
		{
            // Parse the new parameter
            let new_param = stringify!(decs: &mut dECS,).parse::<TokenStream>().unwrap();
            
            // Combine the new parameter with the existing contents of the group
            let mut new_group_stream = new_param;
            new_group_stream.extend(g.stream().into_iter());

            // Create a new group with the updated token stream
            let mut new_group = Group::new(Delimiter::Parenthesis, new_group_stream);
			new_group.set_span(g.span());
            function_definition[pos] = TokenTree::Group(new_group);
        }
    }

    function_definition.into_iter().collect()
}