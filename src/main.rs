#![warn(non_upper_case_globals)]
#![feature(box_syntax)]
#![feature(box_patterns)]
use std::env;
use std::sync::Arc;
use std::cell::{RefCell, RefMut};
use std::path::Path;
use swc::ecmascript::ast;
use swc::common::Span;
use swc::common::SourceMap;
use serde_json::json;
use serde_json::to_value;
use asteroid::typescript::ast::Syntax;
use asteroid::typescript::ast::Callback;
use asteroid::typescript::ast::audit_script;
use asteroid::typescript::parser;
use asteroid::report::{self, Reporter, Source};


fn callback(
    syn: &Syntax,
    locate: Option<&Span>,
    mut reporter: RefMut<Reporter>,
    cm: Arc<SourceMap>
) {
    match syn {
        Syntax::Callee(expr) => {
            match expr {
                ast::Expr::Ident(i) => {
                    reporter.report(
                        report::Kind::FnCall,
                        Some(i.span),
                        report::Level::Default,
                        to_value(expr).unwrap(),
                        json!({
                            "call_type": "PureCallee",
                            "func_name": i.sym.to_ascii_lowercase()
                        }),
                        {
                            match locate {
                                Some(l) => {
                                    Source::new(l, cm)
                                },
                                None => None
                            }
                        },
                        None,
                        None
                    )
                },
                ast::Expr::Member(i) => {
                    if let ast::ExprOrSuper::Expr(e) = &i.obj {
                        match e {
                            box ast::Expr::Ident(id) => {
                                if let ast::Expr::Ident(prop) = &*i.prop {
                                    reporter.report(
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
                                        {
                                            match locate {
                                                Some(l) => {
                                                    Source::new(l, cm)
                                                },
                                                None => None
                                            }
                                        },
                                        None,
                                        None
                                    )
                                } else {
                                    reporter.report(
                                        report::Kind::MethodCall,
                                        Some(id.span),
                                        report::Level::Warning,
                                        to_value(expr).unwrap(),
                                        json!({
                                            "call_type": "MemberCallee",
                                            "func_name": "UnknowYet!"
                                        }),
                                        {
                                            match locate {
                                                Some(l) => {
                                                    Source::new(l, cm)
                                                },
                                                None => None
                                            }
                                        },
                                        None,
                                        Some("Invalid Nested Expr".to_string())
                                    );
                                }
                            },
                            _ => {
                                reporter.report(
                                    report::Kind::MethodCall,
                                    locate.map(|a| *a),
                                    report::Level::Warning,
                                    to_value(expr).unwrap(),
                                    json!({
                                        "call_type": "MemberCallee",
                                        "func_name": "UnknowYet!"
                                    }),
                                    {
                                        match locate {
                                            Some(l) => {
                                                Source::new(l, cm)
                                            },
                                            None => None
                                        }
                                    },
                                    None,
                                    Some("Invalid Nested Expr".to_string())
                                );
                            }

                        }
                    } else {
                        if let ast::Expr::Ident(id) = &*i.prop {
                            reporter.report(
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
                                None,
                                Some("SuperCall of unknow obj".to_string())
                            );
                        }
                    }
                },
                _ => {
                    reporter.report(
                        report::Kind::MethodCall,
                        locate.map(|a| *a),
                        report::Level::Critical,
                        to_value(expr).unwrap(),
                        json!({
                            "call_type": "UnAuditedCallee",
                            "func_name": "UnKnowYet!"
                        }),
                        {
                            match locate {
                                Some(l) => {
                                    Source::new(l, cm)
                                },
                                None => None
                            }
                        },
                        None,
                        None
                    );
               }
            }
        },
        _ => ()
    }
}


fn main() {
    let reporter: RefCell<Reporter> = RefCell::new(Reporter::new());
    let args: Vec<String> = env::args().collect();
    let (script, cm) = parser::parse_file(Path::new(&args[1]));
    let callback_wrapper: Callback = box (move |x, y| {
        callback(x, y, reporter.borrow_mut(), cm.clone());
        box reporter.clone()
    });
    match audit_script(&script, &Some(callback_wrapper)) {
        Some(ret) => {
            let rep: Option<&RefCell<Reporter>> = ret.downcast_ref::<RefCell<Reporter>>();
            if let Some(r) = rep {
                println!("{}", r.borrow().to_json());
            } else {
                panic!("failed on downcast")
            }
        },
        None => {
            panic!("Callback Should return Rporter")
        }
    }
}
