use std::collections::HashMap;
use swc::ecmascript::ast;


pub enum Syntax {
    Stmt(ast::Stmt),
    Expr(ast::Expr)
}

pub type Callback = fn(&Syntax) -> ();
pub type CallbackMap = HashMap<String, Callback>;
