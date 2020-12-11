use std::sync::Arc;
use std::cell::{RefCell, RefMut};
use std::path::Path;
use swc::ecmascript::ast;
use swc::common::Span;
use swc::common::SourceMap;
use serde_json::json;
use serde_json::to_value;
use crate::typescript::ast::Syntax;
use crate::typescript::ast::Callback;
use crate::typescript::ast::audit_script;
use crate::typescript::parser;
use crate::report::{self, Reporter, Source};
use std::os::raw::c_char;
use std::ffi::CString;
use std::ffi::CStr;




pub fn callback(
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

#[no_mangle]
pub extern "C" fn report(p: *const c_char) -> *mut c_char {
    let path = unsafe {
        assert!(!p.is_null());
        CStr::from_ptr(p)
    };
    let reporter: RefCell<Reporter> = RefCell::new(Reporter::new());
    let (script, cm) = parser::parse_file(Path::new(&path.to_str().unwrap()));
    let callback_wrapper: Callback = box (move |x, y| {
        callback(x, y, reporter.borrow_mut(), cm.clone());
        box reporter.clone()
    });
    match audit_script(&script, &Some(callback_wrapper)) {
        Some(ret) => {
            let rep: Option<&RefCell<Reporter>> = ret.downcast_ref::<RefCell<Reporter>>();
            if let Some(r) = rep {
                return CString::new(r.borrow().to_json()).unwrap().into_raw();
            } else {
                panic!("failed on downcast");
            }
        },
        None => {
            panic!("Callback Should return Rporter");
        }
    }
}
