use std::env;
use std::any::Any;
use std::path::Path;
use swc::ecmascript::ast;
use asteroid::typescript::ast::Syntax;
use asteroid::typescript::ast::audit_script;
use asteroid::typescript::parser;
use asteroid::typescript::ast::print_as_json;
use asteroid::typescript::ast::as_value;
use swc::common::AstNode;
use swc::common::Span;


fn callback(syn: &Syntax, locate: Option<&Span>) {
    match syn {
        Syntax::Callee(expr) => {
            match expr {
                ast::Expr::Ident(i) => {
                    let call_type = "PureCallee";
                    println!("[{:?}{:?}], {} {}", i.span.lo(), i.span.hi(), call_type, i.sym.to_ascii_lowercase());
                },
                ast::Expr::Member(i) => {
                    let call_type = "MemberCallee";
                    if let ast::ExprOrSuper::Expr(e) = &i.obj {
                        match &**e {
                            ast::Expr::Ident(id) => {
                                if let ast::Expr::Ident(prop) = &*i.prop {
                                    println!("[{:?}{:?}], {} {}.{}", id.span.lo(), id.span.hi(), call_type, id.sym.to_ascii_lowercase(), prop.sym.to_ascii_lowercase());
                                } else {
                                    println!("Invalid nested Expr");
                                }
                            },
                            _ => {
                                println!("Invalid nested Expr");
                            }
                        }
                    }                },
                _ => {
                    let call_type = "UnAuditCallee";
                    if let Some(l) = locate {
                        println!("[{:?}{:?}], {}", l.lo(), l.hi(), call_type);
                    } else {
                        println!("[?, ?], {}", call_type);
                    }
                    print_as_json(expr);
                }
            }
        }
        _ => ()
    }
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let script = parser::parse_file(Path::new(&args[1]));
    audit_script(&script, Some(callback));
}
