use asteroid::typescript::parser;
use std::path::Path;
use std::any::type_name;

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

#[test]
fn test_parse_file() {
    let path = Path::new("tests/greeter.ts");
    let (script, _) = parser::parse_file(&path);
    assert_eq!(type_of(&script), "&swc_ecma_ast::module::Script");
}


#[test]
fn test_parse_code() {
    let code = "function foo() {}";
    let (script, _) = parser::parse_code(code);
    assert_eq!(type_of(&script), "&swc_ecma_ast::module::Script");
}
