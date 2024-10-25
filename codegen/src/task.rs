use crate::input_and_compile_error;
use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::{quote, ToTokens};
use syn::{punctuated::Punctuated, LitStr, Token};

pub struct TaskArgs {
    // 路由路径，例如 "/foo"
    pub(crate) path: syn::LitStr,
    // 注解的额外参数, 当前只允许name参数
    pub(crate) options: Punctuated<syn::MetaNameValue, Token![,]>,
}

impl syn::parse::Parse for TaskArgs {
    fn parse(input: syn::parse::ParseStream<'_>) -> syn::Result<Self> {
        let path = input.parse::<syn::LitStr>().map_err(|mut err| {
            err.combine(syn::Error::new(
                err.span(),
                r#"invalid service definition, expected #[<method>("<path>")]"#,
            ));

            err
        })?;

        let options = input.parse_terminated(syn::MetaNameValue::parse, Token![,])?;

        Ok(Self { path, options })
    }
}

struct Args {
    path: syn::LitStr,
    resource_name: Option<syn::LitStr>,
}

impl Args {
    fn new(args: TaskArgs) -> syn::Result<Self> {
        let mut resource_name = None;

        for nv in args.options {
            if nv.path.is_ident("name") {
                if let syn::Expr::Lit(syn::ExprLit {
                    lit: syn::Lit::Str(lit),
                    ..
                }) = nv.value
                {
                    resource_name = Some(lit);
                } else {
                    return Err(syn::Error::new_spanned(
                        nv.value,
                        "Attribute name expects literal string",
                    ));
                }
            }
        }
        Ok(Args {
            path: args.path,
            resource_name,
        })
    }
}

pub struct Task {
    // 被注解的函数名
    name: syn::Ident,

    args: Vec<Args>,

    // handler函数ast
    ast: syn::ItemFn,
}

impl Task {
    pub fn new(args: TaskArgs, ast: syn::ItemFn) -> syn::Result<Self> {
        // 获取函数名
        let name = ast.sig.ident.clone();

        let args = Args::new(args)?;

        // 函数需要一个返回值
        if matches!(ast.sig.output, syn::ReturnType::Default) {
            return Err(syn::Error::new_spanned(
                ast,
                "Function has no return type. Cannot be used as handler",
            ));
        }

        Ok(Self {
            name,
            args: vec![args],
            ast,
        })
    }
}

impl ToTokens for Task {
    fn to_tokens(&self, output: &mut TokenStream2) {
        let Self { name, ast, args } = self;

        #[allow(unused_variables)] // used when force-pub feature is disabled
        let vis = &ast.vis;

        // TODO(breaking): remove this force-pub forwards-compatibility feature
        #[cfg(feature = "compat-routing-macros-force-pub")]
        let vis = syn::Visibility::Public(<Token![pub]>::default());

        let registrations: TokenStream2 = args
            .iter()
            .map(|args| {
                let Args {
                    path,
                    resource_name,
                } = args;

                let resource_name = resource_name
                    .as_ref()
                    .map_or_else(|| name.to_string(), LitStr::value);

                quote! {
                    todo!
                }
            })
            .collect();

        let stream = quote! {
            #[allow(non_camel_case_types, missing_docs)]
            #vis struct #name;

            impl ::actix_web::dev::HttpServiceFactory for #name {
                fn register(self, __config: &mut actix_web::dev::AppService) {
                    #ast
                    #registrations
                }
            }
        };

        output.extend(stream);
    }
}

pub(crate) fn with(args: TokenStream, input: TokenStream) -> TokenStream {
    // 解析注解参数
    let args = match syn::parse(args) {
        Ok(args) => args,
        Err(err) => return input_and_compile_error(input, err),
    };

    // 解析函数体ast
    let ast: syn::ItemFn = match syn::parse::<syn::ItemFn>(input.clone()) {
        Ok(ast) => ast,
        Err(err) => return input_and_compile_error(input, err),
    };

    match Task::new(args, ast) {
        Ok(t) => t.into_token_stream().into(),
        Err(err) => input_and_compile_error(input, err),
    }
}
