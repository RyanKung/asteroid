#![warn(non_upper_case_globals)]

use std::env;
use std::sync::Mutex;
use std::path::Path;
use swc::ecmascript::ast;
use swc::common::Span;
use serde_json::json;
use serde_json::to_value;
use lazy_static::lazy_static;
use asteroid::typescript::ast::Syntax;
use asteroid::typescript::ast::audit_script;
use asteroid::typescript::parser;
use asteroid::report::{self, Reporter};


lazy_static! {
    static ref reporter: Mutex<Reporter> = Mutex::new(Reporter::new());
}


fn callback(syn: &Syntax, locate: Option<&Span>) {
    match syn {
        Syntax::Callee(expr) => {
            match expr {
                ast::Expr::Ident(i) => {
                    reporter.lock().unwrap().report(
                        report::Kind::FnCall,
                        Some(i.span),
                        report::Level::Default,
                        to_value(expr).unwrap(),
                        json!({
                            "call_type": "PureCallee",
                            "func_name": i.sym.to_ascii_lowercase()
                        }),
                        None,
                        None
                    )
                },
                ast::Expr::Member(i) => {
                    if let ast::ExprOrSuper::Expr(e) = &i.obj {
                        match &**e {
                            ast::Expr::Ident(id) => {
                                if let ast::Expr::Ident(prop) = &*i.prop {
                                    reporter.lock().unwrap().report(
                                        report::Kind::MethodCall,
                                        Some(id.span),
                                        report::Level::Default,
                                        to_value(expr).unwrap(),
                                        json!({
                                            "call_type": "MemberCallee",
                                            "func_name": format!(
                                                "{}.{}",
                                                id.sym.to_ascii_lowercase(),
                                                prop.sym.to_ascii_lowercase()
                                            )
                                        }),
                                        None,
                                        None
                                    )
                                } else {
                                    reporter.lock().unwrap().report(
                                        report::Kind::MethodCall,
                                        Some(id.span),
                                        report::Level::Warning,
                                        to_value(expr).unwrap(),
                                        json!({
                                            "call_type": "MemberCallee",
                                            "func_name": "UnknowYet!"
                                        }),
                                        None,
                                        Some("Invalid Nested Expr".to_string())
                                    );
                                }
                            },
                            _ => {
                                reporter.lock().unwrap().report(
                                    report::Kind::MethodCall,
                                    locate.map(|a| *a),
                                    report::Level::Warning,
                                    to_value(expr).unwrap(),
                                    json!({
                                        "call_type": "MemberCallee",
                                        "func_name": "UnknowYet!"
                                    }),
                                    None,
                                    Some("Invalid Nested Expr".to_string())
                                );
                            }

                        }
                    } else {
                        if let ast::Expr::Ident(id) = &*i.prop {
                            reporter.lock().unwrap().report(
                                report::Kind::MethodCall,
                                Some(id.span),
                                report::Level::Warning,
                                to_value(expr).unwrap(),
                                json!({
                                    "call_type": "MemberCallee",
                                    "func_name": format!(
                                        "Super.{}",
                                        id.sym.to_ascii_lowercase()
                                    )
                                }),
                                None,
                                Some("SuperCall of unknow obj".to_string())
                            );
                        }
                    }
                },
                _ => {
                    reporter.lock().unwrap().report(
                        report::Kind::MethodCall,
                        locate.map(|a| *a),
                        report::Level::Critical,
                        to_value(expr).unwrap(),
                        json!({
                            "call_type": "UnAuditedCallee",
                            "func_name": "UnKnowYet!"
                        }),
                        None,
                        None
                    );
               }
            }
        }
        _ => ()
    }
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let script = parser::parse_file(Path::new(&args[1]));
    audit_script(&script, &Some(Box::new(callback)));
    println!("{}", reporter.lock().unwrap().to_json());
}
