use std::env;
use std::path::Path;
use asteroid::typescript::ast;
use asteroid::typescript::parser;
use asteroid::events::{CallbackMap};


fn main() {
    let args: Vec<String> = env::args().collect();
    let script = parser::parse_file(Path::new(&args[1]));
    let map = CallbackMap::new();
    ast::audit_script(&script, Some(&map));
}
