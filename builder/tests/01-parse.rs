// 需要实现一个派生宏 derive(Builder)
//
// 更进一步，我们可能将派生宏的输入解析为`syn::DeriveInput`语法树结构
//
// 花点时间看看 DeriveInput 结构的详细信息对后续有帮助
/// ```
/// pub struct DeriveInput {
///     pub attrs: Vec<Attribute>,
///     pub vis: Visibility,
///     pub ident: Ident,
///     pub generics: Generics,
///     pub data: Data,
/// }
/// ```
// 资源:
//
//   - The Syn crate 解析过程宏输入到自定义的语法树
//     https://github.com/dtolnay/syn
//
//   - The DeriveInput 派生宏的语法表示
//     https://docs.rs/syn/1.0/syn/struct.DeriveInput.html
//
//   - 使用syn的例子
//     https://github.com/dtolnay/syn/tree/master/examples/heapsize

use derive_builder::Builder;

#[derive(Builder)]
pub struct Command {
    executable: String,
    args: Vec<String>,
    env: Vec<String>,
    current_dir: String,
}

fn main() {}
