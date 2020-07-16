use swc::ecmascript::ast;
use swc::common::Span;
use std::any::Any;
use serde_json;


pub fn print_as_json<T>(node: &T)
    where T: serde::ser::Serialize
{
    println!("{}", serde_json::to_string_pretty(node).unwrap());
}

pub fn as_value<T>(node: &T) -> serde_json::value::Value
    where T: serde::ser::Serialize
{
    serde_json::to_value(node).unwrap()
}


pub enum Syntax {
    Stmt(ast::Stmt),
    Expr(ast::Expr),
    Decl(ast::Decl),
    Script(ast::Script),
    Func(ast::Function),
    Callee(ast::Expr)
}

pub type Callback = Box<dyn Fn(&Syntax, Option<&Span>) -> Box<dyn Any>>;


pub fn audit_script(script: &ast::Script, callback: &Option<Callback>) -> Option<Box<dyn Any>> {
    audit_stmts(&script.body, callback);
    match callback {
        Some(f) => {
            Some(f(&Syntax::Script(script.clone()), Some(&script.span)))
        },
        None => None
    }
}


pub fn audit_stmts(stmts: &Vec<ast::Stmt>, callback: &Option<Callback>) {
    match &stmts[..] {
        [] => (),
        [xs@.., x] => {
            audit_stmt(x, callback);
            audit_stmts(&xs.to_vec(), callback)
        },
    }
}

pub fn audit_stmt(stmt: &ast::Stmt, callback: &Option<Callback>) {
    match stmt {
        ast::Stmt::Block(s) => audit_block(s, callback),
        ast::Stmt::With(s) => {
            audit_expr(&s.obj, callback);
            audit_stmt(&s.body, callback);
        },
        ast::Stmt::Return(s) => {
            if let Some(ref expr) = s.arg {
                audit_expr(expr, callback);
            }
        },
        ast::Stmt::Labeled(s) => {
            audit_stmt(&s.body, callback);
        },
        ast::Stmt::Break(_) => {
        },
        ast::Stmt::Continue(_) => {
        },
        ast::Stmt::If(s) => {
            audit_expr(&s.test, callback);
            audit_stmt(&s.cons, callback);
            if let Some(ref stmt) = s.alt {
                audit_stmt(stmt, callback);
            }

        },
        ast::Stmt::Switch(s) => {
            audit_expr(&s.discriminant, callback);
            for i in &s.cases {
                if let Some(ref expr) = i.test {
                    audit_expr(&expr, callback);
                }
            }
        },
        ast::Stmt::Throw(s) => {
            audit_expr(&s.arg, callback);
        },
        ast::Stmt::Try(s) => {
            audit_block(&s.block, callback);
            if let Some(ref h) = s.handler {
                if let Some(ref p) = h.param {
                    audit_pat(&p, callback);
                };
                audit_block(&h.body, callback);
            }
            if let Some(ref f) = s.finalizer {
                audit_block(&f, callback)
            }
        },
        ast::Stmt::While(s) => {
            audit_expr(&s.test, callback);
            audit_stmt(&s.body, callback);
        },
        ast::Stmt::DoWhile(s) => {
            audit_expr(&s.test, callback);
            audit_stmt(&s.body, callback);
        },
        ast::Stmt::For(s) => {
            if let Some(init) = &s.init {
                match init {
                    ast::VarDeclOrExpr::VarDecl(e) => {
                        audit_var_decl(&e, callback);
                    },
                    ast::VarDeclOrExpr::Expr(e) => {
                        audit_expr(&e, callback);
                    }
                }
            }
            if let Some(ref test) = s.test {
                audit_expr(test, callback);
            }
            if let Some(ref update) = s.update {
                audit_expr(update, callback);
            }
            audit_stmt(&s.body, callback);
        },
        ast::Stmt::ForIn(s) => {
            match s.left {
                ast::VarDeclOrPat::VarDecl(ref e) => {
                    audit_var_decl(&e, callback);
                },
                ast::VarDeclOrPat::Pat(ref e) => {
                    audit_pat(&e, callback);
                }
            }
            audit_expr(&s.right, callback);
            audit_stmt(&s.body, callback);
        },
        ast::Stmt::ForOf(s) => {
            match s.left {
                ast::VarDeclOrPat::VarDecl(ref e) => {
                    audit_var_decl(&e, callback);
                },
                ast::VarDeclOrPat::Pat(ref e) => {
                    audit_pat(&e, callback);
                }
            }
            audit_expr(&s.right, callback);
            audit_stmt(&s.body, callback);

        },
        ast::Stmt::Decl(s) => audit_decl(&s, callback),
        ast::Stmt::Expr(s) => {
            audit_expr(&s.expr, callback);
        },
        _ => ()
    }
}


fn audit_block(stmt: &ast::BlockStmt, callback: &Option<Callback>) {
    audit_stmts(&stmt.stmts, callback);
}

fn audit_pat(pat: &ast::Pat, callback: &Option<Callback>) {

    match pat {
        ast::Pat::Ident(_) => (),
        ast::Pat::Array(s) => {
            for e in &s.elems {
                if let Some(p) = e {
                    audit_pat(&p, callback);
                }
            }
        },
        ast::Pat::Rest(s) => {
            audit_pat(&s.arg, callback)
        },
        ast::Pat::Object(s) => {
            for o in &s.props {
                audit_obj_pat_prop(&o, callback);
            }
        },
        ast::Pat::Assign(s) => {
            audit_pat(&s.left, callback);
            audit_expr(&s.right, callback);
        },
        ast::Pat::Expr(s) => {
            audit_expr(&s, callback);
        },
        _ => ()
    }

}


fn audit_var_decl(decl: &ast::VarDecl, callback: &Option<Callback>) {
    for d in &decl.decls {
        audit_pat(&d.name, callback);
        if let Some(ref expr) = d.init {
            audit_expr(expr, callback);
        }
    }
}

fn audit_decl(decl: &ast::Decl, callback: &Option<Callback>) {
    match decl {
        ast::Decl::Class(s) => audit_class(&s.class, callback),
        ast::Decl::Fn(s) => audit_fn_decl(&s, callback),
        ast::Decl::Var(s) => audit_var_decl(&s, callback),
        ast::Decl::TsInterface(s) => {
            for e in &s.body.body {
                audit_ts_type_ele(&e, callback);
            }
        },
        ast::Decl::TsTypeAlias(_) => (),
        ast::Decl::TsEnum(s) => {
            for m in &s.members {
                if let Some(expr) = &m.init {
                    audit_expr(&expr, callback);
                }
            }
        },
        ast::Decl::TsModule(s) => {
            if let Some(ns) = &s.body {
                audit_ts_namespace(&ns, callback);
            }
        }
    }
}

fn audit_obj_pat_prop(prop: &ast::ObjectPatProp, callback: &Option<Callback>) {
    match prop {
        ast::ObjectPatProp::KeyValue(s) => {
            audit_pat(&s.value, callback);
        },
        ast::ObjectPatProp::Assign(s) => {
            if let Some(v) = &s.value {
                audit_expr(&v, callback);
            }
        },
        ast::ObjectPatProp::Rest(s) => {
            audit_pat(&s.arg, callback);
        },
    }
}


fn audit_class(cls: &ast::Class, callback: &Option<Callback>) {
    for d in &cls.decorators {
        audit_expr(&d.expr, callback);
    }
    for d in &cls.body {
        audit_class_member(&d, callback);
    }
    if let Some(expr) = &cls.super_class {
        audit_expr(&expr, callback);
    }
}

fn audit_class_member(cls_m: &ast::ClassMember,  callback: &Option<Callback>) {
    match cls_m {
        ast::ClassMember::Constructor(s) => {
            audit_prop_name(&s.key, callback);
            for p in &s.params {
                audit_param_prop(&p, callback);
            }
            if let Some(ref body) = s.body {
                audit_block(&body, callback);
            }

        },
        ast::ClassMember::Method(s) => {
            audit_prop_name(&s.key, callback);
            audit_fn(&s.function, callback);
        },
        ast::ClassMember::PrivateMethod(s) => {
            audit_fn(&s.function, callback);
        },
        ast::ClassMember::ClassProp(s) => {
            audit_expr(&s.key, callback);
            if let Some(ref expr) = s.value {
                audit_expr(&expr, callback);
            }
            for d in &s.decorators {
                audit_expr(&d.expr, callback);
            }
        },
        ast::ClassMember::PrivateProp(s) => {
            if let Some(ref expr) = s.value {
                audit_expr(&expr, callback);
            }
            for d in &s.decorators {
                audit_expr(&d.expr, callback);
            }

        },
        ast::ClassMember::TsIndexSignature(s) => {
            for p in &s.params {
                audit_ts_fn_param(&p, callback);
            }
        }
    }
}

fn audit_ts_fn_param(param: &ast::TsFnParam, callback: &Option<Callback>) {
    match param {
        ast::TsFnParam::Ident(s) => {
            audit_pat(&ast::Pat::from(s.clone()), callback);
        },
        ast::TsFnParam::Array(s) => {
            audit_pat(&ast::Pat::from(s.clone()), callback);
        },
        ast::TsFnParam::Rest(s) => {
             audit_pat(&ast::Pat::from(s.clone()), callback);

        },
        ast::TsFnParam::Object(s) => {
            audit_pat(&ast::Pat::from(s.clone()), callback);
        },
    }

}

fn audit_prop_name(name: &ast::PropName, callback: &Option<Callback>) {
    match name {
        ast::PropName::Computed(e) => {
            audit_expr(&e.expr, callback);
        },
        _ => ()
    }
}


fn audit_param_prop(prop: &ast::ParamOrTsParamProp, callback: &Option<Callback>) {
    match prop {
        ast::ParamOrTsParamProp::TsParamProp(s) => {
            for d in &s.decorators {
                audit_expr(&d.expr, callback);
            }
            match s.param {
                ast::TsParamPropParam::Ident(_) => (),
                ast::TsParamPropParam::Assign(ref p) => {
                    audit_pat(&ast::Pat::from(p.clone()), callback);
                },
            }
        },
        ast::ParamOrTsParamProp::Param(s) => audit_param(&s, callback)
    }
}


fn audit_param(param: &ast::Param, callback: &Option<Callback>) {
    for d in &param.decorators {
        audit_expr(&d.expr, callback);
    }
    audit_pat(&param.pat, callback);
}

fn audit_fn(func: &ast::Function, callback: &Option<Callback>) {
    for p in &func.params {
        audit_param(&p, callback);
    }
    for d in &func.decorators {
        audit_expr(&d.expr, callback);
    }
    if let Some(ref stmt) = func.body {
        audit_block(&stmt, callback);
    }
    if let Some(f) = callback {
        f(&Syntax::Func(func.clone()), Some(&func.span));
    }
}

fn audit_ts_type_ele(ele: &ast::TsTypeElement, callback: &Option<Callback>) {
    match &ele {
        ast::TsTypeElement::TsCallSignatureDecl(s) => {
            for p in &s.params {
                audit_ts_fn_param(&p, callback);
            }
        },
        ast::TsTypeElement::TsConstructSignatureDecl(s) => {
            for p in &s.params {
                audit_ts_fn_param(&p, callback);
            }
        },
        ast::TsTypeElement::TsPropertySignature(s) => {
            audit_expr(&s.key, callback);
            if let Some(expr) = &s.init {
                audit_expr(&expr, callback)
            }
            for p in &s.params {
                audit_ts_fn_param(&p, callback);
            }
        },
        ast::TsTypeElement::TsMethodSignature(s) => {
            audit_expr(&s.key, callback);
            for p in &s.params {
                audit_ts_fn_param(&p, callback);
            }
        },
        ast::TsTypeElement::TsIndexSignature(s) => {
            for p in &s.params {
                audit_ts_fn_param(&p, callback);
            }

        },
    }
}

fn audit_ts_namespace(ns: &ast::TsNamespaceBody, callback: &Option<Callback>) {
    match &ns {
        ast::TsNamespaceBody::TsModuleBlock(s) => {
            for m in &s.body {
                match &m {
                    ast::ModuleItem::ModuleDecl(s) => audit_module_decl(&s),
                    ast::ModuleItem::Stmt(s) => audit_stmt(&s, callback)
                }
            }
        },
        ast::TsNamespaceBody::TsNamespaceDecl(s) => {
            audit_ts_namespace(&s.body, callback);
        }
    }
}

pub fn audit_expr(expr: &ast::Expr, callback: &Option<Callback>) {
    match &expr {
        ast::Expr::This(_) => (),
        ast::Expr::Array(s) => {
            for e in &s.elems {
                if let Some(eos) = &e {
                    audit_expr(&eos.expr, callback);
                }
            }
        },
        ast::Expr::Object(s) => {
            for e in &s.props {
                match e {
                    ast::PropOrSpread::Spread(s) => audit_expr(&s.expr, callback),
                    ast::PropOrSpread::Prop(s) => audit_prop(&s, callback)
                }
             }
        },
        ast::Expr::Fn(s) => audit_fn(&s.function, callback),
        ast::Expr::Unary(s) => audit_expr(&s.arg, callback),
        ast::Expr::Update(s) => audit_expr(&s.arg,callback),
        ast::Expr::Bin(s) => {
            audit_expr(&s.left, callback);
            audit_expr(&s.right, callback);
        },
        ast::Expr::Assign(s) => {
            match s.left {
                ast::PatOrExpr::Expr(ref s) => audit_expr(&s, callback),
                ast::PatOrExpr::Pat(ref s) => audit_pat(&s, callback)
            }
            audit_expr(&s.right, callback);

        },
        ast::Expr::Member(s) => {
            match s.obj {
                ast::ExprOrSuper::Expr(ref s) => audit_expr(&s, callback),
                ast::ExprOrSuper::Super(_) => ()
            }
        },
        ast::Expr::Cond(s) => {
            audit_expr(&s.test, callback);
            audit_expr(&s.cons, callback);
            audit_expr(&s.alt, callback);
        },
        ast::Expr::Call(s) => {
            audit_call(&s, callback);
        },
        ast::Expr::New(s) => {
            audit_callee(&s.span, &s.callee, callback);
            if let Some(args) = &s.args {
                for a in args {
                    audit_expr(&a.expr, callback);
                }
            }
        },
        ast::Expr::Seq(s) => {
            for expr in &s.exprs {
                audit_expr(&expr, callback);
            }
        },
        ast::Expr::Ident(_) => (),
        ast::Expr::Lit(_) => (),
        ast::Expr::Tpl(s) => {
            for expr in &s.exprs {
                audit_expr(&expr, callback);
            }
        },
        ast::Expr::TaggedTpl(s) => {
            audit_expr(&s.tag, callback);
            for expr in &s.exprs {
                audit_expr(&expr, callback);
            }
        },
        ast::Expr::Arrow(s) => {
            for p in &s.params {
                audit_pat(&p, callback)
            }
            match &s.body {
                ast::BlockStmtOrExpr::BlockStmt(s) => audit_block(&s, callback),
                ast::BlockStmtOrExpr::Expr(s) => audit_expr(&s, callback),
            }
        },
        ast::Expr::Class(s) => {
            audit_class(&s.class, callback);
        },
        ast::Expr::Yield(s) => {
            if let Some(i) = &s.arg {
                audit_expr(&i, callback);
            }
        },
        ast::Expr::MetaProp(_) => (),
        ast::Expr::Await(s) => audit_expr(&s.arg, callback),
        ast::Expr::Paren(s) => audit_expr(&s.expr, callback),
        ast::Expr::JSXMember(_) => {
            panic!("JSX is not supported");
        },
        ast::Expr::JSXNamespacedName(_) => {
            panic!("JSX is not supported");
        },
        ast::Expr::JSXEmpty(_) => (),
        ast::Expr::JSXElement(_) => {
            panic!("JSX is not supported");
        },
        ast::Expr::JSXFragment(_) => {
            panic!("JSX is not supported");
        },
        ast::Expr::TsTypeAssertion(s) => audit_expr(&s.expr, callback),
        ast::Expr::TsConstAssertion(s) => audit_expr(&s.expr, callback),
        ast::Expr::TsNonNull(s) => audit_expr(&s.expr, callback),
        ast::Expr::TsTypeCast(s) => audit_expr(&s.expr, callback),
        ast::Expr::TsAs(s) => audit_expr(&s.expr, callback),
        ast::Expr::PrivateName(_) => (),
        ast::Expr::OptChain(s) => audit_expr(&s.expr, callback),
        ast::Expr::Invalid(_) => ()
    }

}

pub fn audit_prop(prop: &ast::Prop, callback: &Option<Callback>) {
    match prop {
        ast::Prop::Shorthand(_) => (),
        ast::Prop::KeyValue(s) => {
            audit_expr(&s.value, callback);
            match s.key {
                ast::PropName::Computed(ref s) => audit_expr(&s.expr, callback),
                _ => ()
            }
        },
        ast::Prop::Assign(s) => audit_expr(&s.value, callback),
        ast::Prop::Getter(s) => {
            match s.key {
                ast::PropName::Computed(ref s) => audit_expr(&s.expr, callback),
                _ => ()
            }
            if let Some(ref body) = s.body {
                audit_block(&body, callback);
            }
        },
        ast::Prop::Setter(s) => {
            audit_pat(&s.param, callback);
            match &s.key {
                ast::PropName::Computed(s) => audit_expr(&s.expr, callback),
                _ => ()
            }
            if let Some(body) = &s.body {
                audit_block(&body, callback);
            }

        },
        ast::Prop::Method(s) => {
            match &s.key {
                ast::PropName::Computed(s) => audit_expr(&s.expr, callback),
                _ => ()
            }
            audit_fn(&s.function, callback);
        },
    }

}

pub fn audit_module_decl(_decl: &ast::ModuleDecl) {
    ()
}


pub fn audit_call(call: &ast::CallExpr, callback: &Option<Callback>) {
    match &call.callee {
        ast::ExprOrSuper::Expr(s) => audit_callee(&call.span, &s, callback),
        ast::ExprOrSuper::Super(_) => ()
    }
    for i in &call.args {
        audit_expr(&i.expr, callback)
    }
    if let Some(f) = callback {
        f(&Syntax::Expr(ast::Expr::Call(call.clone())), Some(&call.span));
    }
}


pub fn audit_fn_decl(decl: &ast::FnDecl, callback: &Option<Callback>) {
    audit_fn(&decl.function, callback);
    if let Some(f) = callback {
        f(&Syntax::Decl(ast::Decl::Fn(decl.clone())), None);
    }
}


pub fn audit_callee(locate: &Span, expr: &ast::Expr, callback: &Option<Callback>) {
    audit_expr(&expr, callback);
    if let Some(f) = callback {
        f(&Syntax::Callee(expr.clone()), Some(locate));
    }

}
