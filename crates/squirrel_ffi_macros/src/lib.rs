use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input,
    ItemFn,
    Visibility,
    Abi,
    LitStr,
    Token,
};

// 定义属性宏
#[proc_macro_attribute]
pub fn sq_function(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // 解析函数
    let mut func = parse_macro_input!(item as ItemFn);
    
    // 设置函数的调用约定为 extern "C"
    // 处理可见性 - 确保函数是公开的
    func.vis = Visibility::Public(Token![pub](proc_macro2::Span::call_site()));
    
    // 添加 extern "C" 调用约定
    func.sig.abi = Some(Abi {
        extern_token: Token![extern](proc_macro2::Span::call_site()),
        name: Some(LitStr::new("C", proc_macro2::Span::call_site())),
    });
    
    // 生成最终代码
    TokenStream::from(quote! { #func })
}
