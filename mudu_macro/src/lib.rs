use mudu::common::result::RS;
use mudu::error::ec::EC;
use mudu::m_error;
use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, GenericArgument, ItemFn, PathArguments, ReturnType, Type};

const RESULT_TYPE_NAME: &str = "RS";
#[proc_macro_attribute]
pub fn mudu_proc(_args: TokenStream, input: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(input as ItemFn);

    // function name
    let fn_name = &input_fn.sig.ident;
    let fn_name_str = fn_name.to_string();
    let fn_wrapper_ident = syn::Ident::new(
        &format!("{}{}", mudu::procedure::proc::MUDU_PROC_PREFIX, fn_name),
        fn_name.span(),
    );

    let fn_inner_ident = syn::Ident::new(
        &format!("{}{}", mudu::procedure::proc::MUDU_PROC_INNER_PREFIX, fn_name),
        fn_name.span(),
    );

    let fn_argv_desc = syn::Ident::new(
        &format!("{}{}", mudu::procedure::proc::MUDU_PROC_ARGV_DESC_PREFIX, fn_name),
        fn_name.span(),
    );
    let fn_result_desc = syn::Ident::new(
        &format!("{}{}", mudu::procedure::proc::MUDU_PROC_RESULT_DESC_PREFIX, fn_name),
        fn_name.span(),
    );
    let fn_proc_desc = syn::Ident::new(
        &format!(
            "{}{}",
            mudu::procedure::proc::MUDU_PROC_PROC_DESC_PREFIX,
            fn_name
        ),
        fn_name.span(),
    );

    let mut types = Vec::new();
    let mut ty_string = Vec::new();
    let mut idents = Vec::new();
    for (i, input_arg) in input_fn.sig.inputs.iter().enumerate() {
        if let syn::FnArg::Typed(pat_type) = input_arg {
            if i == 0 {
                // skip first argument xid:XID
                continue;
            }

            if let syn::Pat::Ident(pat_ident) = &*pat_type.pat {
                idents.push(&pat_ident.ident);
                ty_string.push(pat_type.ty.to_token_stream().to_string());
                types.push(&pat_type.ty);
            }
        }
    }

    // argument conversion
    let mut param_conversions: Vec<_> = Vec::new();
    for (i, ty) in types.iter().enumerate() {
        let type_str = &ty_string[i];
        let _ident = idents[i];
        param_conversions.push(quote! {
            ::mudu::tuple::datum::binary_to_typed::<#ty, _>(&param.param_vec()[#i], #type_str)?
        })
    }

    let (_ret_type, inner_type) = handle_return_type(&input_fn).unwrap();

    let return_desc_construction = build_return_desc(&inner_type);

    let invoke_handling = if is_vec_return_type(&inner_type) {
        quote! {
            let return_desc = #return_desc_construction;
            let res = #fn_name(param.xid(), #(#param_conversions),*);
            Ok(::mudu::procedure::proc_result::ProcResult::from_vec(res, &return_desc)?)
        }
    } else if is_tuple_return_type(&inner_type) {
        quote! {
            let return_desc = #return_desc_construction;
            let res = #fn_name(param.xid(), #(#param_conversions),*);
            Ok(::mudu::procedure::proc_result::ProcResult::from(res, &return_desc)?)
        }
    } else {
        // basic type
        quote! {
            let return_desc = #return_desc_construction;
            let res = #fn_name(param.xid(), #(#param_conversions),*);
            let tuple = (res,);
            Ok(::mudu::procedure::proc_result::ProcResult::from(tuple, &return_desc)?)
        }
    };

    let output = quote! {
        #input_fn

        #[unsafe(no_mangle)]
        pub extern "C" fn #fn_wrapper_ident (p1_ptr: *const u8, p1_len: usize, p2_ptr: *mut u8, p2_len: usize) -> i32 {
            ::mudu::procedure::proc::invoke_proc_wrapper(
                p1_ptr, p1_len,
                p2_ptr, p2_len,
                #fn_inner_ident
            )
        }

        pub fn #fn_inner_ident(
            param: ::mudu::procedure::proc_param::ProcParam,
        ) -> ::mudu::common::result::RS<::mudu::procedure::proc_result::ProcResult> {
            // generate tuple desc
            let desc = <(#(#types),*)  as ::mudu::tuple::rs_tuple_datum::RsTupleDatum>::tuple_desc_static();

            #invoke_handling
        }

        pub fn #fn_argv_desc()  -> &'static ::mudu::tuple::tuple_field_desc::TupleFieldDesc {
            static ARGV_DESC: std::sync::OnceLock<::mudu::tuple::tuple_field_desc::TupleFieldDesc> =
                std::sync::OnceLock::new();
            ARGV_DESC.get_or_init(||
                {
                    <(#(#types),*)  as ::mudu::tuple::rs_tuple_datum::RsTupleDatum>::tuple_desc_static()
                }
            )
        }

        pub fn #fn_result_desc() -> &'static ::mudu::tuple::tuple_field_desc::TupleFieldDesc {
            static RESULT_DESC: std::sync::OnceLock<::mudu::tuple::tuple_field_desc::TupleFieldDesc> =
                std::sync::OnceLock::new();
            RESULT_DESC.get_or_init(||
                {
                    #return_desc_construction
                }
            )
        }

        pub fn #fn_proc_desc()  -> &'static ::mudu::procedure::proc_desc::ProcDesc {
            static PROC_DESC: std::sync::OnceLock<
                ::mudu::procedure::proc_desc::ProcDesc,
            > = std::sync::OnceLock::new();
            PROC_DESC
                .get_or_init(|| {
                    ::mudu::procedure::proc_desc::ProcDesc::new(
                        std::env!("CARGO_PKG_NAME").to_string(),
                        #fn_name_str.to_string(),
                        #fn_argv_desc().clone(),
                        #fn_result_desc().clone()
                    )
                })
        }
    };

    output.into()
}

fn build_return_desc(inner_type: &Type) -> proc_macro2::TokenStream {
    if is_vec_return_type(inner_type) {
        // Vec<T>
        if let Type::Path(type_path) = inner_type {
            if let Some(segment) = type_path.path.segments.last() {
                if let PathArguments::AngleBracketed(args) = &segment.arguments {
                    if let Some(GenericArgument::Type(element_type)) = args.args.first() {
                        // for Vec<T>，use T
                        if is_tuple_return_type(element_type) {
                            // Vec<(T1, T2, ...)>
                            return quote! {
                                <#element_type  as ::mudu::tuple::rs_tuple_datum::RsTupleDatum>::tuple_desc_static()
                            };
                        } else {
                            // Vec<T> - wrap with Vec<(T,)>
                            return quote! {
                                <(#element_type,) as ::mudu::tuple::rs_tuple_datum::RsTupleDatum>::tuple_desc_static()
                            };
                        }
                    }
                }
            }
        }
    } else if is_tuple_return_type(inner_type) {
        // a tuple (T1, T2, ...)
        return quote! {
            <#inner_type as ::mudu::tuple::rs_tuple_datum::RsTupleDatum>::tuple_desc_static()
        };
    } else {
        // basic type T - use tuple (T,)
        return quote! {
            use ;
            <(#inner_type,) as ::mudu::tuple::rs_tuple_datum::RsTupleDatum>::tuple_desc_static()
        };
    }

    // default
    quote! {
        use ;
        <() as ::mudu::tuple::rs_tuple_datum::RsTupleDatum>::tuple_desc_static()
    }
}
// check if return a vec type
fn is_vec_return_type(ty: &Type) -> bool {
    if let Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            if segment.ident == "Vec" {
                return true;
            }
        }
    }
    false
}

// check if return a tuple type
fn is_tuple_return_type(ty: &Type) -> bool {
    if let Type::Tuple(_) = ty {
        return true;
    }
    false
}


// return (result type, inner type), eg. (Result<T, MError>, T)
fn handle_return_type(item_fn: &ItemFn) -> RS<(Type, Type)> {
    let return_type = &item_fn.sig.output;
    let box_type = match return_type {
        ReturnType::Default => {
            panic!("A Mudu Procedure cannot return \"()\"")
        }
        ReturnType::Type(_, ty) => {
            ty
        }
    };
    let ty_path = if let Type::Path(type_path) = &(**box_type) {
        if let Some(segment) = type_path.path.segments.last() {
            if segment.ident == RESULT_TYPE_NAME {
                type_path
            } else {
                return Err(m_error!(EC::ParseErr,
                    format!("Expected Result type, found {}", segment.ident)
                ));
            }
        } else {
            return Err(m_error!(EC::ParseErr,"Expected Result type"));
        }
    } else {
        return Err(m_error!(EC::ParseErr,"Expected Result type"));
    };

    // test generics parameters, it must be RS<T>,
    let generics = if let PathArguments::AngleBracketed(args) =
        &ty_path.path.segments.last().unwrap().arguments {
        &args.args
    } else {
        return Err(m_error!(EC::ParseErr,"Result type must have generic parameters"));
    };
    if generics.len() != 1 {
        return Err(m_error!(EC::ParseErr,format!("Result must have exactly 2 generic parameters, found {}", generics.len())));
    }

    // retrieve T and E type in Result<T, E>
    let t_type = match &generics[0] {
        GenericArgument::Type(ty) => ty,
        _ => return Err(m_error!(EC::ParseErr, "Expected type parameter for T")),
    };

    Ok((*box_type.clone(), t_type.clone()))
}

