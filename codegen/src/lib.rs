use proc_macro::TokenStream;

mod task;

#[proc_macro_attribute]
pub fn task(args: TokenStream, input: TokenStream) -> TokenStream {
    // args对应的是注解的参数部分
    // input对应的是被装饰的函数体
    task::with(args, input)
}

fn input_and_compile_error(mut item: TokenStream, err: syn::Error) -> TokenStream {
    let compile_err = TokenStream::from(err.to_compile_error());
    item.extend(compile_err);
    item
}