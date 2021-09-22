/// 为了能让编译器识别字段属性，必须在派生宏定义的时候声明它，否则编译器将会报告一个
/// 未识别的属性，并拒绝编译。
///
/// ```
///  #[proc_macro_derive(Builder, attributes(builder))]
/// ```
///
/// 这些属性称为惰性属性，惰性表明这些属性本身并不对应宏调用，而是在其他宏调用展开时会
/// 查看这些属性。
//
// If the new one-at-a-time builder method is given the same name as the field,
// avoid generating an all-at-once builder method for that field because the
// names would conflict.
//
//
// Resources:
//
//   - Relevant syntax tree types:
//     https://docs.rs/syn/1.0/syn/struct.Attribute.html
//     https://docs.rs/syn/1.0/syn/enum.Meta.html

use derive_builder::Builder;

#[derive(Builder)]
pub struct Command {
    executable: String,
    #[builder(each = "arg")]
    args: Vec<String>,
    #[builder(each = "env")]
    env: Vec<String>,
    current_dir: Option<String>,
}

fn main() {
    let command = Command::builder()
        .executable("cargo".to_owned())
        .arg("build".to_owned())
        .arg("--release".to_owned())
        .build()
        .unwrap();

    assert_eq!(command.executable, "cargo");
    assert_eq!(command.args, vec!["build", "--release"]);
}
