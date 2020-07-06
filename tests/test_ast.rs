use asteroid::typescript::ast;
use asteroid::typescript::parser;


#[test]
fn test_audit_script() {
    let code = "function foo() {};\n foo()";
    let script = parser::parse_code(code);
    ast::audit_script(&script)
}
