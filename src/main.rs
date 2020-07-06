use std::env;
use std::path::Path;
use asteroid::typescript::ast;
use asteroid::typescript::parser;


fn main() {
    let args: Vec<String> = env::args().collect();
    let script = parser::parse_file(Path::new(&args[1]));
    ast::audit_script(&script);
}
