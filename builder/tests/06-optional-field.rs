/// 有些字段并不是必须的，可以将该字段的类型定义为std::option::Option<T>类型来处理
///
/// 在解析了宏输入中的类型为std::option::Option<T>的标识符之后，需要为这种类型生成对应
/// 的builder方法，在下面的测试用例中，`current_dir`字段是一个可选项，并不需要在main函
/// 数中对其进行赋值。
///
/// rust编译器会在宏展开之后才进行`name resolution`，这就意味着，在宏展开时，并没有`type`
/// 的概念，仅仅存在token串，一般情况下，可能会存在多个不同的token指向同一种类型：
///     std::option::Option<T> Option<T> <Vec<Option<T>> as IntoIterator>::Iterm都是
///     同一种类型的不同表示
/// 相反，一个token也可能指向多个不同的类型：
///     Error的意义依据上下文决定--std::error::Error 或 std::io::Error
/// 结果就是，在过程宏中，不可能凭借比较两个token来判断他们是否指向同一种类型
///
//
// In the context of the current test case, all of this means that there isn't
// some compiler representation of Option that our macro can compare fields
// against to find out whether they refer to the eventual Option type after name
// resolution. Instead all we get to look at are the tokens of how the user has
// described the type in their code. By necessity, the macro will look for
// fields whose type is written literally as Option<...> and will not realize
// when the same type has been written in some different way.
//
// The syntax tree for types parsed from tokens is somewhat complicated because
// there is such a large variety of type syntax in Rust, so here is the nested
// data structure representation that your macro will want to identify:
//
//     Type::Path(
//         TypePath {
//             qself: None,
//             path: Path {
//                 segments: [
//                     PathSegment {
//                         ident: "Option",
//                         arguments: PathArguments::AngleBracketed(
//                             AngleBracketedGenericArguments {
//                                 args: [
//                                     GenericArgument::Type(
//                                         ...
//                                     ),
//                                 ],
//                             },
//                         ),
//                     },
//                 ],
//             },
//         },
//     )

use derive_builder::Builder;

#[derive(Builder)]
pub struct Command {
    executable: String,
    args: Vec<String>,
    env: Vec<String>,
    current_dir: Option<String>,
}

fn main() {
    let command = Command::builder()
        .executable("cargo".to_owned())
        .args(vec!["build".to_owned(), "--release".to_owned()])
        .env(vec![])
        .build()
        .unwrap();
    assert!(command.current_dir.is_none());

    let command = Command::builder()
        .executable("cargo".to_owned())
        .args(vec!["build".to_owned(), "--release".to_owned()])
        .env(vec![])
        .current_dir("..".to_owned())
        .build()
        .unwrap();
    assert!(command.current_dir.is_some());
}
