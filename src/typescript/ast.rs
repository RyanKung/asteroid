use swc::ecmascript::ast;
use swc::common::Span;

pub fn audit_script(script: &ast::Script) {
    audit_stmts(&script.body);
}


pub fn audit_stmts(stmts: &Vec<ast::Stmt>) {
    for stmt in stmts {
        audit_stmt(&stmt);
    }
}

pub fn audit_stmt(stmt: &ast::Stmt) {
    match stmt {
        ast::Stmt::Block(s) => audit_block(s),
        ast::Stmt::With(s) => {
            audit_expr(&s.obj);
            audit_stmt(&s.body);
        },
        ast::Stmt::Return(s) => {
            if let Some(expr) = &s.arg {
                audit_expr(expr);
            }
        },
        ast::Stmt::Labeled(s) => {
            audit_stmt(&s.body);
        },
        ast::Stmt::Break(_) => {
        },
        ast::Stmt::Continue(_) => {
        },
        ast::Stmt::If(s) => {
            audit_expr(&s.test);
            audit_stmt(&s.cons);
            if let Some(stmt) = &s.alt {
                audit_stmt(stmt);
            }

        },
        ast::Stmt::Switch(s) => {
            audit_expr(&s.discriminant);
            for i in &s.cases {
                if let Some(expr) = &i.test {
                    audit_expr(&expr);
                }
            }
        },
        ast::Stmt::Throw(s) => {
            audit_expr(&s.arg);
        },
        ast::Stmt::Try(s) => {
            audit_block(&s.block);
            if let Some(h) = &s.handler {
                if let Some(p) = &h.param {
                    audit_pat(&p);
                };
                audit_block(&h.body);
            }
            if let Some(f) = &s.finalizer {
                audit_block(&f)
            }
        },
        ast::Stmt::While(s) => {
            audit_expr(&s.test);
            audit_stmt(&s.body);
        },
        ast::Stmt::DoWhile(s) => {
            audit_expr(&s.test);
            audit_stmt(&s.body);
        },
        ast::Stmt::For(s) => {
            if let Some(init) = &s.init {
                match init {
                    ast::VarDeclOrExpr::VarDecl(e) => {
                        audit_var_decl(&e);
                    },
                    ast::VarDeclOrExpr::Expr(e) => {
                        audit_expr(&e);
                    }
                }
            }
            if let Some(test) = &s.test {
                audit_expr(test);
            }
            if let Some(update) = &s.update {
                audit_expr(update);
            }
            audit_stmt(&s.body);
        },
        ast::Stmt::ForIn(s) => {
            match &s.left {
                ast::VarDeclOrPat::VarDecl(e) => {
                    audit_var_decl(&e);
                },
                ast::VarDeclOrPat::Pat(e) => {
                    audit_pat(&e);
                }
            }
            audit_expr(&s.right);
            audit_stmt(&s.body);
        },
        ast::Stmt::ForOf(s) => {
            match &s.left {
                ast::VarDeclOrPat::VarDecl(e) => {
                    audit_var_decl(&e);
                },
                ast::VarDeclOrPat::Pat(e) => {
                    audit_pat(&e);
                }
            }
            audit_expr(&s.right);
            audit_stmt(&s.body);

        },
        ast::Stmt::Decl(s) => audit_decl(&s),
        ast::Stmt::Expr(s) => {
            audit_expr(&s.expr);
        },
        _ => ()
    }
}


fn audit_block(stmt: &ast::BlockStmt) {
    audit_stmts(&stmt.stmts);
}

fn audit_pat(pat: &ast::Pat) {
    match pat {
        ast::Pat::Ident(_) => (),
        ast::Pat::Array(s) => {
            for e in &s.elems {
                if let Some(p) = e {
                    audit_pat(&p);
                }
            }
        },
        ast::Pat::Rest(s) => {
            audit_pat(&s.arg)
        },
        ast::Pat::Object(s) => {
            for o in &s.props {
                audit_obj_pat_prop(&o);
            }
        },
        ast::Pat::Assign(s) => {
            audit_pat(&s.left);
            audit_expr(&s.right);
        },
        ast::Pat::Expr(s) => {
            audit_expr(&s);
        },
        _ => ()
    }

}


fn audit_var_decl(decl: &ast::VarDecl) {
    for d in &decl.decls {
        if let Some(expr) = &d.init {
            audit_expr(expr)
        }
    }
}

fn audit_decl(decl: &ast::Decl) {
    match decl {
        ast::Decl::Class(s) => {
            audit_class(&s.class);
        },
        ast::Decl::Fn(s) => {
            audit_fn(&s.function);
        },
        ast::Decl::Var(s) => {
            for v in &s.decls {
                if let Some(expr) = &v.init {
                    audit_expr(&expr)
                }
            }
        },
        ast::Decl::TsInterface(s) => {
            for e in &s.body.body {
                audit_ts_type_ele(&e);
            }
        },
        ast::Decl::TsTypeAlias(_) => (),
        ast::Decl::TsEnum(s) => {
            for m in &s.members {
                if let Some(expr) = &m.init {
                    audit_expr(&expr);
                }
            }
        },
        ast::Decl::TsModule(s) => {
            if let Some(ns) = &s.body {
                audit_ts_namespace(&ns);
            }
        }
    }
}

fn audit_obj_pat_prop(prop: &ast::ObjectPatProp) {
    match prop {
        ast::ObjectPatProp::KeyValue(s) => {
            audit_pat(&s.value);
        },
        ast::ObjectPatProp::Assign(s) => {
            if let Some(v) = &s.value {
                audit_expr(&v);
            }
        },
        ast::ObjectPatProp::Rest(s) => {
            audit_pat(&s.arg);
        },
    }
}


fn audit_class(cls: &ast::Class) {
    for d in &cls.decorators {
        audit_expr(&d.expr);
    }
    for d in &cls.body {
        audit_class_member(&d);
    }
    if let Some(expr) = &cls.super_class {
        audit_expr(&expr);
    }
}

fn audit_class_member(cls_m: &ast::ClassMember) {
    match cls_m {
        ast::ClassMember::Constructor(s) => {
            audit_prop_name(&s.key);
            for p in &s.params {
                audit_param_prop(&p);
            }
            if let Some(body) = &s.body {
                audit_block(&body);
            }

        },
        ast::ClassMember::Method(s) => {
            audit_prop_name(&s.key);
            audit_fn(&s.function);
        },
        ast::ClassMember::PrivateMethod(s) => {
            audit_fn(&s.function);
        },
        ast::ClassMember::ClassProp(s) => {
            audit_expr(&s.key);
            if let Some(expr) = &s.value {
                audit_expr(&expr);
            }
            for d in &s.decorators {
                audit_expr(&d.expr);
            }
        },
        ast::ClassMember::PrivateProp(s) => {
            if let Some(expr) = &s.value {
                audit_expr(&expr);
            }
            for d in &s.decorators {
                audit_expr(&d.expr);
            }

        },
        ast::ClassMember::TsIndexSignature(s) => {
            for p in &s.params {
                audit_ts_fn_param(&p);
            }
        }
    }
}

fn audit_ts_fn_param(param: &ast::TsFnParam) {
    match param {
        ast::TsFnParam::Ident(s) => {
            audit_pat(&ast::Pat::from(s.clone()));
        },
        ast::TsFnParam::Array(s) => {
            audit_pat(&ast::Pat::from(s.clone()));
        },
        ast::TsFnParam::Rest(s) => {
             audit_pat(&ast::Pat::from(s.clone()));

        },
        ast::TsFnParam::Object(s) => {
            audit_pat(&ast::Pat::from(s.clone()));
        },
    }

}

fn audit_prop_name(name: &ast::PropName) {
    match name {
        ast::PropName::Computed(e) => {
            audit_expr(&e.expr);
        },
        _ => ()
    }
}


fn audit_param_prop(prop: &ast::ParamOrTsParamProp) {
    match prop {
        ast::ParamOrTsParamProp::TsParamProp(s) => {
            for d in &s.decorators {
                audit_expr(&d.expr);
            }
            match &s.param {
                ast::TsParamPropParam::Ident(_) => (),
                ast::TsParamPropParam::Assign(p) => {
                    audit_pat(&ast::Pat::from(p.clone()));
                },
            }
        },
        ast::ParamOrTsParamProp::Param(s) => audit_param(&s)
    }
}


fn audit_param(param: &ast::Param) {
    for d in &param.decorators {
        audit_expr(&d.expr);
    }
    audit_pat(&param.pat);
}

fn audit_fn(func: &ast::Function) {
    for p in &func.params {
        audit_param(&p);
    }
    for d in &func.decorators {
        audit_expr(&d.expr);
    }
    if let Some(stmt) = &func.body {
        audit_block(&stmt);
    }
}

fn audit_ts_type_ele(ele: &ast::TsTypeElement) {
    match &ele {
        ast::TsTypeElement::TsCallSignatureDecl(s) => {
            for p in &s.params {
                audit_ts_fn_param(&p);
            }
        },
        ast::TsTypeElement::TsConstructSignatureDecl(s) => {
            for p in &s.params {
                audit_ts_fn_param(&p);
            }
        },
        ast::TsTypeElement::TsPropertySignature(s) => {
            audit_expr(&s.key);
            if let Some(expr) = &s.init {
                audit_expr(&expr)
            }
            for p in &s.params {
                audit_ts_fn_param(&p);
            }
        },
        ast::TsTypeElement::TsMethodSignature(s) => {
            audit_expr(&s.key);
            for p in &s.params {
                audit_ts_fn_param(&p);
            }
        },
        ast::TsTypeElement::TsIndexSignature(s) => {
            for p in &s.params {
                audit_ts_fn_param(&p);
            }

        },
    }
}

fn audit_ts_namespace(ns: &ast::TsNamespaceBody) {
    match &ns {
        ast::TsNamespaceBody::TsModuleBlock(s) => {
            for m in &s.body {
                match &m {
                    ast::ModuleItem::ModuleDecl(s) => audit_module_decl(&s),
                    ast::ModuleItem::Stmt(s) => audit_stmt(&s)
                }
            }
        },
        ast::TsNamespaceBody::TsNamespaceDecl(s) => {
            audit_ts_namespace(&s.body);
        }
    }
}

pub fn audit_expr(expr: &ast::Expr) {
    match &expr {
        ast::Expr::This(_) => (),
        ast::Expr::Array(s) => {
            for e in &s.elems {
                if let Some(eos) = &e {
                    audit_expr(&eos.expr);
                }
            }
        },
        ast::Expr::Object(s) => {
            for e in &s.props {
                match e {
                    ast::PropOrSpread::Spread(s) => audit_expr(&s.expr),
                    ast::PropOrSpread::Prop(s) => audit_prop(&s)
                }
             }
        },
        ast::Expr::Fn(s) => audit_fn(&s.function),
        ast::Expr::Unary(s) => audit_expr(&s.arg),
        ast::Expr::Update(s) => audit_expr(&s.arg),
        ast::Expr::Bin(s) => {
            audit_expr(&s.left);
            audit_expr(&s.right);
        },
        ast::Expr::Assign(s) => {
            match &s.left {
                ast::PatOrExpr::Expr(s) => audit_expr(&s),
                ast::PatOrExpr::Pat(s) => audit_pat(&s)
            }
            audit_expr(&s.right);

        },
        ast::Expr::Member(s) => {
            match &s.obj {
                ast::ExprOrSuper::Expr(s) => audit_expr(&s),
                ast::ExprOrSuper::Super(_) => ()
            }
        },
        ast::Expr::Cond(s) => {
            audit_expr(&s.test);
            audit_expr(&s.cons);
            audit_expr(&s.alt);
        },
        ast::Expr::Call(s) => {
            println!("Call: {:?}", s);
            match &s.callee {
                ast::ExprOrSuper::Expr(s) => audit_expr(&s),
                ast::ExprOrSuper::Super(_) => ()
            }
            for i in &s.args {
                audit_expr(&i.expr)
            }
        },
        ast::Expr::New(s) => {
            audit_expr(&s.callee);
            if let Some(args) = &s.args {
                for a in args {
                    audit_expr(&a.expr);
                }
            }
        },
        ast::Expr::Seq(s) => {
            for expr in &s.exprs {
                audit_expr(&expr);
            }
        },
        ast::Expr::Ident(_) => (),
        ast::Expr::Lit(_) => (),
        ast::Expr::Tpl(s) => {
            for expr in &s.exprs {
                audit_expr(&expr);
            }
        },
        ast::Expr::TaggedTpl(s) => {
            audit_expr(&s.tag);
            for expr in &s.exprs {
                audit_expr(&expr);
            }
        },
        ast::Expr::Arrow(s) => {
            for p in &s.params {
                audit_pat(&p)
            }
            match &s.body {
                ast::BlockStmtOrExpr::BlockStmt(s) => audit_block(&s),
                ast::BlockStmtOrExpr::Expr(s) => audit_expr(&s),
            }
        },
        ast::Expr::Class(s) => {
            audit_class(&s.class);
        },
        ast::Expr::Yield(s) => {
            if let Some(i) = &s.arg {
                audit_expr(&i);
            }
        },
        ast::Expr::MetaProp(_) => (),
        ast::Expr::Await(s) => audit_expr(&s.arg),
        ast::Expr::Paren(s) => audit_expr(&s.expr),
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
        ast::Expr::TsTypeAssertion(s) => audit_expr(&s.expr),
        ast::Expr::TsConstAssertion(s) => audit_expr(&s.expr),
        ast::Expr::TsNonNull(s) => audit_expr(&s.expr),
        ast::Expr::TsTypeCast(s) => audit_expr(&s.expr),
        ast::Expr::TsAs(s) => audit_expr(&s.expr),
        ast::Expr::PrivateName(_) => (),
        ast::Expr::OptChain(s) => audit_expr(&s.expr),
        ast::Expr::Invalid(_) => ()
    }

}

pub fn audit_prop(prop: &ast::Prop) {
    match prop {
        ast::Prop::Shorthand(_) => (),
        ast::Prop::KeyValue(s) => {
            audit_expr(&s.value);
            match &s.key {
                ast::PropName::Computed(s) => audit_expr(&s.expr),
                _ => ()
            }
        },
        ast::Prop::Assign(s) => audit_expr(&s.value),
        ast::Prop::Getter(s) => {
            match &s.key {
                ast::PropName::Computed(s) => audit_expr(&s.expr),
                _ => ()
            }
            if let Some(body) = &s.body {
                audit_block(&body);
            }
        },
        ast::Prop::Setter(s) => {
            audit_pat(&s.param);
            match &s.key {
                ast::PropName::Computed(s) => audit_expr(&s.expr),
                _ => ()
            }
            if let Some(body) = &s.body {
                audit_block(&body);
            }

        },
        ast::Prop::Method(s) => {
            match &s.key {
                ast::PropName::Computed(s) => audit_expr(&s.expr),
                _ => ()
            }
            audit_fn(&s.function);
        },
    }

}

pub fn audit_module_decl(_decl: &ast::ModuleDecl) {
    ()
}


pub fn record_span(span: &Span) {
    println!("{:?}", span);
}
