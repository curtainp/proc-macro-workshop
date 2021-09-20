/// 为派生宏修饰的类型生成对应的构造器类型：新类型名称为 修饰类型名称+Builder
/// 对应到用例中：
/// ```
///  pub struct CommandBuilder {
///     executable: Option<String>,
///     args: Option<Vec<String>>,
///     env: Option<Vec<String>>,
///     current_dir: Option<String>,
///  }
/// ```
/// 然后为修饰类型实现构造器函数
/// ```
///  impl Command {
///     pub fn builder() -> CommandBuilder {
///         CommandBuilder {
///             executable: None,
///             args: None,
///             env: None,
///             current_dir: None,
///         }
///     }
///  }
/// ```
///
/// ### 需要用到的知识点
///  * 标识符使用`syn::Ident`类型表示
///  * 从`syn::DeriveInput`类型中获取修饰结构体的各个组成部分：字段名、字段类型、结构名等等
///  * 使用`quote`crate提供的`quote!`宏将内部语法树转换成`rustc`支持的`TokenStream`类型
///
/// 资源:
///
///   - quote crate 将syn生成的内部ast转换成rustc支持的TokenStream
///     https://github.com/dtolnay/quote
///
///   - 将类型名和Builder组合在一起为类型实现构造器类型
///     https://docs.rs/syn/1.0/syn/struct.Ident.html
///
/// 当在lib.rs中实现了对应的TokenStream操作之后，将此文件中的代码拷贝到项目根目录下的main.rs中
/// 然后执行 `cargo expand -q`即可查看过程宏展开之后的代码
/// ```
///     #![feature(prelude_import)]
///     #[prelude_import]
///     use std::prelude::rust_2018::*;
///     #[macro_use]
///     extern crate std;
///     use derive_builder::Builder;
///     struct Command {
///         executable: String,
///         args: Vec<String>,
///         env: Vec<String>,
///         current_dir: String,
///     }
///     pub struct CommandBuilder {}
///     impl Command {
///         pub fn builder() -> CommandBuilder {}
///     }
///     fn main() {
///         let _ = Command::builder();
///     }
/// ```

use derive_builder::Builder;

#[derive(Builder)]
pub struct Command {
    executable: String,
    args: Vec<String>,
    env: Vec<String>,
    current_dir: String,
}

fn main() {
    let builder = Command::builder();

    let _ = builder;
}
