use swc::ecmascript::ast;
use swc::common::Span;
use crate::events::{CallbackMap, Syntax};


pub fn audit_script(script: &ast::Script, map: Option<&CallbackMap>) {
    audit_stmts(&script.body, map);
}


pub fn audit_stmts(stmts: &Vec<ast::Stmt>, map: Option<&CallbackMap>) {
    for stmt in stmts {
        audit_stmt(&stmt, map);
    }
}

pub fn audit_stmt(stmt: &ast::Stmt, map: Option<&CallbackMap>) {
    match stmt {
        ast::Stmt::Block(s) => audit_block(s, map),
        ast::Stmt::With(s) => {
            audit_expr(&s.obj, map);
            audit_stmt(&s.body, map);
        },
        ast::Stmt::Return(s) => {
            if let Some(expr) = &s.arg {
                audit_expr(expr, map);
            }
        },
        ast::Stmt::Labeled(s) => {
            audit_stmt(&s.body, map);
        },
        ast::Stmt::Break(_) => {
        },
        ast::Stmt::Continue(_) => {
        },
        ast::Stmt::If(s) => {
            audit_expr(&s.test, map);
            audit_stmt(&s.cons, map);
            if let Some(stmt) = &s.alt {
                audit_stmt(stmt, map);
            }

        },
        ast::Stmt::Switch(s) => {
            audit_expr(&s.discriminant, map);
            for i in &s.cases {
                if let Some(expr) = &i.test {
                    audit_expr(&expr, map);
                }
            }
        },
        ast::Stmt::Throw(s) => {
            audit_expr(&s.arg, map);
        },
        ast::Stmt::Try(s) => {
            audit_block(&s.block, map);
            if let Some(h) = &s.handler {
                if let Some(p) = &h.param {
                    audit_pat(&p, map);
                };
                audit_block(&h.body, map);
            }
            if let Some(f) = &s.finalizer {
                audit_block(&f, map)
            }
        },
        ast::Stmt::While(s) => {
            audit_expr(&s.test, map);
            audit_stmt(&s.body, map);
        },
        ast::Stmt::DoWhile(s) => {
            audit_expr(&s.test, map);
            audit_stmt(&s.body, map);
        },
        ast::Stmt::For(s) => {
            if let Some(init) = &s.init {
                match init {
                    ast::VarDeclOrExpr::VarDecl(e) => {
                        audit_var_decl(&e, map);
                    },
                    ast::VarDeclOrExpr::Expr(e) => {
                        audit_expr(&e, map);
                    }
                }
            }
            if let Some(test) = &s.test {
                audit_expr(test, map);
            }
            if let Some(update) = &s.update {
                audit_expr(update, map);
            }
            audit_stmt(&s.body, map);
        },
        ast::Stmt::ForIn(s) => {
            match &s.left {
                ast::VarDeclOrPat::VarDecl(e) => {
                    audit_var_decl(&e, map);
                },
                ast::VarDeclOrPat::Pat(e) => {
                    audit_pat(&e, map);
                }
            }
            audit_expr(&s.right, map);
            audit_stmt(&s.body, map);
        },
        ast::Stmt::ForOf(s) => {
            match &s.left {
                ast::VarDeclOrPat::VarDecl(e) => {
                    audit_var_decl(&e, map);
                },
                ast::VarDeclOrPat::Pat(e) => {
                    audit_pat(&e, map);
                }
            }
            audit_expr(&s.right, map);
            audit_stmt(&s.body, map);

        },
        ast::Stmt::Decl(s) => audit_decl(&s, map),
        ast::Stmt::Expr(s) => {
            audit_expr(&s.expr, map);
        },
        _ => ()
    }
}


fn audit_block(stmt: &ast::BlockStmt, map: Option<&CallbackMap>) {
    audit_stmts(&stmt.stmts, map);
}

fn audit_pat(pat: &ast::Pat, map: Option<&CallbackMap>) {
    match pat {
        ast::Pat::Ident(_) => (),
        ast::Pat::Array(s) => {
            for e in &s.elems {
                if let Some(p) = e {
                    audit_pat(&p, map);
                }
            }
        },
        ast::Pat::Rest(s) => {
            audit_pat(&s.arg, map)
        },
        ast::Pat::Object(s) => {
            for o in &s.props {
                audit_obj_pat_prop(&o, map);
            }
        },
        ast::Pat::Assign(s) => {
            audit_pat(&s.left, map);
            audit_expr(&s.right, map);
        },
        ast::Pat::Expr(s) => {
            audit_expr(&s, map);
        },
        _ => ()
    }

}


fn audit_var_decl(decl: &ast::VarDecl, map: Option<&CallbackMap>) {
    for d in &decl.decls {
        if let Some(expr) = &d.init {
            audit_expr(expr, map)
        }
    }
}

fn audit_decl(decl: &ast::Decl, map: Option<&CallbackMap>) {
    match decl {
        ast::Decl::Class(s) => {
            audit_class(&s.class, map);
        },
        ast::Decl::Fn(s) => {
            audit_fn(&s.function, map);
        },
        ast::Decl::Var(s) => {
            for v in &s.decls {
                if let Some(expr) = &v.init {
                    audit_expr(&expr, map)
                }
            }
        },
        ast::Decl::TsInterface(s) => {
            for e in &s.body.body {
                audit_ts_type_ele(&e, map);
            }
        },
        ast::Decl::TsTypeAlias(_) => (),
        ast::Decl::TsEnum(s) => {
            for m in &s.members {
                if let Some(expr) = &m.init {
                    audit_expr(&expr, map);
                }
            }
        },
        ast::Decl::TsModule(s) => {
            if let Some(ns) = &s.body {
                audit_ts_namespace(&ns, map);
            }
        }
    }
}

fn audit_obj_pat_prop(prop: &ast::ObjectPatProp, map: Option<&CallbackMap>) {
    match prop {
        ast::ObjectPatProp::KeyValue(s) => {
            audit_pat(&s.value, map);
        },
        ast::ObjectPatProp::Assign(s) => {
            if let Some(v) = &s.value {
                audit_expr(&v, map);
            }
        },
        ast::ObjectPatProp::Rest(s) => {
            audit_pat(&s.arg, map);
        },
    }
}


fn audit_class(cls: &ast::Class, map: Option<&CallbackMap>) {
    for d in &cls.decorators {
        audit_expr(&d.expr, map);
    }
    for d in &cls.body {
        audit_class_member(&d, map);
    }
    if let Some(expr) = &cls.super_class {
        audit_expr(&expr, map);
    }
}

fn audit_class_member(cls_m: &ast::ClassMember,  map: Option<&CallbackMap>) {
    match cls_m {
        ast::ClassMember::Constructor(s) => {
            audit_prop_name(&s.key, map);
            for p in &s.params {
                audit_param_prop(&p, map);
            }
            if let Some(body) = &s.body {
                audit_block(&body, map);
            }

        },
        ast::ClassMember::Method(s) => {
            audit_prop_name(&s.key, map);
            audit_fn(&s.function, map);
        },
        ast::ClassMember::PrivateMethod(s) => {
            audit_fn(&s.function, map);
        },
        ast::ClassMember::ClassProp(s) => {
            audit_expr(&s.key, map);
            if let Some(expr) = &s.value {
                audit_expr(&expr, map);
            }
            for d in &s.decorators {
                audit_expr(&d.expr, map);
            }
        },
        ast::ClassMember::PrivateProp(s) => {
            if let Some(expr) = &s.value {
                audit_expr(&expr, map);
            }
            for d in &s.decorators {
                audit_expr(&d.expr, map);
            }

        },
        ast::ClassMember::TsIndexSignature(s) => {
            for p in &s.params {
                audit_ts_fn_param(&p, map);
            }
        }
    }
}

fn audit_ts_fn_param(param: &ast::TsFnParam, map: Option<&CallbackMap>) {
    match param {
        ast::TsFnParam::Ident(s) => {
            audit_pat(&ast::Pat::from(s.clone()), map);
        },
        ast::TsFnParam::Array(s) => {
            audit_pat(&ast::Pat::from(s.clone()), map);
        },
        ast::TsFnParam::Rest(s) => {
             audit_pat(&ast::Pat::from(s.clone()), map);

        },
        ast::TsFnParam::Object(s) => {
            audit_pat(&ast::Pat::from(s.clone()), map);
        },
    }

}

fn audit_prop_name(name: &ast::PropName, map: Option<&CallbackMap>) {
    match name {
        ast::PropName::Computed(e) => {
            audit_expr(&e.expr, map);
        },
        _ => ()
    }
}


fn audit_param_prop(prop: &ast::ParamOrTsParamProp, map: Option<&CallbackMap>) {
    match prop {
        ast::ParamOrTsParamProp::TsParamProp(s) => {
            for d in &s.decorators {
                audit_expr(&d.expr, map);
            }
            match &s.param {
                ast::TsParamPropParam::Ident(_) => (),
                ast::TsParamPropParam::Assign(p) => {
                    audit_pat(&ast::Pat::from(p.clone()), map);
                },
            }
        },
        ast::ParamOrTsParamProp::Param(s) => audit_param(&s, map)
    }
}


fn audit_param(param: &ast::Param, map: Option<&CallbackMap>) {
    for d in &param.decorators {
        audit_expr(&d.expr, map);
    }
    audit_pat(&param.pat, map);
}

fn audit_fn(func: &ast::Function, map: Option<&CallbackMap>) {
    for p in &func.params {
        audit_param(&p, map);
    }
    for d in &func.decorators {
        audit_expr(&d.expr, map);
    }
    if let Some(stmt) = &func.body {
        audit_block(&stmt, map);
    }
}

fn audit_ts_type_ele(ele: &ast::TsTypeElement, map: Option<&CallbackMap>) {
    match &ele {
        ast::TsTypeElement::TsCallSignatureDecl(s) => {
            for p in &s.params {
                audit_ts_fn_param(&p, map);
            }
        },
        ast::TsTypeElement::TsConstructSignatureDecl(s) => {
            for p in &s.params {
                audit_ts_fn_param(&p, map);
            }
        },
        ast::TsTypeElement::TsPropertySignature(s) => {
            audit_expr(&s.key, map);
            if let Some(expr) = &s.init {
                audit_expr(&expr, map)
            }
            for p in &s.params {
                audit_ts_fn_param(&p, map);
            }
        },
        ast::TsTypeElement::TsMethodSignature(s) => {
            audit_expr(&s.key, map);
            for p in &s.params {
                audit_ts_fn_param(&p, map);
            }
        },
        ast::TsTypeElement::TsIndexSignature(s) => {
            for p in &s.params {
                audit_ts_fn_param(&p, map);
            }

        },
    }
}

fn audit_ts_namespace(ns: &ast::TsNamespaceBody, map: Option<&CallbackMap>) {
    match &ns {
        ast::TsNamespaceBody::TsModuleBlock(s) => {
            for m in &s.body {
                match &m {
                    ast::ModuleItem::ModuleDecl(s) => audit_module_decl(&s),
                    ast::ModuleItem::Stmt(s) => audit_stmt(&s, map)
                }
            }
        },
        ast::TsNamespaceBody::TsNamespaceDecl(s) => {
            audit_ts_namespace(&s.body, map);
        }
    }
}

pub fn audit_expr(expr: &ast::Expr, map: Option<&CallbackMap>) {
    match &expr {
        ast::Expr::This(_) => (),
        ast::Expr::Array(s) => {
            for e in &s.elems {
                if let Some(eos) = &e {
                    audit_expr(&eos.expr, map);
                }
            }
        },
        ast::Expr::Object(s) => {
            for e in &s.props {
                match e {
                    ast::PropOrSpread::Spread(s) => audit_expr(&s.expr, map),
                    ast::PropOrSpread::Prop(s) => audit_prop(&s, map)
                }
             }
        },
        ast::Expr::Fn(s) => audit_fn(&s.function, map),
        ast::Expr::Unary(s) => audit_expr(&s.arg, map),
        ast::Expr::Update(s) => audit_expr(&s.arg,map),
        ast::Expr::Bin(s) => {
            audit_expr(&s.left, map);
            audit_expr(&s.right, map);
        },
        ast::Expr::Assign(s) => {
            match &s.left {
                ast::PatOrExpr::Expr(s) => audit_expr(&s, map),
                ast::PatOrExpr::Pat(s) => audit_pat(&s, map)
            }
            audit_expr(&s.right, map);

        },
        ast::Expr::Member(s) => {
            match &s.obj {
                ast::ExprOrSuper::Expr(s) => audit_expr(&s, map),
                ast::ExprOrSuper::Super(_) => ()
            }
        },
        ast::Expr::Cond(s) => {
            audit_expr(&s.test, map);
            audit_expr(&s.cons, map);
            audit_expr(&s.alt, map);
        },
        ast::Expr::Call(s) => {
            audit_call(&s, None);
        },
        ast::Expr::New(s) => {
            audit_expr(&s.callee, map);
            if let Some(args) = &s.args {
                for a in args {
                    audit_expr(&a.expr, map);
                }
            }
        },
        ast::Expr::Seq(s) => {
            for expr in &s.exprs {
                audit_expr(&expr, map);
            }
        },
        ast::Expr::Ident(_) => (),
        ast::Expr::Lit(_) => (),
        ast::Expr::Tpl(s) => {
            for expr in &s.exprs {
                audit_expr(&expr, map);
            }
        },
        ast::Expr::TaggedTpl(s) => {
            audit_expr(&s.tag, map);
            for expr in &s.exprs {
                audit_expr(&expr, map);
            }
        },
        ast::Expr::Arrow(s) => {
            for p in &s.params {
                audit_pat(&p, map)
            }
            match &s.body {
                ast::BlockStmtOrExpr::BlockStmt(s) => audit_block(&s, map),
                ast::BlockStmtOrExpr::Expr(s) => audit_expr(&s, map),
            }
        },
        ast::Expr::Class(s) => {
            audit_class(&s.class, map);
        },
        ast::Expr::Yield(s) => {
            if let Some(i) = &s.arg {
                audit_expr(&i, map);
            }
        },
        ast::Expr::MetaProp(_) => (),
        ast::Expr::Await(s) => audit_expr(&s.arg, map),
        ast::Expr::Paren(s) => audit_expr(&s.expr, map),
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
        ast::Expr::TsTypeAssertion(s) => audit_expr(&s.expr, map),
        ast::Expr::TsConstAssertion(s) => audit_expr(&s.expr, map),
        ast::Expr::TsNonNull(s) => audit_expr(&s.expr, map),
        ast::Expr::TsTypeCast(s) => audit_expr(&s.expr, map),
        ast::Expr::TsAs(s) => audit_expr(&s.expr, map),
        ast::Expr::PrivateName(_) => (),
        ast::Expr::OptChain(s) => audit_expr(&s.expr, map),
        ast::Expr::Invalid(_) => ()
    }

}

pub fn audit_prop(prop: &ast::Prop, map: Option<&CallbackMap>) {
    match prop {
        ast::Prop::Shorthand(_) => (),
        ast::Prop::KeyValue(s) => {
            audit_expr(&s.value, map);
            match &s.key {
                ast::PropName::Computed(s) => audit_expr(&s.expr, map),
                _ => ()
            }
        },
        ast::Prop::Assign(s) => audit_expr(&s.value, map),
        ast::Prop::Getter(s) => {
            match &s.key {
                ast::PropName::Computed(s) => audit_expr(&s.expr, map),
                _ => ()
            }
            if let Some(body) = &s.body {
                audit_block(&body, map);
            }
        },
        ast::Prop::Setter(s) => {
            audit_pat(&s.param, map);
            match &s.key {
                ast::PropName::Computed(s) => audit_expr(&s.expr, map),
                _ => ()
            }
            if let Some(body) = &s.body {
                audit_block(&body, map);
            }

        },
        ast::Prop::Method(s) => {
            match &s.key {
                ast::PropName::Computed(s) => audit_expr(&s.expr, map),
                _ => ()
            }
            audit_fn(&s.function, map);
        },
    }

}

pub fn audit_module_decl(_decl: &ast::ModuleDecl) {
    ()
}


pub fn record_span(span: &Span) {
    println!("{:?}", span);
}


pub fn audit_call(call: &ast::CallExpr, map: Option<&CallbackMap>) {
    match &call.callee {
        ast::ExprOrSuper::Expr(s) => audit_expr(&s, map),
        ast::ExprOrSuper::Super(_) => ()
    }
    for i in &call.args {
        audit_expr(&i.expr, map)
    }
    if let Some(m) = map {
        if let Some(f) = m.get("Call") {
            f(&Syntax::Expr(ast::Expr::Call(call.clone())));
        }
    }
}
